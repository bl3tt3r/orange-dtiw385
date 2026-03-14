use reqwest::Error;
use serde::Deserialize;

use crate::{cmd::Cmd, decoder::Decoder};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InfosData {
    pub played_media_type: String,
    pub played_media_state: String,
    pub played_media_id: String,
    pub played_media_context_id: String,
    pub played_media_position: String,
    pub time_shifting_state: String,
    pub mac_address: String,
    pub wol_support: String,
    pub friendly_name: String,
    pub active_standby_state: String,
    pub npvr_support: String,
}

pub trait Infos {
    fn infos(&self) -> impl Future<Output = Result<InfosData, Error>>;
}

impl Infos for Decoder {
    async fn infos(&self) -> Result<InfosData, Error> {
        Ok(self
            .send::<InfosData>(Some(crate::Operation::ReadInfos), None, None)
            .await?
            .result
            .data)
    }
}
