use dtiw385::Decoders;
use std::ops::RangeInclusive;

// Subnet to scan — adjust to match your local network
const IPS_RANGE: RangeInclusive<[u8; 4]> = [192, 168, 1, 0]..=[192, 168, 1, 255];

// Only probe port 8080; extend the range (e.g. 8080..=8090) to scan multiple ports
const PORTS_RANGE: RangeInclusive<u16> = 8080..=8080;

#[tokio::main]
pub async fn main() {
    // Build and launch the scan — decoders are streamed back as they respond,
    // without waiting for the full scan to complete
    let mut decoders = Decoders::search(IPS_RANGE, PORTS_RANGE)
        .with_concurrency(50) // probe up to 50 addresses simultaneously
        .find();

    let mut decoder_found = false;

    // Each iteration yields one responding decoder as soon as it is discovered
    while let Some(decoder) = decoders.recv().await {
        let ip = decoder.ip();
        match decoder.infos().await {
            Ok(infos) => println!("Decoder found on {} = {}", ip, infos.friendly_name),
            Err(error) => println!("Error on {} : {}", ip, error),
        }
        decoder_found = true;
    }

    // The channel closes once all addresses have been probed
    if !decoder_found {
        println!("No decoder found.");
    }
}
