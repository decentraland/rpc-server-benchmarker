import { createRpcClient } from "@dcl/rpc"
import { IWebSocket, WebSocketTransport } from "@dcl/rpc/dist/transports/WebSocket"
import { loadService } from "@dcl/rpc/dist/codegen"
import { WebSocket } from 'ws';
import { Book, BookServiceDefinition, GetBookRequest } from "./api";
import { startEvent, endEvent } from 'elapsed-time-util';
import { arg, max, mean, min } from "mathjs";
import percentile from "percentile";
import fs from 'fs';
import minimist from "minimist";

const args = minimist(process.argv.slice(2))

const numberOfRequests = args.n;
const concurrency = args.c;
const result_file = args.r

if (!numberOfRequests || !concurrency || !result_file) {
    throw new Error("Missing args. Check if you correcly passed -n, -c, and -r");
}

function sleep(){
    return new Promise((resolve) => {
        setTimeout(() => {
            resolve(true)
        }, 250)
    })
}

class ElapsedTimeRecord {
    constructor(public type: string, public time: number) {
    }

    save() {
        const record = `${this.type},${this.time} \n`;
        fs.appendFile(`${result_file}.csv`, record, (err) => {
            if (err) {
                throw new Error(err.message);
            }
        })
    }

}

async function main() {
    const start = startEvent("start");
    let clients = [];
    for (let i = 0; i < numberOfRequests; i++) {
        clients.push(handleClient(i))
        if (clients.length == concurrency) {
            await Promise.all(clients)
            await sleep()
            clients = []
            console.log(`Completed ${i+1} requests`);
        }
    }
    console.log("Terminated")
    const end = endEvent("start");
    return end
}

const handlelClientTimes: number[] = [];
const rpcClientTimes: number[] = [];
const rpcPortTimes: number[] = [];
const bookRequestsTimes: number[] = [];

async function handleClient(numConnection: number) {
    startEvent(`Connection ${numConnection}`);
    let ws = new WebSocket('ws://0.0.0.0:8080');
    const clientSocket = WebSocketTransport(ws as IWebSocket)
    startEvent(`RpcClient ${numConnection}`);
    const client = await createRpcClient(clientSocket)
    const rpcClient = endEvent(`RpcClient ${numConnection}`);
    startEvent(`RpcClientPort ${numConnection}`);
    const clientPort = await client.createPort("my-port")
    const rpcPort = endEvent(`RpcClientPort ${numConnection}`);
    const clientBookService = loadService(clientPort, BookServiceDefinition)
    startEvent(`Request ${numConnection}`);
    await clientBookService.getBook({ isbn: 1008 });
    const request = endEvent(`Request ${numConnection}`);
    ws.close();
    const elapsed = endEvent(`Connection ${numConnection}`);

    handlelClientTimes.push(elapsed)
    bookRequestsTimes.push(request)
    rpcClientTimes.push(rpcClient)
    rpcPortTimes.push(rpcPort)
    new ElapsedTimeRecord("GETLARGEBOOKREQUEST", request).save();
    new ElapsedTimeRecord("WHOLECONNECTION", elapsed).save();
}

main()
.then((totalElapsedTime) => {
    const cleanTotal = totalElapsedTime / 1000;
    console.log("Total test duration: ", cleanTotal);

    const reqPerSecondInSeconds = numberOfRequests / cleanTotal;
    console.log("Request / Second: ", reqPerSecondInSeconds)

    console.log("Handle Client time (mean): ", mean(handlelClientTimes));
    console.log("Rpc Client (mean): ", mean(rpcClientTimes));
    console.log("Rpc Port (mean): ", mean(rpcPortTimes));
    console.log("Request time (mean): ", mean(bookRequestsTimes));

    console.log("Handle Client time (min): ", min(handlelClientTimes));
    console.log("Rpc Client (min): ", min(rpcClientTimes));
    console.log("Rpc Port (min): ", min(rpcPortTimes));
    console.log("Request time (min): ", min(bookRequestsTimes));

    console.log("Handle Client time (max): ", max(handlelClientTimes));
    console.log("Rpc Client (max): ", max(rpcClientTimes));
    console.log("Rpc Port (max): ", max(rpcPortTimes));
    console.log("Request time (max): ", max(bookRequestsTimes));

    const percentiles = percentile([50, 75, 80, 95, 98, 100], bookRequestsTimes);
    //@ts-ignore
    console.log("Percentiles: ", { 50: percentiles[0], 75: percentiles[1], 85: percentiles[2], 95: percentiles[3], 98: percentiles[4], 100: percentiles[5]  })
})
.catch((err) => {
    console.log(err)
})