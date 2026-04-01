use dtiw385::{Decoders, key::Key};

#[tokio::main]
pub async fn main() {
    let decoder = Decoders::connect([192, 168, 1, 16], 8080);
    decoder
        .press(Key::PowerOnOff)
        .await
        .expect("Unable to switch decoder power");
}
