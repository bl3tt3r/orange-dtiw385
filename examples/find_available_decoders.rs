use std::ops::RangeInclusive;

use dtiw385::Decoders;

const IPS_RANGE: RangeInclusive<[u8; 4]> = [192, 168, 1, 0]..=[192, 168, 1, 255];
const PORTS_RANGE: RangeInclusive<u16> = 8080..=8080;

#[tokio::main]
pub async fn main() {
    let mut decoders = Decoders::search(IPS_RANGE, PORTS_RANGE).find();
    let mut decoder_found = false;

    while let Some(decoder) = decoders.recv().await {
        let ip = decoder.ip();
        match decoder.infos().await {
            Ok(infos) => println!("Decoder found on {} = {} ", ip, infos.friendly_name),
            Err(error) => println!("Error on {} : {}", ip, error),
        }
        decoder_found = true;
    }

    if !decoder_found {
        println!("No one decoder found.");
    }
}
