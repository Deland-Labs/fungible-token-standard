use std::time::Duration;

pub const MAX_TXS_CACHE_IN_DFT: usize = 1000;
// 4G
pub const MAX_HEAP_MEMORY_SIZE: u32 = 4294967295u32;
// 2T
pub const CYCLES_PER_AUTO_SCALING: u64 = 2000_000_000_000;
pub const DEFAULT_FEE_RATE_DECIMALS: u8 = 8;
// transaction window
pub const DEFAULT_TRANSACTION_WINDOW: Duration = Duration::from_secs(24 * 60 * 60);
// default max transactions in windows
pub const DEFAULT_MAX_TRANSACTIONS_IN_WINDOW: Duration = Duration::from_secs(1_000_000);
// permitted drift
pub const PERMITTED_DRIFT: Duration = Duration::from_secs(60);
