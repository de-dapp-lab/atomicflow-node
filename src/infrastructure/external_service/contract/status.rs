use crate::infrastructure::external_service::contract::client::Client;
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::prelude::*;

#[derive(Clone)]
pub struct StatusContract {
    pub contract: Contract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
}

impl StatusContract {
    pub async fn new(client: Client) -> anyhow::Result<Self> {
        let abi: Abi = serde_json::from_str(include_str!("abi/SubscriptionManager.json"))?;

        let contract_address = dotenvy::var("STATUS_CONTRACT_ADDRESS")?;
        let contract =
            Contract::<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>::new(
                contract_address.parse::<Address>()?,
                abi,
                client.client,
            );

        Ok(StatusContract { contract })
    }
}
