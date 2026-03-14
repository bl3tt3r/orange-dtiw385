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
