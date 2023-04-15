use ethers::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Client {
    pub client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl Client {
    pub async fn new() -> anyhow::Result<Self> {
        let rpc_url = dotenvy::var("RPC_ENDPOINT")?;
        let provider: Provider<Http> = Provider::<Http>::try_from(rpc_url)?;

        let priv_key = dotenvy::var("SERVER_PRIVATE_KEY")?;
        let chain_id = dotenvy::var("CHAIN_ID")?;
        let chain_id: u64 = chain_id.parse()?;
        let wallet = priv_key.parse::<LocalWallet>()?.with_chain_id(chain_id);

        let client = SignerMiddleware::new_with_provider_chain(provider, wallet)
            .await
            .unwrap();
        let client = Arc::new(client);

        Ok(Client { client })
    }
}
