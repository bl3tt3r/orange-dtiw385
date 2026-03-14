use std::{
    io::{self, Write},
    net::Ipv4Addr,
    process::exit,
};

use orange_dtiw385::{Infos, Mode, Operation, cmd::Cmd, decoder::Decoder, key};

#[tokio::main]
pub async fn main() {
    println!("---------------------------------------------------------------");
    println!("| Bienvenue dans le cli de controlle de votre decoder DTIW385 |");
    println!("---------------------------------------------------------------");

    let ip: Ipv4Addr =
        ask_for_value("> Entrez l'adresse ip de votre decoder (e.g. 192.168.1.16) :")
            .parse()
            .unwrap_or_else(|_| {
                println!("! L'adresse ip fournit est invalide.");
                exit(1);
            });
    let port: u16 = ask_for_value("> Entrez le port de votre decoder (e.g. 8080) :")
        .parse()
        .unwrap_or_else(|_| {
            println!("! Le port fournit est invalide.");
            exit(1);
        });

    println!("| Connexion au decoder ...");
    let decoder = Decoder::new(ip).with_port(port);

    if let Ok(_) = decoder.infos().await {
        println!("| Decoder trouvé");

        let mut action = 'i';

        while action != 'q' {
            if action == 'i' {
                if let Ok(infos) = decoder.infos().await {
                    println!("| Nom : {}", infos.friendly_name);
                    println!("| Actif : {}", infos.active_standby_state);
                } else {
                    println!("! Le decoder est innaceesible !");
                    exit(1);
                }
            } else if action == 'o' {
                match decoder
                    .send::<()>(
                        Some(Operation::SendKey),
                        Some(&key::Power::OnOff),
                        Some(Mode::Press),
                    )
                    .await
                {
                    Ok(_) => println!("| Action realisé"),
                    Err(_) => println!("| Action impossible"),
                }
            } else if action == 'q' {
                println!("| Au revoir.");
                exit(0);
            } else {
                println!("| Action inconnu !");
            }

            println!("| Envoyer un action au decoder ?");
            println!("| q = quitter");
            println!("| i = information");
            println!("| o = on / off");

            action = ask_for_value("action (q,i,p) : ").parse().unwrap();
        }
    } else {
        println!("! Decoder introuvable");
        exit(1);
    }
}

fn ask_for_value(request: &str) -> String {
    print!("{}", request);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
