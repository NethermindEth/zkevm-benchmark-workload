#![no_main]

use ere_platform_sp1::{sp1_zkvm, SP1Platform};
use ethrex_executor_guest::guest::{EthrexStatelessExecutorGuest, Guest};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    EthrexStatelessExecutorGuest::run_output_sha256::<SP1Platform>();
}
