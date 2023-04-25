use benchmarker_rs::{BookServiceClient, GetBookRequest, RPCServiceClient};
use dcl_rpc::client::RpcClient;
use dcl_rpc::transports::web_socket::{WebSocketClient, WebSocketTransport};
use std::time::Duration;
use tokio::time::{sleep, Instant};

fn mean(numbers: &Vec<u128>) -> u128 {
    let sum: u128 = numbers.iter().sum();

    sum / numbers.len() as u128
}

#[tokio::main]
async fn main() {
    let test_elapsed_time = Instant::now();
    let mut set = tokio::task::JoinSet::new();

    let concurrency = 100;
    let mut whole_conns = vec![];
    let mut client_conns = vec![];
    let mut port_creations = vec![];
    let mut reqs = vec![];

    let mut current = 0;
    for i in 0..10000 {
        current += 1;
        if current == concurrency {
            while let Some(res) = set.join_next().await {
                let (whole_conn, client_conn, port, request) = res.unwrap();
                whole_conns.push(whole_conn);
                client_conns.push(client_conn);
                port_creations.push(port);
                reqs.push(request);
            }
            println!("Completed Requests: {}", i + 1);
            sleep(Duration::from_millis(250)).await;
            current = 0;
        }
        set.spawn(handle_client());
    }
    let test_elapsed_time = test_elapsed_time.elapsed().as_secs();

    let mean_whole = mean(&whole_conns);
    let mean_client_conns = mean(&client_conns);
    let mean_ports = mean(&port_creations);
    let mean_reqs = mean(&reqs);

    println!("Test duration: {}", test_elapsed_time);
    println!("Entire Connection (mean) {mean_whole}");
    println!("Client Connetion (mean) {mean_client_conns}");
    println!("Port Creation (mean) {mean_ports}");
    println!("Request (mean) {mean_reqs}");
}

async fn handle_client() -> (u128, u128, u128, u128) {
    let whole_connection = Instant::now();
    let ws = WebSocketClient::connect("ws://0.0.0.0:8080").await.unwrap();
    let transport = WebSocketTransport::new(ws);

    let client_connection = Instant::now();
    let mut client = RpcClient::new(transport).await.unwrap();
    let client_creation_elapsed = client_connection.elapsed().as_millis();

    let port_creation = Instant::now();
    let client_port = client.create_port("my-port").await.unwrap();
    let port_creation_elapsed = port_creation.elapsed().as_millis();

    let service = client_port
        .load_module::<BookServiceClient<WebSocketTransport>>("BookService")
        .await
        .unwrap();

    let request = Instant::now();
    service.get_book(GetBookRequest { isbn: 1008 }).await;
    let req_elapsed = request.elapsed().as_millis();

    let whole_connection = whole_connection.elapsed().as_millis();

    (
        whole_connection,
        client_creation_elapsed,
        port_creation_elapsed,
        req_elapsed,
    )
}
