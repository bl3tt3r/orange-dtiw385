use dtiw385::Decoders;

const IP: [u8; 4] = [192, 168, 1, 16];
const PORT: u16 = 8080;

#[tokio::main]
pub async fn main() {
    let decoder = Decoders::connect(IP, PORT);
    match decoder.infos().await {
        Ok(infos) => {
            println!("played_media_type: {}", infos.played_media_type);
            println!("played_media_state: {}", infos.played_media_state);
            println!("played_media_id: {}", infos.played_media_id);
            println!("played_media_context_id: {}", infos.played_media_context_id);
            println!("played_media_position: {}", infos.played_media_position);
            println!("time_shifting_state: {}", infos.time_shifting_state);
            println!("mac_address: {}", infos.mac_address);
            println!("wol_support: {}", infos.wol_support);
            println!("friendly_name: {}", infos.friendly_name);
            println!("active_standby_state: {}", infos.active_standby_state);
            println!("npvr_support: {}", infos.npvr_support);
        }
        Err(error) => println!("Error : {}", error),
    }
}
