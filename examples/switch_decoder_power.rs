use dtiw385::{Decoders, key::Key};

const IP: [u8; 4] = [192, 168, 1, 16];
const PORT: u16 = 8080;

#[tokio::main]
pub async fn main() {
    let decoder = Decoders::connect(IP, PORT);
    decoder
        .press(Key::PowerOnOff)
        .await
        .expect("Unable to switch decoder power");
}
