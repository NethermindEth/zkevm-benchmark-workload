use ere_platform_risc0::Risc0Platform;
use ethrex_executor_guest::guest::{EthrexStatelessExecutorGuest, Guest};

pub fn main() {
    EthrexStatelessExecutorGuest::run_output_sha256::<Risc0Platform>();
}
