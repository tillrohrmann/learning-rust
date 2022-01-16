use std::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::Duration;
use tokio_stream::wrappers::TcpListenerStream;
use futures::stream::StreamExt;

fn main() {
    // let runtime = tokio::runtime::Runtime::new().unwrap();
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_io().enable_time().worker_threads(3).build().unwrap();
    let join_handle = runtime.spawn(async {
        // Listen for incoming TCP connections on localhost port 7878
        let listener = TcpListenerStream::new(TcpListener::bind("127.0.0.1:7878").await.unwrap());
        listener.for_each_concurrent(None, |tcp_stream| async move {
            let stream = tcp_stream.unwrap();
            tokio::spawn(handle_connection(stream));
        }).await;
    });


    runtime.block_on(join_handle).unwrap();
}

async fn handle_connection(mut stream: TcpStream) {
    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // tokio::time::sleep(Duration::from_secs(5)).await;
        println!("Sleeping now :-)");
        std::thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}