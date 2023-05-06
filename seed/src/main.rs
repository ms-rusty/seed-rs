use seed_game_server::GameServer;
use seed_network_server::NetworkServer;
use tokio::runtime::Builder;

fn main() {
    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Error on build the Tokio Runtime.");

    runtime.block_on(async {
        async_main().await;
    });
}

async fn async_main() {
    let game_server = GameServer::new();
    let network_server = NetworkServer::new().await;

    tokio::task::spawn(async move {
        network_server.run().await;
    });

    game_server.run().await;
}
