import { createRpcServer } from '@dcl/rpc';
import * as codegen from '@dcl/rpc/dist/codegen';
import { IWebSocket, WebSocketTransport } from '@dcl/rpc/dist/transports/WebSocket';
import { Book, BookServiceDefinition, GetBookRequest, QueryBooksRequest } from './api';
import { WebSocketServer } from 'ws';
import { readFileSync } from 'fs';

export type TestContext = { hardcodedDatabase: Book[] }

const text = readFileSync('../book-large-content.txt').toString();

const context: TestContext = {
    hardcodedDatabase: [
       {
        author: "mr steve",
        title: "Rust: crash course",
        isbn: 1000,
        content: ""
    },
     {
        author: "mr jobs",
        title: "Rust: how do futures work under the hood?",
        isbn: 1001,
        content: "",
    },
     {
        author: "mr robot",
        title: "Create a robot from scrath",
        isbn: 1002,
        content: "",

    },
     {
        author: "vitalik",
        title: "Blockchain 101",
        isbn: 1003,
        content: "",

    },
     {
        author: "buterin",
        title: "Smart Contracts 101",
        isbn: 1004,
        content: "",

    },
     {
        author: "mr mendez",
        title: "Intro to DCL SDK 7",
        isbn: 1006,
        content: "",

    },
     {
        author: "mr kuruk",
        title: "Advanced AI",
        isbn: 1006,
        content: "",

    },
     {
        author: "mr cazala",
        title: "Neural Networks",
        isbn: 1007,
        content: "",

    },
    {
      author: "mr cazala",
      title: "Neural Networks 2",
      isbn: 1008,
      content: text
    },
     {
        author: "buterin",
        title: "Lightining Network",
        isbn: 1009,
        content: "",

    },
     {
        author: "buterin",
        title: "ZK Proof",
        isbn: 1010,
        content: "",

    },
     {
        author: "buterin",
        title: "Solidty",
        isbn: 1011,
        content: "",

    },
     {
        author: "buterin",
        title: "ERC-20",
        isbn: 1012,
        content: "",

    },
    ],
}

const rpcServer = createRpcServer<TestContext>({})
rpcServer.setHandler(async function handler(port) {
  codegen.registerService(port, BookServiceDefinition, async () => ({
    async getBook(req: GetBookRequest, context) {
      return context.hardcodedDatabase.find((book) => book.isbn == req.isbn)!
    },
  }))
});

const ws_servr = new WebSocketServer({ port: 8080 });

ws_servr.on('connection', (conn) => {
  let transport = WebSocketTransport(conn as IWebSocket);
  rpcServer.attachTransport(transport, context)
})

ws_servr.on("error", (error) => {
  console.log("Error: ", error)
})