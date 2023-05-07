use seed_game_world::GameWorld;
use seed_network_server::NetworkServer;

fn main() {
    let game_world = GameWorld::new();

    let network_server = NetworkServer::new();

    std::thread::spawn(move || {
        game_world.run();
    });

    network_server.run();
}
