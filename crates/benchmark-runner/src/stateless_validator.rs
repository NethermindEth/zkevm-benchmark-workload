//! Stateless validator guest program.

mod eest;
mod fixtures;
mod inputs;

use crate::guest_programs::GuestFixture;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use strum::{AsRefStr, EnumString};

pub use fixtures::{benchmark_fixture_paths, iter_benchmark_fixture_paths, load_benchmark_fixture};

/// Execution client variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum ExecutionClient {
    /// Reth stateless block validation guest program.
    Reth,
    /// Ethrex stateless block validation guest program.
    Ethrex,
    /// Zilkworm stateless block validation guest program.
    Zilkworm,
    /// Zesu stateless block validation guest program.
    Zesu,
    /// Nethermind stateless block validation guest program (Zisk-only, externally built).
    Nethermind,
}

/// Extra information about the block being benchmarked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// Gas used by the block
    pub block_used_gas: u64,
}

impl ExecutionClient {
    /// Returns the version string of the execution client (tag or short commit hash),
    /// extracted from the resolved `Cargo.lock` at build time.
    pub const fn version(&self) -> &'static str {
        match self {
            Self::Reth => env!("RETH_EL_VERSION"),
            Self::Ethrex => env!("ETHREX_EL_VERSION"),
            Self::Zilkworm => env!("ZILKWORM_EL_VERSION"),
            // Externally shipped (not via ere-guests): real version comes from the
            // guest artifact source label. Zesu is a TODO until it's republished
            // through ere-guests and `--guest-artifact-base-url` can be dropped.
            Self::Zesu | Self::Nethermind => "unknown",
        }
    }
}

/// Lazily prepares stateless validator inputs from a fixture folder.
pub fn stateless_validator_input_iter(
    input_folder: &Path,
    selected_fixtures: Option<&[String]>,
    el: ExecutionClient,
    existing_output_dir: Option<&Path>,
) -> Result<impl Iterator<Item = Result<Box<dyn GuestFixture>>>> {
    fixtures::stateless_validator_input_iter(
        input_folder,
        selected_fixtures,
        el,
        existing_output_dir,
    )
}
