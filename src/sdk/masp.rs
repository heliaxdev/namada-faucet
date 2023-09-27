use borsh::{BorshDeserialize, BorshSerialize};
use masp_proofs::prover::LocalTxProver;
use namada::sdk::masp::{ShieldedContext, ShieldedUtils};

#[derive(Default)]
pub struct SdkShieldedCtx {
    pub shielded_context: ShieldedContext<SdkShieldedUtils>,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, Default)]
pub struct SdkShieldedUtils {}

#[async_trait::async_trait]
impl ShieldedUtils for SdkShieldedUtils {
    fn local_tx_prover(&self) -> LocalTxProver {
        panic!();
    }

    async fn load(self) -> std::io::Result<ShieldedContext<Self>> {
        panic!();
    }

    async fn save(&self, _ctx: &ShieldedContext<Self>) -> std::io::Result<()> {
        panic!();
    }
}
