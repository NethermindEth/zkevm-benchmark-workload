//! SP1 Cluster guest program for stateless validator
//!
//! This is identical to the SP1 guest program since SP1 Cluster uses the same
//! program format and runtime.

#![no_main]

use ere_platform_sp1::{SP1Platform, sp1_zkvm};
use reth_guest::guest::{Guest, RethStatelessValidatorGuest};
use tracing_subscriber::fmt;

sp1_zkvm::entrypoint!(main);

/// Entry point.
pub fn main() {
    init_tracing_just_like_println();
    RethStatelessValidatorGuest::run_output_sha256::<SP1Platform>();
}

/// Initializes a basic `tracing` subscriber that mimics `println!` behavior.
fn init_tracing_just_like_println() {
    let plain = fmt::format()
        .without_time()
        .with_level(false)
        .with_target(false);

    fmt::Subscriber::builder()
        .event_format(plain)
        .with_writer(std::io::stdout)
        .with_max_level(tracing::Level::INFO)
        .init();
}
