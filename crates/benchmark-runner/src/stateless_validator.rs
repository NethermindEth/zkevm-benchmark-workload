//! Stateless validator guest program.

use crate::guest_programs::{GenericGuestFixture, GuestFixture};
use anyhow::{bail, Context, Result};
use ere_guests_guest::Guest;
use ere_guests_integration_tests::NoopPlatform;
use ere_guests_stateless_validator_ethrex::{
    guest::StatelessValidatorEthrexGuest,
    host::{build_eip8025_input, Eip8025InputSource},
};
use ere_guests_stateless_validator_reth::guest::{
    StatelessValidatorRethGuest, StatelessValidatorRethInput,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use strum::{AsRefStr, EnumString};
use tracing::info;
use walkdir::WalkDir;
use witness_generator::StatelessValidationFixture;

/// Execution client variants.
#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumString, AsRefStr)]
#[strum(ascii_case_insensitive)]
pub enum ExecutionClient {
    /// Reth stateless block validation guest program.
    Reth,
    /// Ethrex stateless block validation guest program.
    Ethrex,
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
            Self::Nethermind => "unknown",
        }
    }

    /// Like [`Self::version`] but for Nethermind also honours a per-build
    /// sidecar (`<bin_path>/stateless-validator-nethermind-zisk.version`,
    /// written by `build-nethermind-guest.sh`) or the `NETHERMIND_VERSION`
    /// env var, in that order.
    pub fn resolve_version(&self, bin_path: Option<&Path>) -> String {
        if !matches!(self, Self::Nethermind) { return self.version().to_owned(); }
        bin_path
            .and_then(|bp| std::fs::read_to_string(
                bp.join("stateless-validator-nethermind-zisk.version")).ok())
            .or_else(|| std::env::var("NETHERMIND_VERSION").ok())
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| self.version().to_owned())
    }
}

/// Lazily walks a fixture folder and yields each fixture file path.
pub fn iter_benchmark_fixture_paths(path: &Path) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
}

/// Resolves benchmark fixture file paths, optionally filtering by fixture name prefix.
pub fn benchmark_fixture_paths(
    input_folder: &Path,
    selected_fixtures: Option<&[String]>,
) -> Result<Vec<PathBuf>> {
    let available_paths: Vec<_> = iter_benchmark_fixture_paths(input_folder).collect();
    let Some(selected_fixtures) = selected_fixtures.filter(|fixtures| !fixtures.is_empty()) else {
        return Ok(available_paths);
    };

    let fixture_prefixes = normalize_fixture_prefixes(selected_fixtures)?;
    let mut resolved_paths = Vec::new();

    for path in available_paths {
        let fixture_name = fixture_name_from_path(&path)?;
        if fixture_prefixes
            .iter()
            .any(|prefix| fixture_name.starts_with(prefix))
        {
            resolved_paths.push(path);
        }
    }

    Ok(resolved_paths)
}

/// Reads and deserializes a single benchmark fixture file.
pub fn load_benchmark_fixture(path: &Path) -> Result<StatelessValidationFixture> {
    let content = std::fs::read(path)?;
    serde_json::from_slice(&content).with_context(|| format!("Failed to parse {}", path.display()))
}

/// Lazily prepares stateless validator inputs from a fixture folder.
pub fn stateless_validator_input_iter(
    input_folder: &Path,
    selected_fixtures: Option<&[String]>,
    el: ExecutionClient,
    existing_output_dir: Option<&Path>,
) -> Result<impl Iterator<Item = Result<Box<dyn GuestFixture>>>> {
    Ok(stateless_validator_input_iter_from_paths(
        benchmark_fixture_paths(input_folder, selected_fixtures)?.into_iter(),
        el,
        existing_output_dir.map(Path::to_path_buf),
    ))
}

fn stateless_validator_input_iter_from_paths<I>(
    paths: I,
    el: ExecutionClient,
    existing_output_dir: Option<PathBuf>,
) -> impl Iterator<Item = Result<Box<dyn GuestFixture>>>
where
    I: Iterator<Item = PathBuf>,
{
    paths.filter_map(move |path| {
        match skip_existing_fixture_output(&path, existing_output_dir.as_deref()) {
            Ok(true) => None,
            Ok(false) => Some(
                load_benchmark_fixture(&path)
                    .and_then(|fixture| stateless_validator_input_from_fixture(fixture, &path, el)),
            ),
            Err(err) => Some(Err(err)),
        }
    })
}

fn skip_existing_fixture_output(path: &Path, existing_output_dir: Option<&Path>) -> Result<bool> {
    let Some(existing_output_dir) = existing_output_dir else {
        return Ok(false);
    };

    let fixture_name = fixture_name_from_path(path)?;
    let output_path = existing_output_dir.join(format!("{fixture_name}.json"));
    if output_path.exists() {
        info!("Skipping {fixture_name} (already exists)");
        return Ok(true);
    }

    Ok(false)
}

fn fixture_name_from_path(path: &Path) -> Result<String> {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .map(ToOwned::to_owned)
        .with_context(|| format!("Failed to derive fixture name from {}", path.display()))
}

