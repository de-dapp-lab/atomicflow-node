use crate::infrastructure::external_service::intmax::F;
use dirs;
use intmax::utils::key_management::memory::{SerializableWalletOnMemory, WalletOnMemory};
use intmax::utils::key_management::types::Wallet;
use intmax_rollup_interface::intmax_zkp_core::zkdsa::account::Address;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::debug;

#[derive(Clone)]
pub struct WalletRepository {
    pub wallet_file_path: PathBuf,
}

impl WalletRepository {
    pub fn new(aggregator_url: &str) -> Self {
        let mut intmax_dir = dirs::home_dir().expect("fail to get home directory");
        intmax_dir.push(".intmax");

        let mut wallet_dir_path = intmax_dir.clone();
        assert!(!aggregator_url.is_empty());
        wallet_dir_path.push(aggregator_url);

        let mut nickname_file_path = wallet_dir_path.clone();
        nickname_file_path.push("nickname");

        let mut wallet_file_path = wallet_dir_path.clone();
        wallet_file_path.push("wallet");

        debug!("wallet file path: {:?}", wallet_file_path);

        Self { wallet_file_path }
    }

    pub fn backup(&self, wallet: &WalletOnMemory) -> anyhow::Result<()> {
        wallet.backup()?;

        Ok(())
    }

    pub fn get_default_account(
        &self,
        wallet: &WalletOnMemory,
    ) -> anyhow::Result<Option<Address<F>>> {
        let account = wallet.get_default_account();

        Ok(account)
    }

    pub fn get_wallet(&self) -> anyhow::Result<WalletOnMemory> {
        let wallet = WalletOnMemory::read_from_file(self.wallet_file_path.clone())?;

        Ok(wallet)
    }

    pub fn encode_wallet(&self, wallet: WalletOnMemory) -> anyhow::Result<String> {
        let raw = SerializableWalletOnMemory {
            data: wallet.data.values().cloned().collect::<Vec<_>>(),
            default_account: wallet.default_account,
        };
        let encoded_wallet = serde_json::to_string(&raw)?;
        Ok(encoded_wallet)
    }

    pub fn decode_wallet(&self, encoded_wallet: &str) -> anyhow::Result<WalletOnMemory> {
        let raw: SerializableWalletOnMemory = serde_json::from_str(encoded_wallet)?;
        let mut result = HashMap::new();
        for value in raw.data.into_iter() {
            result.insert(value.account.address, value);
        }

        Ok(WalletOnMemory {
            data: result,
            default_account: raw.default_account,
            wallet_file_path: self.wallet_file_path.clone(),
        })
    }
}
