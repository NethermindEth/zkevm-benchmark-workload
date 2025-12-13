//! Risc0 guest program

use ere_platform_risc0::Risc0Platform;
use reth_executor_guest::guest::{Guest, RethStatelessExecutorGuest};

/// Entry point.
pub fn main() {
    RethStatelessExecutorGuest::run_output_sha256::<Risc0Platform>();
}
