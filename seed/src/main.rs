use seed_network_server::NetworkServer;
use tokio::{net::TcpListener, runtime::Builder};

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        async_main().await;
    });
}

async fn async_main() {
    let listener = TcpListener::bind("127.0.0.1:65535")
        .await
        .expect("Error on bind TcpListener.");

    NetworkServer::new(listener).run().await;
}
