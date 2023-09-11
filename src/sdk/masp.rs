use borsh::{BorshDeserialize, BorshSerialize};
use masp_proofs::prover::LocalTxProver;
use namada::ledger::masp::{ShieldedContext, ShieldedUtils};

pub struct SdkShieldedCtx {
    pub shielded_context: ShieldedContext<SdkShieldedUtils>,
}

impl Default for SdkShieldedCtx {
    fn default() -> Self {
        Self {
            shielded_context: Default::default(),
        }
    }
}

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct SdkShieldedUtils { }

impl Default for SdkShieldedUtils {
    fn default() -> Self {
        Self { }
    }
}

#[async_trait::async_trait(?Send)]
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
