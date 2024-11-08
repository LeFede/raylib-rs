use steamworks as sw;

pub fn steam() -> (sw::Friend<sw::ClientManager>, Vec<String>) {
    let (client, single) = sw::Client::init().unwrap();
    let _cb = client.register_callback(|_p: sw::PersonaStateChange| {});
    let user = client.user();
    let friends = client.friends();
    let me = friends.get_friend(user.steam_id());
    let list = friends.get_friends(sw::FriendFlags::IMMEDIATE);

    // Filtramos los amigos en línea y extraemos sus nombres
    let online_friend_names: Vec<String> = list
        .iter()
        .filter(|f| f.state() == sw::FriendState::Online) // Filtrar amigos en línea
        .map(|f| f.name().to_string()) // Convertir el nombre de cada amigo a String
        .collect(); // Recopilar en un Vec<String>

    (me, online_friend_names)
}
