use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<D> {
    pub result: Result<D>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result<D> {
    pub response_code: String,
    pub message: String,
    pub data: D,
}

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
