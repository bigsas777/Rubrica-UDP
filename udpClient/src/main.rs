use std::{net::UdpSocket, io};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Persona {
    nome: String,
    cognome: String
}

fn main() {

    // Spazio di ricezione del client
    let mut recv_data = [0; 1024];

    // Socket UDP client
    let socket = UdpSocket::bind("127.0.0.1:34254").expect("Errore binding!!!!");

    loop {
        // Input comando
        let mut input_cmd = String::new();

        io::stdin()
            .read_line(&mut input_cmd)
            .expect("Errore nella lettura.");


        // Pulizia input
        input_cmd = input_cmd.trim().to_lowercase();


        // * Interpretazione comandi

        // Separa le parole del comando 
        let input_cmd: Vec<String> = input_cmd.split_whitespace().map(|s| s.to_string()).collect();

        // println!("{:?}", input_cmd);

        // Matching del comando ricevuto
        if input_cmd[0] == "insert" {
            
            let nome = get_param(&input_cmd, String::from("--name"));
            let cognome = get_param(&input_cmd, String::from("--surname"));
            // let server = get_param(&input_cmd, String::from("--server"));
            
            let contatto = Persona {
                nome: nome,
                cognome: cognome,
            };

            let contatto_serialized = serde_json::to_string(&contatto);
            // println!("{}", contatto_serialized.as_ref().unwrap());

            let _ = socket.send_to(contatto_serialized.unwrap().as_bytes(), "127.0.0.1:4242").expect("Errore di invio!!!");
        }
        else if input_cmd[0] == "list" {
            let fake_contatto = Persona {
                nome: String::from("list"),
                cognome: String::from("list"),
            };

            let contatto_serialized = serde_json::to_string(&fake_contatto);

            let _ = socket.send_to(contatto_serialized.unwrap().as_bytes(), "127.0.0.1:4242").expect("Errore di invio!!!");
        }
        else {
            println!("Comando non conosciuto");
        }

        // Ricevimento risposta dal server
        let (num_bytes, _src_addr) = socket.recv_from(&mut recv_data).expect("Errore ricezione!!");
        // Da array di byte a stringa
        let command_data = String::from_utf8_lossy(&recv_data[..num_bytes]);
        // Da JSON a struct Persona
        let rubrica: Result<Vec<Persona>, serde_json::Error> = serde_json::from_str(&command_data);

        match rubrica {
            Ok(rubrica) => {
                // Stampa i dati formattati
                println!("--- Rubrica ---");
                for contatto in rubrica {
                    println!("Nome: {}\nCognome: {}\n", contatto.nome, contatto.cognome);
                }
            },
            Err(err) => {
                eprintln!("Errore durante la deserializzazione del JSON: {}", err);
            }
        }
    }
}

// Cerca una stringa in un vettore di stringhe
// TODO: miglior gestione della stringa non trovata
fn find_str(str_vec: &Vec<String>, target: String) -> usize {
    let mut pos = 0;
    for str in str_vec {
        if *str == target {
            return pos
        }

        pos += 1;
    }

    0
}

// Ritorna i valori dei parametri di --name e --surname
fn get_param(str_vec: &Vec<String>, param: String) -> String {
    let option_pos = find_str(str_vec, param);
    str_vec[option_pos + 1].clone()
}

