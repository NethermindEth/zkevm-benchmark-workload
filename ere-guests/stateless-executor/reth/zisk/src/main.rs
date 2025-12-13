//! ZisK guest program

#![no_main]

use ere_platform_zisk::{ZiskPlatform, ziskos};
use reth_executor_guest::guest::{Guest, RethStatelessExecutorGuest};

ziskos::entrypoint!(main);

/// Entry point.
pub fn main() {
    RethStatelessExecutorGuest::run_output_sha256::<ZiskPlatform>();
}
