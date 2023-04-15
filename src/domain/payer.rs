use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Payer {
    pub address: String,
    pub evm_address: String,
    pub assets: String,
}

impl Payer {
    pub fn new(
        address: String,
        evm_address: String,
        assets: String,
    ) -> Self {
        Self {
            address,
            evm_address,
            assets,
        }
    }

    pub fn update_assets(&mut self, assets: String) {
        self.assets = assets
    }
}
