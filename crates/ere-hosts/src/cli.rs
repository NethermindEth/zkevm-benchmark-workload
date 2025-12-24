//! CLI definitions for the zkVM benchmarker

use std::path::PathBuf;

use anyhow::Result;
use benchmark_runner::{runner::Action, stateless_executor, stateless_validator};
use clap::{Parser, Subcommand, ValueEnum};
use ere_dockerized::zkVMKind;
use ere_zkvm_interface::{ClusterProverConfig, ProverResourceType};

/// Command line interface for the zkVM benchmarker
#[derive(Parser)]
#[command(name = "zkvm-benchmarker")]
#[command(about = "Benchmark different Ere compatible zkVMs")]
#[command(version)]
#[derive(Debug)]
pub struct Cli {
    /// Resource type for proving
    #[arg(short, long, value_enum, default_value = "gpu")]
    pub resource: Resource,

    /// Action to perform
    #[arg(short, long, value_enum, default_value = "prove")]
    pub action: BenchmarkAction,

    /// zkVM instances to benchmark
    #[arg(long, required(true), value_parser = <zkVMKind as std::str::FromStr>::from_str)]
    pub zkvms: Vec<zkVMKind>,

    /// Rerun the benchmarks even if the output folder already contains results
    #[arg(long, default_value_t = false)]
    pub force_rerun: bool,

    /// Guest program to benchmark
    #[command(subcommand)]
    pub guest_program: GuestProgramCommand,

    /// Output folder for benchmark results
    #[arg(short, long, default_value = "zkevm-metrics")]
    pub output_folder: PathBuf,

    /// Output folder for dumping input files used in benchmarks
    #[arg(long)]
    pub dump_inputs: Option<PathBuf>,

    /// Cluster endpoint URL (for --resource cluster)
    #[arg(long, env = "SP1_CLUSTER_ENDPOINT")]
    pub cluster_endpoint: Option<String>,

    /// Cluster Redis URL for artifact storage (for --resource cluster)
    #[arg(long, env = "SP1_CLUSTER_REDIS_URL")]
    pub cluster_redis_url: Option<String>,

    /// Number of GPUs to use for cluster proving (for --resource cluster)
    #[arg(long, env = "SP1_CLUSTER_NUM_GPUS")]
    pub cluster_num_gpus: Option<u32>,
}

/// Subcommands for different guest programs
#[derive(Subcommand, Clone, Debug)]
pub enum GuestProgramCommand {
    /// Ethereum Stateless Executor
    StatelessExecutor {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input", conflicts_with = "input_file")]
        input_folder: PathBuf,
        /// Input file for a single benchmark fixture
        #[arg(long)]
        input_file: Option<PathBuf>,
        /// Execution client to benchmark
        #[arg(short, long)]
        execution_client: ExecutionClient,
    },
    /// Ethereum Stateless Validator
    StatelessValidator {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input", conflicts_with = "input_file")]
        input_folder: PathBuf,
        /// Input file for a single benchmark fixture
        #[arg(long)]
        input_file: Option<PathBuf>,
        /// Execution client to benchmark
        #[arg(short, long)]
        execution_client: ExecutionClient,
    },
    /// Empty program
    EmptyProgram,

    /// Block encoding length
    BlockEncodingLength {
        /// Input folder for benchmark fixtures
        #[arg(short, long, default_value = "zkevm-fixtures-input")]
        input_folder: PathBuf,

        /// Number of times to loop the benchmark
        #[arg(long)]
        loop_count: u16,

        /// Encoding format
        #[arg(short, long, value_enum)]
        format: BlockEncodingFormat,
    },
}

/// Encoding formats for block encoding length program
#[derive(Debug, Clone, ValueEnum)]
pub enum BlockEncodingFormat {
    /// RLP encoding
    Rlp,
    /// SSZ encoding
    Ssz,
}

/// Execution clients for the stateless validator
#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum ExecutionClient {
    /// Reth execution client
    Reth,
    /// Ethrex execution client
    Ethrex,
}

impl ExecutionClient {
    /// Get the guest relative path for the execution client
    pub fn guest_rel_path(&self, guest_program: &str) -> Result<PathBuf> {
        let validator_path = match self {
            Self::Reth => "stateless-validator/reth",
            Self::Ethrex => "stateless-validator/ethrex",
        };
        let executor_path = match self {
            Self::Reth => "stateless-executor/reth",
            Self::Ethrex => "stateless-executor/ethrex",
        };
        let path = match guest_program {
            "stateless-validator" => PathBuf::from(validator_path),
            "stateless-executor" => PathBuf::from(executor_path),
            "empty-program" => PathBuf::from("empty-program"),
            "block-encoding-length" => PathBuf::from("block-encoding-length"),
            _ => return Err(anyhow::anyhow!("Unknown guest program: {}", guest_program)),
        };
        Ok(path)
    }
}

/// Prover resource types
#[derive(Debug, Clone, ValueEnum)]
pub enum Resource {
    /// CPU resource
    Cpu,
    /// GPU resource
    Gpu,
    /// Multi-GPU cluster (e.g., SP1 Cluster)
    Cluster,
}

/// Benchmark actions
#[derive(Debug, Clone, ValueEnum)]
pub enum BenchmarkAction {
    /// Only do zkVM execution
    Execute,
    /// Create a zkVM proof
    Prove,
}

impl Cli {
    /// Convert CLI resource options to ProverResourceType
    pub fn prover_resource_type(&self) -> ProverResourceType {
        match self.resource {
            Resource::Cpu => ProverResourceType::Cpu,
            Resource::Gpu => ProverResourceType::Gpu,
            Resource::Cluster => ProverResourceType::Cluster(ClusterProverConfig {
                endpoint: self.cluster_endpoint.clone().unwrap_or_default(),
                api_key: None,
                num_gpus: self.cluster_num_gpus,
                redis_url: self.cluster_redis_url.clone(),
            }),
        }
    }
}

impl From<BenchmarkAction> for Action {
    fn from(action: BenchmarkAction) -> Self {
        match action {
            BenchmarkAction::Execute => Self::Execute,
            BenchmarkAction::Prove => Self::Prove,
        }
    }
}

impl From<BlockEncodingFormat> for block_encoding_length_guest::guest::BlockEncodingFormat {
    fn from(format: BlockEncodingFormat) -> Self {
        match format {
            BlockEncodingFormat::Rlp => Self::Rlp,
            BlockEncodingFormat::Ssz => Self::Ssz,
        }
    }
}

impl From<ExecutionClient> for stateless_validator::ExecutionClient {
    fn from(client: ExecutionClient) -> Self {
        match client {
            ExecutionClient::Reth => Self::Reth,
            ExecutionClient::Ethrex => Self::Ethrex,
        }
    }
}

impl From<ExecutionClient> for stateless_executor::ExecutionClient {
    fn from(client: ExecutionClient) -> Self {
        match client {
            ExecutionClient::Reth => Self::Reth,
            ExecutionClient::Ethrex => Self::Ethrex,
        }
    }
}