use std::sync::Arc;
use steamworks as sw;

pub fn start_client(host_id: sw::SteamId) {
    let (client, single) = sw::Client::init().expect("No se pudo inicializar Steamworks");
    let networking = client.networking();

    // Enviar mensaje al host
    let message = b"Hola desde el cliente!";
    networking
        .send_p2p_packet(host_id, message, sw::P2PSend::Reliable)
        .unwrap();
    println!("Cliente envió mensaje al host con ID: {:?}", host_id);

    // Callback para recibir mensajes del host
    let _cb = client.register_callback(move |msg: sw::NetworkingMessage| {
        let data = msg.data();
        println!(
            "Cliente recibió mensaje del host: {:?}",
            String::from_utf8_lossy(data)
        );
    });

    // Bucle para mantener el cliente activo y recibir mensajes
    loop {
        single.run_callbacks(); // Ejecuta callbacks para procesar mensajes entrantes
    }
}
