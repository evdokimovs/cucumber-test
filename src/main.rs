mod entity;
mod file_server;
mod utils;
mod world;

use cucumber_rust::{given, then, WorldInit as _};

use self::{file_server::FileServer, world::BrowserWorld};

#[given(regex = "Room with ID '(.*)'")]
async fn given_room_with_id(world: &mut BrowserWorld, id: String) {
    world.create_room(&id).await;
}

#[then(regex = "Room with ID '(.*)' should exist in the BrowserWorld")]
async fn then_room_should_exist(world: &mut BrowserWorld, id: String) {
    let room = world.get_room(&id).unwrap();
    let js_id = room.get_id().await;
    assert_eq!(id, js_id);
}

#[tokio::main]
async fn main() {
    let _server = FileServer::run();
    let runner = BrowserWorld::init(&["./features"]);
    runner.run_and_exit().await;
}