fn normalize_fixture_prefixes(prefixes: &[String]) -> Result<Vec<String>> {
    let mut normalized_prefixes = Vec::with_capacity(prefixes.len());
    let mut seen_prefixes = HashSet::new();

    for prefix in prefixes {
        let normalized_prefix = normalize_fixture_prefix(prefix)?;
        if seen_prefixes.insert(normalized_prefix.clone()) {
            normalized_prefixes.push(normalized_prefix);
        }
    }

    Ok(normalized_prefixes)
}

fn normalize_fixture_prefix(prefix: &str) -> Result<String> {
    let normalized = prefix.trim();
    if normalized.is_empty() {
        bail!("Fixture prefix cannot be empty");
    }

    Ok(normalized
        .strip_suffix(".json")
        .unwrap_or(normalized)
        .to_owned())
}

fn stateless_validator_input_from_fixture(
    fixture: StatelessValidationFixture,
    fixture_path: &Path,
    el: ExecutionClient,
) -> Result<Box<dyn GuestFixture>> {
    match el {
        ExecutionClient::Reth => reth_input_from_fixture(fixture),
        ExecutionClient::Ethrex => ethrex_input_from_fixture(fixture),
        ExecutionClient::Nethermind => nethermind_input_from_fixture(fixture, fixture_path),
    }
}

/// Env var pointing at the `StatelessInputGen` binary built by
/// `build-nethermind-guest.sh`. The Nethermind guest's wire format is
/// canonically produced by that tool — we shell out per fixture.
pub const NETHERMIND_INPUT_GEN_ENV: &str = "NETHERMIND_STATELESS_INPUT_GEN";

fn nethermind_input_from_fixture(
    fixture: StatelessValidationFixture,
    fixture_path: &Path,
) -> Result<Box<dyn GuestFixture>> {
    if !fixture.success {
        bail!("Nethermind guest only validates success fixtures, skip negative {}", fixture.name);
    }
    let stateless_input_gen = std::env::var(NETHERMIND_INPUT_GEN_ENV)
        .map(PathBuf::from)
        .with_context(|| format!("{NETHERMIND_INPUT_GEN_ENV} env var must be set"))?;

    // Synthetic forks at time=0 ⇒ emit chain_config envelope so the guest builds
    // a SpecProvider from it instead of using its hardcoded mainnet schedule.
    let cfg = &fixture.stateless_input.chain_config;
    let has_synthetic_timing = cfg.shanghai_time.is_some()
        || cfg.cancun_time.is_some()
        || cfg.prague_time.is_some()
        || cfg.osaka_time.is_some();

    let tmp = tempfile::tempdir().context("tempdir for StatelessInputGen output")?;
    let mut cmd = std::process::Command::new(&stateless_input_gen);
    cmd.arg("--from-fixture").arg(fixture_path)
        .arg("--no-zisk")
        .arg("--output").arg(tmp.path());
    if has_synthetic_timing {
        cmd.arg("--chain-config-envelope");
    }
    if !cmd.status().context("spawn StatelessInputGen")?.success() {
        bail!("StatelessInputGen failed for {}", fixture.name);
    }

    let block_num = fixture.stateless_input.block.number;
    let stdin = std::fs::read(tmp.path().join(format!("{block_num}.bin")))?;
    let expected_public_values = std::fs::read(tmp.path().join(format!("{block_num}.hash")))?;

    // `with_stdin` (not `with_prefixed_stdin`) — the C# guest reads framed
    // payload directly, no u32 prefix like Rust guests' Platform expects.
    Ok(Box::new(GenericGuestFixture::<BlockMetadata> {
        name: fixture.name,
        input: ere_dockerized::Input::new().with_stdin(stdin),
        expected_public_values,
        metadata: BlockMetadata {
            block_used_gas: fixture.stateless_input.block.gas_used,
        },
    }))
}

fn ethrex_input_from_fixture(fixture: StatelessValidationFixture) -> Result<Box<dyn GuestFixture>> {
    let StatelessValidationFixture {
        name,
        stateless_input,
        success,
    } = fixture;
    let input = build_eip8025_input(Eip8025InputSource::Legacy {
        stateless_input: &stateless_input,
        valid_block: success,
    })
    .context("Failed to create Ethrex stateless validator input")?;
    let output = StatelessValidatorEthrexGuest::compute::<NoopPlatform>(input.clone());
    let metadata = BlockMetadata {
        block_used_gas: stateless_input.block.gas_used,
    };

    Ok(
        GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorEthrexGuest>(
            name, input, output, metadata,
        )?
        .output_sha256()
        .into_boxed(),
    )
}

fn reth_input_from_fixture(fixture: StatelessValidationFixture) -> Result<Box<dyn GuestFixture>> {
    let StatelessValidationFixture {
        name,
        stateless_input,
        success,
    } = fixture;
    info!(
        "Preparing Reth stateless validator input for fixture {}",
        name
    );
    let input = StatelessValidatorRethInput::new(&stateless_input, success)
        .context("Failed to create Reth stateless validator input")?;

    let output = StatelessValidatorRethGuest::compute::<NoopPlatform>(input.clone());
    let metadata = BlockMetadata {
        block_used_gas: stateless_input.block.gas_used,
    };

    Ok(
        GenericGuestFixture::<BlockMetadata>::new::<StatelessValidatorRethGuest>(
            name, input, output, metadata,
        )?
        .output_sha256()
        .into_boxed(),
    )
}
