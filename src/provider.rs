use alloy_provider::{
    Identity, ProviderBuilder, RootProvider,
    fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
};
use alloy_rpc_client::RpcClient;
use alloy_rpc_types_engine::{Claims, JwtSecret};
use alloy_transport::Authorization;
use alloy_transport_http::{Http, reqwest::Client};
use reqwest::header::HeaderValue;

pub type NodlProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

fn auth(jwt: JwtSecret) -> eyre::Result<Authorization> {
    let claims = Claims::default();
    let token = jwt.encode(&claims)?;

    let auth = Authorization::Bearer(token);

    Ok(auth)
}

#[derive(Debug, Default)]
pub struct NodlProviderBuilder {
    pub jwt: Option<JwtSecret>,
    pub url: String,
}

impl NodlProviderBuilder {
    pub fn jwt(mut self, jwt: Option<JwtSecret>) -> Self {
        self.jwt = jwt;
        self
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn build(self) -> eyre::Result<NodlProvider> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(jwt) = self.jwt.clone() {
            let auth = auth(jwt)?;

            let mut auth_value: HeaderValue =
                HeaderValue::from_str(&auth.to_string()).expect("Header should be valid string");
            auth_value.set_sensitive(true);

            headers.insert(reqwest::header::AUTHORIZATION, auth_value);
        };

        let client = Client::builder().default_headers(headers).build()?;
        let transport = Http::with_client(client, self.url.parse()?);
        let rpc_client = RpcClient::new(transport, false);

        Ok(ProviderBuilder::new().on_client(rpc_client))
    }
}
