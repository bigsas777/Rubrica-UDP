use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Persona {
    nome: String,
    cognome: String
}

fn main() {
    // Spazio di ricezione del server
    let mut recv_data = [0; 1024];

    let mut rubrica: Vec<Persona> = Vec::new();

    // UDP socket del server
    let socket = UdpSocket::bind("127.0.0.1:4242").expect("Errore binding!!");

    loop {
        println!("inizio");
        // Riceve i dati inviati a questo indirizzo
        let (num_bytes, src_addr) = socket.recv_from(&mut recv_data).expect("Errore ricezione!!");
        // Trasforma i byte in una stringa JSON
        let command_data = String::from_utf8_lossy(&recv_data[..num_bytes]);
        println!("dati ricevuti: {}", command_data);
        let contatto_deserialized: Result<Persona, serde_json::Error> = serde_json::from_str(&command_data);

        if contatto_deserialized.as_ref().unwrap().nome == "list" {
            println!("list");
            let str_rubrica = serde_json::to_string(&rubrica).unwrap();
            let _ = socket.send_to(str_rubrica.as_bytes(), "127.0.0.1:34254").expect("Errore di invio!!!");
        }
        else { // Aggiunge il nuovo contatto
            println!("nuovo contatto");
            rubrica.push(contatto_deserialized.unwrap());
        }
    }
}
