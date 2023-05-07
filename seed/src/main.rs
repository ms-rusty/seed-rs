use seed_game_world::GameWorld;
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
    let game_world = GameWorld::new();

    let network_server = NetworkServer::new().await;

    std::thread::spawn(move || {
        game_world.run();
    });

    network_server.run().await;
}
