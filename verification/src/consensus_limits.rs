use std::sync::Arc;
const LEGACY_MAX_BLOCK_SIZE: usize = 1_000_000;
const LEGACY_MAX_BLOCK_SIGOPS: usize = 20_000;

pub trait ConsensusLimits : Send + Sync {
	fn max_block_sigops(&self) -> usize;
	fn max_block_size(&self) -> usize;
    fn max_transaction_size(&self) -> usize;
}

// Temporary limits to mitigate DOS in Bitcoins infancy.
pub struct LegacyLimits { }

impl LegacyLimits {
    pub fn new() -> Self {
        LegacyLimits { }
    }
}

impl ConsensusLimits for LegacyLimits {
    fn max_block_sigops(&self) -> usize { LEGACY_MAX_BLOCK_SIGOPS }
	fn max_block_size(&self) -> usize { LEGACY_MAX_BLOCK_SIZE }
    fn max_transaction_size(&self) -> usize { LEGACY_MAX_BLOCK_SIZE }
}

pub type ConsensusLimitsRef = Arc<ConsensusLimits>;
