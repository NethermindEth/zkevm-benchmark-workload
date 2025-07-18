#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io, path::Path, time::Duration};
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use thiserror::Error;

/// Represents a single profile run.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ProfileRun {
    /// Name of the test case.
    pub name: String,
    /// Gas used during execution.
    pub gas_used: u64,
    /// Execution metrics for the profile run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionMetrics>,
    /// Proving metrics for the profile run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proving: Option<ProvingMetrics>,
}

/// Hardware specs of the profile runner.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HardwareInfo {
    /// CPU model name.
    pub cpu_model: String,
    /// Total RAM in GiB.
    pub total_ram_gib: u64,
    /// Available GPUs.
    pub gpus: Vec<GpuInfo>,
}

/// Information about a GPU.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct GpuInfo {
    /// GPU model name.
    pub model: String,
}

impl HardwareInfo {
    /// Detects hardware information from the current system.
    pub fn detect() -> Self {
        let mut system = System::new();
        system.refresh_specifics(RefreshKind::everything().with_cpu(CpuRefreshKind::everything()));

        Self {
            cpu_model: system
                .cpus()
                .first()
                .map(|cpu| cpu.brand().to_string())
                .unwrap_or_else(|| "Unknown CPU".to_string()),
            total_ram_gib: system.total_memory() / (1024 * 1024 * 1024),
            gpus: detect_gpus(),
        }
    }

    /// Serializes the hardware information to a JSON string in the provided path.
    pub fn to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), MetricsError> {
        let path = path.as_ref();
        ensure_parent_dirs(path)?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

/// Detects available GPUs on the system.
fn detect_gpus() -> Vec<GpuInfo> {
    let mut gpus = Vec::new();

    if let Ok(output) = std::process::Command::new("nvidia-smi")
        .arg("--query-gpu=gpu_name")
        .arg("--format=csv,noheader,nounits")
        .output()
    {
        if output.status.success() {
            let gpu_names = String::from_utf8_lossy(&output.stdout);
            for line in gpu_names.lines() {
                let gpu_name = line.trim();
                if !gpu_name.is_empty() {
                    gpus.push(GpuInfo {
                        model: gpu_name.to_string(),
                    });
                }
            }
        }
    }

    gpus
}

/// Information about a crash that occurred during a workload.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct CrashInfo {
    /// The reason for the crash (e.g., panic message).
    pub reason: String,
}

/// Metrics for execution workloads, either successful or crashed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMetrics {
    /// Metrics for a successful execution workload.
    Success {
        /// Total number of cycles for the entire workload execution.
        total_num_cycles: u64,
        /// Region-specific cycles, mapping region names to their cycle counts.
        region_cycles: HashMap<String, u64>,
        /// Execution duration.
        execution_duration: Duration,
        /// Output data from execution.
        output: Option<Vec<u8>>,
        /// Error message if any.
        error: Option<String>,
    },
    /// Metrics for a crashed execution workload.
    Crashed(CrashInfo),
}

/// Metrics for proving workloads, either successful or crashed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProvingMetrics {
    /// Metrics for a successful proving workload.
    Success {
        /// Proof size in bytes.
        proof_size: usize,
        /// Proving time in milliseconds.
        proving_time_ms: u128,
        /// Total number of cycles used.
        cycles_used: u64,
        /// Output data from proving.
        output: Option<Vec<u8>>,
        /// Error message if any.
        error: Option<String>,
    },
    /// Metrics for a crashed proving workload.
    Crashed(CrashInfo),
}

/// Errors that can occur during metrics processing.
#[derive(Error, Debug)]
pub enum MetricsError {
    /// Error during JSON serialization or deserialization.
    #[error("serde (de)serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Error during file system I/O operations.
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

impl ProfileRun {
    /// Serializes a list of `ProfileRun` into a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Serde` if serialization fails.
    pub fn to_json(items: &[Self]) -> Result<String, MetricsError> {
        serde_json::to_string(items).map_err(MetricsError::from)
    }

    /// Deserializes a list of `ProfileRun` from a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Serde` if deserialization fails.
    pub fn from_json(json: &str) -> Result<Vec<Self>, MetricsError> {
        serde_json::from_str(json).map_err(MetricsError::from)
    }

    /// Serializes `items` using JSON pretty-print and writes them to `path` atomically.
    ///
    /// The file is created if it does not exist and truncated if it does.
    /// Parent directories are created if they are missing.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Io` if any filesystem operation fails.
    /// Returns `MetricsError::Serde` if JSON serialization fails.
    pub fn to_path<P: AsRef<Path>>(path: P, items: &[Self]) -> Result<(), MetricsError> {
        let path = path.as_ref();
        ensure_parent_dirs(path)?;
        let json = serde_json::to_string_pretty(items)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Reads the file at `path` and deserializes a `Vec<ProfileRun>` from its JSON content.
    ///
    /// # Errors
    ///
    /// Returns `MetricsError::Io` if reading the file fails.
    /// Returns `MetricsError::Serde` if JSON deserialization fails.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Vec<Self>, MetricsError> {
        let contents = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&contents)?)
    }
}

fn ensure_parent_dirs<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    if let Some(parent) = path.as_ref().parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_hardware_info_detection() {
        let info = HardwareInfo::detect();
        assert!(!info.cpu_model.is_empty());
        assert!(info.total_ram_gib > 0);
    }

    #[test]
    fn test_profile_run_serialization() {
        let run = ProfileRun {
            name: "test_case".to_string(),
            gas_used: 12345,
            execution: Some(ExecutionMetrics::Success {
                total_num_cycles: 1000,
                region_cycles: HashMap::from_iter([
                    ("setup".to_string(), 100),
                    ("compute".to_string(), 800),
                    ("teardown".to_string(), 100),
                ]),
                execution_duration: Duration::from_millis(150),
                output: Some(vec![1, 2, 3]),
                error: None,
            }),
            proving: Some(ProvingMetrics::Success {
                proof_size: 256,
                proving_time_ms: 2000,
                cycles_used: 5000,
                output: Some(vec![4, 5, 6]),
                error: None,
            }),
        };

        let json = ProfileRun::to_json(&[run.clone()]).unwrap();
        let deserialized = ProfileRun::from_json(&json).unwrap();
        assert_eq!(deserialized, vec![run]);
    }
} 