use std::sync::Arc;
use steamworks as sw;

pub fn start_host() {
    let (client, single) = sw::Client::init().expect("No se pudo inicializar Steamworks");
    let networking = client.networking();
    let _cb = client.register_callback(move |msg: sw::NetworkingMessage| {
        let sender_id = msg.sender();
        let data = msg.data();

        // Procesa el mensaje recibido
        println!(
            "Host recibió mensaje de {}: {:?}",
            sender_id,
            String::from_utf8_lossy(data)
        );

        // Ejemplo de respuesta al cliente
        let reply = b"Mensaje recibido por el host!";
        networking
            .send_p2p_packet(sender_id, reply, sw::P2PSend::Reliable)
            .unwrap();
    });

    println!("Host esperando conexiones...");

    // Bucle para mantener el host activo
    loop {
        single.run_callbacks(); // Ejecuta callbacks para procesar mensajes entrantes
    }
}
