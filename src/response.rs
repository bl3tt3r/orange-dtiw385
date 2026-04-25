use serde::Deserialize;

/// Top-level wrapper around the decoder's HTTP response.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<D> {
    pub result: ApiResult<D>,
}

/// Inner result payload returned by the decoder API.
///
/// `response_code` is `"0"` on success; any other value indicates an error,
/// with details available in `message`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResult<D> {
    pub response_code: String,
    pub message: String,
    pub data: D,
}

/// Empty data payload returned by key commands (press, hold, release).
///
/// The decoder returns an empty JSON object `{}` for these operations.
/// Used internally — public methods expose `()` instead.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiEmpty {}

/// Device information returned by the decoder's status endpoint.
///
/// Obtained via [`Decoder::infos`](crate::Decoder::infos).
#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "serializable", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct ApiInfosData {
    /// Type of the currently played media (e.g. `"LIVE"`, `"VOD"`).
    pub played_media_type: String,

    /// Playback state of the current media (e.g. `"PLAY"`, `"PAUSE"`).
    pub played_media_state: String,

    /// Identifier of the currently played media.
    pub played_media_id: String,

    /// Context identifier of the currently played media.
    pub played_media_context_id: String,

    /// Current playback position, in seconds.
    pub played_media_position: String,

    /// Time-shifting state (e.g. `"0"` = disabled).
    pub time_shifting_state: String,

    /// MAC address of the decoder.
    pub mac_address: String,

    /// Whether Wake-on-LAN is supported (`"0"` or `"1"`).
    pub wol_support: String,

    /// Human-readable name assigned to the decoder.
    pub friendly_name: String,

    /// Active standby state (`"0"` = active, `"1"` = standby).
    pub active_standby_state: String,

    /// Whether network PVR is supported (`"0"` or `"1"`).
    pub npvr_support: String,
}
