
use steamworks as sw;

fn main() {
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

    let message = "tal".to_string();

    // Convertimos la cadena en bytes
    let message_bytes = message.as_bytes();

    // El tipo de mensaje puede ser Reliable o Unreliable, dependiendo de lo que necesites
    let send_type = sw::SendType::Reliable; // O `Unreliable` si no necesitas garantía de entrega

    // Enviar el mensaje P2P a nuestro propio steam_id
    let my_steam_id = user.steam_id(); // Tu propio steam_id (o el de otro usuario)

    client.networking().send_p2p_packet(
        my_steam_id,   // Destinatario
        send_type,     // Tipo de mensaje (Reliable o Unreliable)
        message_bytes, // El mensaje en bytes
    );

    // Registramos un callback para recibir mensajes
    client.register_callback(|e: sw::NetworkingMessageReceived| {
        if e.message_type == send_type {
            // Convertimos los bytes del mensaje de vuelta a String
            let received_message = String::from_utf8_lossy(&e.message);
            println!("Mensaje recibido: {}", received_message);
        }
    });

    // Mantenemos el cliente activo para recibir los mensajes
    single.run_callbacks();

    println!("{:?}", online_friend_names);
}
