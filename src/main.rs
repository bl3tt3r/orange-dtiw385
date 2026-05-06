use axum::{
    extract::Path,
    response::Html,
    routing::{get, post},
    Router,
};
use dtiw385::{key::Key, Decoders};

const IP: [u8; 4] = [192, 168, 1, 19];
const PORT: u16 = 8080;

async fn index() -> Html<&'static str> {
    Html(include_str!("index.html"))
}

async fn press(Path(key_name): Path<String>) -> String {
    let decoder = Decoders::connect(IP, PORT);
    let key = match key_name.as_str() {
        "power"     => Key::PowerOnOff,
        "ok"        => Key::Ok,
        "up"        => Key::Up,
        "down"      => Key::Down,
        "left"      => Key::Left,
        "right"     => Key::Right,
        "back"      => Key::Back,
        "menu"      => Key::Menu,
        "vol_up"    => Key::VolumeUp,
        "vol_down"  => Key::VolumeDown,
        "mute"      => Key::Mute,
        "ch_up"     => Key::ChannelUp,
        "ch_down"   => Key::ChannelDown,
        "play"      => Key::Play,
        "pause"     => Key::Pause,
        "stop"      => Key::Stop,
        "forward"   => Key::Forward,
        "rewind"    => Key::Rewind,
        "n0"        => Key::N0,
        "n1"        => Key::N1,
        "n2"        => Key::N2,
        "n3"        => Key::N3,
        "n4"        => Key::N4,
        "n5"        => Key::N5,
        "n6"        => Key::N6,
        "n7"        => Key::N7,
        "n8"        => Key::N8,
        "n9"        => Key::N9,
        _           => return "unknown key".to_string(),
    };
    match decoder.press(key).await {
        Ok(_)  => "ok".to_string(),
        Err(e) => format!("error: {e}"),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/press/{key}", post(press));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🎮 Télécommande dispo sur http://192.168.1.14:3000");
    axum::serve(listener, app).await.unwrap();
}