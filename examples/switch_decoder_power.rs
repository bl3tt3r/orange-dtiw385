use dtiw385::{Decoders, key::Key};

// Target decoder — update these to match your device
const IP: [u8; 4] = [192, 168, 1, 16];
const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let decoder = Decoders::connect(IP, PORT);

    // Send a short press on the power button (down + up)
    // Use hold() + release() if you need to control the duration
    decoder
        .press(Key::PowerOnOff)
        .await
        .expect("Unable to switch decoder power");
}
