# Benchmarking tool for RPC Servers
Project structure: 
- benchmarker: RPC Client on Typescript. In charge of setting up multiple clients and making requests
- rs-server: RPC Server on Rust. The rs-server will clone the rpc-rust repo because it's still not published as a package.
- ts-server: RPC Server on Node. `tsc --init` project with minimum needed stuff.
- api.proto: Where the protocol and service are defined, shared by the benchmarker, rs-server and ts-server. 
- book-large-content.txt: Large file that both servers use to store a book into an in-memory database which contains this large text in its `content` field.

## How to use it?
Make sure you have `node`, `rust` and `cargo` installed. 

### Prepare environment 

To have all ready to make benchmarks, you must run:

```shell
make build-ts
```

```shell
make build-rs
```

After both commands finished you are ready.

### Run the tool

```shell
make ab server={SERVER_TO_RUN} n={TOTAL_REQUEST_TO_DO} c={CONCURRENCY_LEVEL_FOR_REQUESTS} 
```

- `server`: Value could be `rs` for Rust and `ts` for Typescript (node). 
- `n` : Refers to total requests to be done by the `benchmarker`. eg: 10000 (used for the presentation benchmarking)
- `c` : Refers to the concurrency level of the tool. eg: 100 (used for the presentation benchmarking)

After running the tool, a `.csv` file will be created with a the type of the metric measure and the elapsed time.
`${type},${elapsed_time}`, there are two different types, one `WHOLECONNECTION` refers to the entire client connection and other one `GETLARGEBOOKREQUEST` that refers to just the request that fetches the book with a `content` field with 1MB. 

The tool also throws an output with some stats similar to `Apache Benchmarking` -> mean, min, max, and percentiles (percentiles are just for the GetBook request). This output shows:
- `Handle Client time` metric: refers to the elapsed time for the entire client connection
- `Rpc Client` metric: refers to the elapsed time for the client creation and the connection is established with server.
- `Rpc Port` metric: refers to the elapsed time for the port creation, that it's also a request.
- `Request time` metric: refers to the elapsed time for the `GetBook` request (the same as `GETLARGEBOOKREQUEST`)
- `percentiles` metric: refers to the elapsed time percentiles for the `GetBook` request

Output example
```shell
Total test duration: {NUMBER}
Request / Second: {NUMBER}
Handle Client time (mean): {NUMBER}
Rpc Client (mean):  {NUMBER}
Rpc Port (mean):  {NUMBER}
Request time (mean):  {NUMBER}
Handle Client time (min):  {NUMBER}
Rpc Client (min):  {NUMBER}
Rpc Port (min):  {NUMBER}
Request time (min):  {NUMBER}
Handle Client time (max):  {NUMBER}
Rpc Client (max):  {NUMBER}
Rpc Port (max):  {NUMBER}
Request time (max):  {NUMBER}
Percentiles:  { '50': {NUMBER}, '75': {NUMBER}, '85': {NUMBER}, '95': {NUMBER}, '98': {NUMBER}, '100': {NUMBER} }
```

## TODO: 
- [X] Adds a benchmarker-rs with the `RpcClient` written in Rust
- [ ] Adds C# implementation for benchmarking