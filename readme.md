# orange-dtiw385

Une crate Rust pour contrôler à distance un décodeur **Orange DTIW385** via son interface HTTP locale.

## Fonctionnalités

- Lecture des informations du décodeur (nom, état veille, média en cours…)
- Envoi de touches de télécommande (alimentation, navigation, volume, chaînes, lecture, chiffres)
- Changement de chaîne
- Support des modes de touche : appui, maintien, relâchement
- Client HTTP async basé sur `tokio` + `reqwest`

## Installation

Ajoute la crate à ton `Cargo.toml` :

```toml
[dependencies]
orange-dtiw385 = { git = "https://github.com/bl3tt3r/orange-dtiw385" }
```

## Utilisation rapide

```rust
use std::net::Ipv4Addr;
use orange_dtiw385::{Infos, Mode, Operation, cmd::Cmd, decoder::Decoder, key};

#[tokio::main]
async fn main() {
    let decoder = Decoder::new("192.168.1.16".parse::<Ipv4Addr>().unwrap())
        .with_port(8080);

    // Lire les informations du décodeur
    let infos = decoder.infos().await.unwrap();
    println!("Nom : {}", infos.friendly_name);
    println!("En veille : {}", infos.active_standby_state);

    // Envoyer une touche (ex: allumer/éteindre)
    decoder
        .send::<()>(
            Some(Operation::SendKey),
            Some(&key::Power::OnOff),
            Some(Mode::Press),
        )
        .await
        .unwrap();
}
```

## Touches disponibles

| Groupe       | Variantes                                           |
| ------------ | --------------------------------------------------- |
| `Power`      | `OnOff`                                             |
| `Navigation` | `Ok`, `Up`, `Down`, `Left`, `Right`, `Back`, `Menu` |
| `Volume`     | `Up`, `Down`, `Mute`                                |
| `Channel`    | `Up`, `Down`                                        |
| `Playback`   | `Play`, `Pause`, `Stop`, `Forward`, `Rewind`        |
| `Number`     | `N0` … `N9`                                         |

## Modes

| Mode      | Description              |
| --------- | ------------------------ |
| `Press`   | Appui simple             |
| `Hold`    | Maintien de la touche    |
| `Release` | Relâchement de la touche |

## Opérations

| Opération       | Code | Description                |
| --------------- | ---- | -------------------------- |
| `SendKey`       | 1    | Envoyer une touche         |
| `ReadInfos`     | 10   | Lire les infos du décodeur |
| `ChangeChannel` | 9    | Changer de chaîne          |

## API HTTP

La crate communique avec le décodeur via :

```
GET http://{ip}:{port}/remoteControl/cmd?operation={op}&key={code}&mode={mode}
```

Le port par défaut est `8080`.

## CLI

Un exemple de CLI interactif est fourni dans `examples/cli.rs`. Il permet de se connecter à un décodeur et d'envoyer des commandes basiques depuis le terminal.

```bash
cargo run --example cli
```

## Dépendances

| Crate     | Version | Utilisation                  |
| --------- | ------- | ---------------------------- |
| `tokio`   | 1.50    | Runtime async                |
| `reqwest` | 0.13    | Client HTTP                  |
| `serde`   | 1       | Désérialisation des réponses |

## Licence

Ce projet n'a pas encore de licence définie.