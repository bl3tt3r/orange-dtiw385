use dtiw385::Decoders;

#[tokio::main]
pub async fn main() {
    let decoder = Decoders::connect([192, 168, 1, 16], 8080);
    match decoder.name().await {
        Ok(name) => println!("Name : {}", name),
        Err(error) => println!("Error : {}", error),
    }
}
