use std::sync::Arc;

use dcl_rpc::{
    server::{RpcServer, RpcServerPort},
    transports::web_socket::{WebSocketServer, WebSocketTransport},
};
use rs_server::{service::book_service, AppContext, Book, BookServiceRegistration};
use tokio::fs;

#[tokio::main]
async fn main() {
    let ws_server = WebSocketServer::new("0.0.0.0:8080");
    let mut on_connection_listener = ws_server.listen().await.unwrap();
    let ctx = AppContext {
        hardcoded_database: create_db().await,
    };
    let mut server = RpcServer::create(ctx);
    server.set_handler(|port: &mut RpcServerPort<AppContext>| {
        BookServiceRegistration::register_service(port, book_service::BookService {})
    });
    let server_events_sender = server.get_server_events_sender();
    tokio::spawn(async move {
        while let Some(Ok(socket)) = on_connection_listener.recv().await {
            server_events_sender
                .send_attach_transport(Arc::new(WebSocketTransport::new(socket)))
                .unwrap();
        }
    });
    server.run().await;
}

async fn create_db() -> Vec<Book> {
    let text = fs::read_to_string("../book-large-content.txt")
        .await
        .expect("Unable to read file");
    vec![
        Book {
            author: "mr steve".to_string(),
            title: "Rust: crash course".to_string(),
            isbn: 1000,
            content: "".to_string(),
        },
        Book {
            author: "mr jobs".to_string(),
            title: "Rust: how do futures work under the hood?".to_string(),
            isbn: 1001,
            content: "".to_string(),
        },
        Book {
            author: "mr robot".to_string(),
            title: "Create a robot from scrath".to_string(),
            isbn: 1002,
            content: "".to_string(),
        },
        Book {
            author: "vitalik".to_string(),
            title: "Blockchain 101".to_string(),
            isbn: 1003,
            content: "".to_string(),
        },
        Book {
            author: "buterin".to_string(),
            title: "Smart Contracts 101".to_string(),
            isbn: 1004,
            content: "".to_string(),
        },
        Book {
            author: "mr mendez".to_string(),
            title: "Intro to DCL SDK 7".to_string(),
            isbn: 1006,
            content: "".to_string(),
        },
        Book {
            author: "mr kuruk".to_string(),
            title: "Advanced AI".to_string(),
            isbn: 1006,
            content: "".to_string(),
        },
        Book {
            author: "mr cazala".to_string(),
            title: "Neural Networks".to_string(),
            isbn: 1007,
            content: "".to_string(),
        },
        Book {
            author: "mr mannakia".to_string(),
            title: "Ethereum from Scratch".to_string(),
            isbn: 1008,
            content: text,
        },
        Book {
            author: "buterin".to_string(),
            title: "Lightining Network".to_string(),
            isbn: 1009,
            content: "".to_string(),
        },
        Book {
            author: "buterin".to_string(),
            title: "ZK Proof".to_string(),
            isbn: 1010,
            content: "".to_string(),
        },
        Book {
            author: "buterin".to_string(),
            title: "Solidty".to_string(),
            isbn: 1011,
            content: "".to_string(),
        },
        Book {
            author: "buterin".to_string(),
            title: "ERC-20".to_string(),
            isbn: 1012,
            content: "".to_string(),
        },
    ]
}
