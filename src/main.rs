use bevy::prelude::*;
use bevy_steamworks::*;
use std::time::Duration;
use steamworks::{
    self as sw,
    networking_types::{NetworkingIdentity, SendFlags},
};

fn steam_system(steam_client: Res<Client>) {
    let (client, single) = sw::Client::init().unwrap();

    let user = client.user();
    let friends = client.friends();
    let me = friends.get_friend(user.steam_id());

    // run_callbacks must be called regularly, or no incoming connections can be received
    let callback_loop = std::thread::spawn(move || loop {
        single.run_callbacks();
        std::thread::sleep(Duration::from_millis(10));
    });
    let networking_messages = client.networking_messages();

    // Accept all new connections
    networking_messages.session_request_callback(|request| request.accept());

    let _received = networking_messages.receive_messages_on_channel(0, 10);
}

fn client_system() {
    let (client, _single) = sw::Client::init().unwrap();

    let user = client.user();
    let friends = client.friends();
    let _me = friends.get_friend(user.steam_id());

    let networking_messages = client.networking_messages();

    let identity = NetworkingIdentity::new_steam_id(user.steam_id());

    match networking_messages.send_message_to_user(
        identity,
        SendFlags::RELIABLE,
        "hello".as_bytes(),
        1,
    ) {
        Ok(val) => println!("{:?}", val),
        _ => (),
    }
}

fn main() {
    // Use the demo Steam AppId for SpaceWar
    App::new()
        // it is important to add the plugin before `RenderPlugin` that comes with `DefaultPlugins`
        .add_plugins(SteamworksPlugin::init_app(480).unwrap())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (steam_system, client_system).chain())
        .run();
}
