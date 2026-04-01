use std::{net::Ipv4Addr, result::Result};

use reqwest::Error;
use serde::de::DeserializeOwned;

use crate::{Mode, Operation, Response, decoder::Decoder, keys::Key};

pub trait Cmd {
    fn send<D: DeserializeOwned>(
        &self,
        operation: Option<Operation>,
        code: Option<&dyn Key>,
        mode: Option<Mode>,
    ) -> impl Future<Output = Result<Response<D>, Error>>;
}

impl Cmd for Decoder {
    async fn send<D: DeserializeOwned>(
        &self,
        operation: Option<Operation>,
        key: Option<&dyn Key>,
        mode: Option<Mode>,
    ) -> Result<Response<D>, Error> {
        let url = &generate_url(&self.ip, &self.port, key, operation, mode);
        println!("url : {}", url);
        self.client
            .get(url)
            .send()
            .await?
            .json::<Response<D>>()
            .await
    }
}

fn generate_url(
    ip: &Ipv4Addr,
    port: &u16,
    code: Option<&dyn Key>,
    operation: Option<Operation>,
    mode: Option<Mode>,
) -> String {
    let base_url = format!("http://{}:{}/remoteControl/cmd", ip, port);
    let params = [
        code.map(|c| format!("key={}", c.get_code())),
        operation.map(|o| format!("operation={}", o.value())),
        mode.map(|m| format!("mode={}", m.value())),
    ];
    let params_query = params
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("&");
    format!("{}?{}", base_url, params_query)
}
