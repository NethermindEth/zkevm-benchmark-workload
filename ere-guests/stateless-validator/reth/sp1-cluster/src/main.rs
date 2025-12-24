//! SP1 Cluster guest program for stateless validator
//!
//! This is identical to the SP1 guest program since SP1 Cluster uses the same
//! program format and runtime.

#![no_main]

extern crate alloc;

use reth_stateless_validator_guest::{
    guest::ethereum_guest,
    sdk::{SDK, ScopeMarker},
};
use sp1_zkvm::io::read_vec;
use tracing_subscriber::fmt;

sp1_zkvm::entrypoint!(main);

#[allow(missing_debug_implementations)]
struct SP1ClusterSDK;

impl SDK for SP1ClusterSDK {
    fn read_input() -> Vec<u8> {
        read_vec()
    }

    fn commit_output(output: [u8; 32]) {
        sp1_zkvm::io::commit(&output);
    }

    fn cycle_scope(scope: ScopeMarker, message: &str) {
        match scope {
            ScopeMarker::Start => {
                println!("cycle-tracker-report-start: {message}")
            }
            ScopeMarker::End => {
                println!("cycle-tracker-report-end: {message}")
            }
        }
    }
}

/// Entry point.
pub fn main() {
    init_tracing_just_like_println();
    ethereum_guest::<SP1ClusterSDK>();
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
