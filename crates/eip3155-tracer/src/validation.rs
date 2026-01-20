//! Gas validation logic for EVM traces.
//!
//! This module provides validation to ensure gas accounting in traces is consistent.
//! The primary validation is that `gasCost == gas_before - gas_after` for each opcode.
//!
//! # Example
//!
//! ```ignore
//! use eip3155_tracer::query::TraceFile;
//! use eip3155_tracer::validation::validate_gas;
//!
//! let trace = TraceFile::load(Path::new("trace.jsonl"))?;
//! let result = validate_gas(&trace);
//!
//! if result.is_valid() {
//!     println!("Gas validation passed!");
//! } else {
//!     for error in &result.errors {
//!         eprintln!("{}", error);
//!     }
//! }
//! ```

use crate::opcodes::{is_call_opcode, is_create_opcode, is_terminating_opcode};
use crate::query::{StructLogEntry, TraceFile};
use serde::Serialize;

/// Result of gas validation.
#[derive(Debug, Clone, Serialize)]
pub struct GasValidationResult {
    /// Whether all validations passed.
    pub valid: bool,
    /// Total number of steps validated.
    pub steps_validated: usize,
    /// Number of steps that passed validation.
    pub steps_passed: usize,
    /// Number of steps skipped (e.g., frame transitions).
    pub steps_skipped: usize,
    /// List of validation errors.
    pub errors: Vec<GasValidationError>,
    /// List of warnings (non-fatal issues).
    pub warnings: Vec<GasValidationWarning>,
}

impl GasValidationResult {
    /// Create a new empty validation result.
    pub fn new() -> Self {
        Self {
            valid: true,
            steps_validated: 0,
            steps_passed: 0,
            steps_skipped: 0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if validation passed.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Add an error and mark result as invalid.
    pub fn add_error(&mut self, error: GasValidationError) {
        self.valid = false;
        self.errors.push(error);
    }

    /// Add a warning (does not affect validity).
    pub fn add_warning(&mut self, warning: GasValidationWarning) {
        self.warnings.push(warning);
    }

    /// Export to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for GasValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// A gas validation error.
#[derive(Debug, Clone, Serialize)]
pub struct GasValidationError {
    /// Transaction index.
    pub tx_index: usize,
    /// Step index within the transaction.
    pub step_index: usize,
    /// Program counter.
    pub pc: u64,
    /// Opcode name.
    pub opcode: String,
    /// Expected gas cost (gas_before - gas_after).
    pub expected_gas_cost: u64,
    /// Reported gas cost (from gasCost field).
    pub reported_gas_cost: u64,
    /// Gas before this operation.
    pub gas_before: u64,
    /// Gas after this operation.
    pub gas_after: u64,
    /// Human-readable error message.
    pub message: String,
}

impl std::fmt::Display for GasValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tx[{}] step[{}] pc={}: {} reports gasCost={} but gas diff={} (gas: {} -> {})",
            self.tx_index,
            self.step_index,
            self.pc,
            self.opcode,
            self.reported_gas_cost,
            self.expected_gas_cost,
            self.gas_before,
            self.gas_after
        )
    }
}

/// A gas validation warning (non-fatal).
#[derive(Debug, Clone, Serialize)]
pub struct GasValidationWarning {
    /// Transaction index.
    pub tx_index: usize,
    /// Step index within the transaction.
    pub step_index: usize,
    /// Warning type.
    pub warning_type: WarningType,
    /// Human-readable message.
    pub message: String,
}

/// Types of validation warnings.
#[derive(Debug, Clone, Serialize)]
pub enum WarningType {
    /// Frame transition (CALL, STATICCALL, etc.) - gas accounting is complex.
    FrameTransition,
    /// Execution terminated (STOP, RETURN, REVERT).
    ExecutionTerminated,
    /// Missing gas information.
    MissingGasInfo,
    /// Depth change detected.
    DepthChange,
    /// Out of gas condition.
    OutOfGas,
}

impl std::fmt::Display for GasValidationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "tx[{}] step[{}] {:?}: {}",
            self.tx_index, self.step_index, self.warning_type, self.message
        )
    }
}

/// Validation options.
#[derive(Debug, Clone)]
pub struct ValidationOptions {
    /// Whether to validate CALL-type opcodes (frame transitions have complex gas).
    pub validate_call_opcodes: bool,
    /// Whether to validate CREATE-type opcodes.
    pub validate_create_opcodes: bool,
    /// Whether to include warnings for skipped steps.
    pub include_warnings: bool,
    /// Maximum number of errors before stopping.
    pub max_errors: Option<usize>,
}

impl Default for ValidationOptions {
    fn default() -> Self {
        Self {
            validate_call_opcodes: true,
            validate_create_opcodes: true,
            include_warnings: true,
            max_errors: None,
        }
    }
}

/// Validate gas consistency in a trace file.
///
/// For each opcode execution, validates that:
/// `gasCost == gas_before - gas_after`
///
/// Special handling is applied for:
/// - CALL/STATICCALL/DELEGATECALL/CALLCODE: Frame transitions have complex gas accounting
/// - CREATE/CREATE2: Contract creation has special gas rules
/// - Terminating opcodes (STOP, RETURN, REVERT): No next step to compare
/// - Depth changes: Indicates entering/exiting a call frame
///
/// # Arguments
///
/// * `trace` - The trace file to validate.
///
/// # Returns
///
/// Returns a `GasValidationResult` containing validation status and any errors.
pub fn validate_gas(trace: &TraceFile) -> GasValidationResult {
    validate_gas_with_options(trace, &ValidationOptions::default())
}

/// Validate gas consistency with custom options.
pub fn validate_gas_with_options(
    trace: &TraceFile,
    options: &ValidationOptions,
) -> GasValidationResult {
    let mut result = GasValidationResult::new();

    for (tx_idx, tx) in trace.transactions.iter().enumerate() {
        // Check if we've hit the error limit
        if let Some(max) = options.max_errors {
            if result.errors.len() >= max {
                break;
            }
        }

        let logs = &tx.struct_logs;
        if logs.is_empty() {
            continue;
        }

        for i in 0..logs.len() {
            result.steps_validated += 1;

            let current = &logs[i];

            // Check if this is the last step
            if i + 1 >= logs.len() {
                result.steps_skipped += 1;
                if options.include_warnings {
                    result.add_warning(GasValidationWarning {
                        tx_index: tx_idx,
                        step_index: i,
                        warning_type: WarningType::ExecutionTerminated,
                        message: format!(
                            "Last step in trace at pc={}, opcode={}",
                            current.pc, current.op_name
                        ),
                    });
                }
                continue;
            }

            let next = &logs[i + 1];

            // Check for depth change (frame transition)
            if next.depth != current.depth {
                result.steps_skipped += 1;
                if options.include_warnings {
                    result.add_warning(GasValidationWarning {
                        tx_index: tx_idx,
                        step_index: i,
                        warning_type: WarningType::DepthChange,
                        message: format!(
                            "Depth change at pc={}: {} -> {} (opcode={})",
                            current.pc, current.depth, next.depth, current.op_name
                        ),
                    });
                }

                // For CALL-type opcodes, we expect a depth change
                if is_call_opcode(current.op) {
                    if !options.validate_call_opcodes {
                        continue;
                    }
                    // Validate call gas forwarding
                    validate_call_gas(tx_idx, i, current, next, logs, &mut result, options);
                }
                continue;
            }

            // Check for CALL opcodes without depth change (precompile calls)
            if is_call_opcode(current.op) {
                // This is likely a precompile call or failed call that didn't enter a new frame
                // The gasCost includes the entire call cost
                result.steps_skipped += 1;
                if options.include_warnings {
                    result.add_warning(GasValidationWarning {
                        tx_index: tx_idx,
                        step_index: i,
                        warning_type: WarningType::FrameTransition,
                        message: format!(
                            "CALL-type opcode at pc={} without depth change (precompile or failed call)",
                            current.pc
                        ),
                    });
                }
                continue;
            }

            // Check for CREATE opcodes
            if is_create_opcode(current.op) {
                if !options.validate_create_opcodes {
                    result.steps_skipped += 1;
                    continue;
                }
                // CREATE has complex gas accounting - skip validation but warn
                result.steps_skipped += 1;
                if options.include_warnings {
                    result.add_warning(GasValidationWarning {
                        tx_index: tx_idx,
                        step_index: i,
                        warning_type: WarningType::FrameTransition,
                        message: format!(
                            "CREATE opcode at pc={} has complex gas accounting",
                            current.pc
                        ),
                    });
                }
                continue;
            }

            // Check for terminating opcodes
            if is_terminating_opcode(current.op) {
                result.steps_skipped += 1;
                if options.include_warnings {
                    result.add_warning(GasValidationWarning {
                        tx_index: tx_idx,
                        step_index: i,
                        warning_type: WarningType::ExecutionTerminated,
                        message: format!(
                            "Terminating opcode {} at pc={}",
                            current.op_name, current.pc
                        ),
                    });
                }
                continue;
            }

            // Standard gas validation: gasCost should equal gas_before - gas_after
            let gas_before = current.gas;
            let gas_after = next.gas;
            let expected_cost = gas_before.saturating_sub(gas_after);
            let reported_cost = current.gas_cost;

            if expected_cost != reported_cost {
                result.add_error(GasValidationError {
                    tx_index: tx_idx,
                    step_index: i,
                    pc: current.pc,
                    opcode: current.op_name.clone(),
                    expected_gas_cost: expected_cost,
                    reported_gas_cost: reported_cost,
                    gas_before,
                    gas_after,
                    message: format!(
                        "Gas mismatch: {} reports gasCost={} but gas diff={} (gas: {} -> {})",
                        current.op_name, reported_cost, expected_cost, gas_before, gas_after
                    ),
                });
            } else {
                result.steps_passed += 1;
            }
        }
    }

    result
}

/// Validate gas for a CALL-type opcode.
///
/// CALL opcodes have complex gas accounting:
/// 1. The caller pays a base cost for the CALL opcode itself
/// 2. Gas is forwarded to the child frame (with 63/64 rule after EIP-150)
/// 3. Unused gas is returned to the caller
fn validate_call_gas(
    tx_idx: usize,
    step_idx: usize,
    call_step: &StructLogEntry,
    _next_step: &StructLogEntry,
    logs: &[StructLogEntry],
    result: &mut GasValidationResult,
    options: &ValidationOptions,
) {
    // For CALL opcodes, the gas accounting is:
    // 1. gasCost on the CALL step = base call cost + gas forwarded to child
    // 2. After the child returns, remaining gas is restored (minus what child used)

    // Find the step when we return from the call (same depth as call step)
    let call_depth = call_step.depth;
    let mut return_step_idx = None;

    for j in (step_idx + 1)..logs.len() {
        if logs[j].depth == call_depth {
            return_step_idx = Some(j);
            break;
        }
    }

    if let Some(return_idx) = return_step_idx {
        let return_step = &logs[return_idx];

        // The gas at return should be approximately:
        // call_step.gas - call_step.gas_cost + returned_gas
        // This is complex to validate precisely, so we just add a warning

        if options.include_warnings {
            let gas_consumed = call_step.gas.saturating_sub(return_step.gas);
            result.add_warning(GasValidationWarning {
                tx_index: tx_idx,
                step_index: step_idx,
                warning_type: WarningType::FrameTransition,
                message: format!(
                    "{} at pc={}: gas {} -> {} (consumed {} during call, gasCost={})",
                    call_step.op_name,
                    call_step.pc,
                    call_step.gas,
                    return_step.gas,
                    gas_consumed,
                    call_step.gas_cost
                ),
            });
        }

        // Validate that child frame didn't consume more gas than allocated
        // This is determined by the gasCost minus what was returned
        let child_gas_limit = call_step.gas_cost;
        let gas_consumed_by_call = call_step.gas.saturating_sub(return_step.gas);

        // The gas consumed by the entire call should roughly equal gasCost
        // (there's some variance due to refunds, but it shouldn't exceed gasCost significantly)
        if gas_consumed_by_call > child_gas_limit + 100 {
            // Allow some tolerance for base costs
            result.add_warning(GasValidationWarning {
                tx_index: tx_idx,
                step_index: step_idx,
                warning_type: WarningType::FrameTransition,
                message: format!(
                    "Call consumed {} gas but gasCost was {} (diff: {})",
                    gas_consumed_by_call,
                    child_gas_limit,
                    gas_consumed_by_call.saturating_sub(child_gas_limit)
                ),
            });
        }
    }
}

/// Quick validation that just checks if there are any gas inconsistencies.
///
/// This is faster than full validation as it stops at the first error.
pub fn has_gas_errors(trace: &TraceFile) -> bool {
    let options = ValidationOptions {
        max_errors: Some(1),
        include_warnings: false,
        ..Default::default()
    };
    let result = validate_gas_with_options(trace, &options);
    !result.is_valid()
}

/// Get a summary of gas usage by opcode.
pub fn gas_usage_summary(trace: &TraceFile) -> GasUsageSummary {
    let mut opcode_gas: std::collections::HashMap<String, (u64, u64)> =
        std::collections::HashMap::new();
    let mut total_gas = 0u64;
    let mut total_steps = 0u64;

    for tx in &trace.transactions {
        for log in &tx.struct_logs {
            let entry = opcode_gas.entry(log.op_name.clone()).or_insert((0, 0));
            entry.0 += 1; // count
            entry.1 += log.gas_cost; // total gas
            total_gas += log.gas_cost;
            total_steps += 1;
        }
    }

    let mut breakdown: Vec<OpcodeGasUsage> = opcode_gas
        .into_iter()
        .map(|(name, (count, gas))| OpcodeGasUsage {
            opcode: name,
            count,
            total_gas: gas,
            avg_gas: if count > 0 {
                gas as f64 / count as f64
            } else {
                0.0
            },
            percentage: if total_gas > 0 {
                (gas as f64 / total_gas as f64) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    // Sort by total gas descending
    breakdown.sort_by(|a, b| b.total_gas.cmp(&a.total_gas));

    GasUsageSummary {
        total_steps,
        total_gas,
        avg_gas_per_step: if total_steps > 0 {
            total_gas as f64 / total_steps as f64
        } else {
            0.0
        },
        breakdown,
    }
}

/// Summary of gas usage across the trace.
#[derive(Debug, Clone, Serialize)]
pub struct GasUsageSummary {
    /// Total number of execution steps.
    pub total_steps: u64,
    /// Total gas used.
    pub total_gas: u64,
    /// Average gas per step.
    pub avg_gas_per_step: f64,
    /// Breakdown by opcode.
    pub breakdown: Vec<OpcodeGasUsage>,
}

/// Gas usage for a single opcode.
#[derive(Debug, Clone, Serialize)]
pub struct OpcodeGasUsage {
    /// Opcode name.
    pub opcode: String,
    /// Number of executions.
    pub count: u64,
    /// Total gas used.
    pub total_gas: u64,
    /// Average gas per execution.
    pub avg_gas: f64,
    /// Percentage of total gas.
    pub percentage: f64,
}

/// Result of validating a directory of trace files.
#[derive(Debug, Clone, Serialize)]
pub struct DirectoryValidationResult {
    /// Total number of files processed.
    pub files_processed: usize,
    /// Number of files that passed validation.
    pub files_passed: usize,
    /// Number of files that failed validation.
    pub files_failed: usize,
    /// Number of files that couldn't be parsed.
    pub files_errored: usize,
    /// Per-file validation results.
    pub file_results: Vec<FileValidationResult>,
}

impl DirectoryValidationResult {
    /// Create a new empty directory validation result.
    pub fn new() -> Self {
        Self {
            files_processed: 0,
            files_passed: 0,
            files_failed: 0,
            files_errored: 0,
            file_results: Vec::new(),
        }
    }

    /// Check if all files passed validation.
    pub fn all_valid(&self) -> bool {
        self.files_failed == 0 && self.files_errored == 0
    }

    /// Export to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get a summary string.
    pub fn summary(&self) -> String {
        format!(
            "Processed {} files: {} passed, {} failed, {} errors",
            self.files_processed, self.files_passed, self.files_failed, self.files_errored
        )
    }
}

impl Default for DirectoryValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result for a single file.
#[derive(Debug, Clone, Serialize)]
pub struct FileValidationResult {
    /// File path.
    pub file_path: String,
    /// File name (without path).
    pub file_name: String,
    /// Whether the file passed validation.
    pub valid: bool,
    /// Parse error if the file couldn't be loaded.
    pub parse_error: Option<String>,
    /// Gas validation result (if file was parsed successfully).
    pub gas_validation: Option<GasValidationResult>,
}

/// Validate all trace files in a directory.
///
/// # Arguments
///
/// * `dir_path` - Path to the directory containing JSONL trace files.
/// * `options` - Validation options to apply to each file.
///
/// # Returns
///
/// Returns a `DirectoryValidationResult` with per-file results.
///
/// # Example
///
/// ```ignore
/// use eip3155_tracer::validation::{validate_directory, ValidationOptions};
/// use std::path::Path;
///
/// let result = validate_directory(
///     Path::new("./traces"),
///     &ValidationOptions::default(),
/// )?;
///
/// println!("{}", result.summary());
///
/// for file_result in &result.file_results {
///     if !file_result.valid {
///         println!("FAILED: {}", file_result.file_name);
///     }
/// }
/// ```
pub fn validate_directory(
    dir_path: &std::path::Path,
    options: &ValidationOptions,
) -> Result<DirectoryValidationResult, std::io::Error> {
    use std::fs;

    let mut result = DirectoryValidationResult::new();

    // Read directory entries
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Skip non-files and non-JSONL files
        if !path.is_file() {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        // Only process .jsonl files
        if !file_name.ends_with(".jsonl") {
            continue;
        }

        result.files_processed += 1;

        let file_path = path.to_string_lossy().to_string();

        // Try to load and validate the file
        match TraceFile::load(&path) {
            Ok(trace) => {
                let gas_result = validate_gas_with_options(&trace, options);
                let valid = gas_result.is_valid();

                if valid {
                    result.files_passed += 1;
                } else {
                    result.files_failed += 1;
                }

                result.file_results.push(FileValidationResult {
                    file_path,
                    file_name,
                    valid,
                    parse_error: None,
                    gas_validation: Some(gas_result),
                });
            }
            Err(e) => {
                result.files_errored += 1;
                result.file_results.push(FileValidationResult {
                    file_path,
                    file_name,
                    valid: false,
                    parse_error: Some(e.to_string()),
                    gas_validation: None,
                });
            }
        }
    }

    // Sort results by file name for consistent output
    result
        .file_results
        .sort_by(|a, b| a.file_name.cmp(&b.file_name));

    Ok(result)
}

/// Validate all trace files in a directory with default options.
pub fn validate_directory_default(
    dir_path: &std::path::Path,
) -> Result<DirectoryValidationResult, std::io::Error> {
    validate_directory(dir_path, &ValidationOptions::default())
}

/// Validate a directory and return only failed files.
pub fn get_failed_files(
    dir_path: &std::path::Path,
    options: &ValidationOptions,
) -> Result<Vec<FileValidationResult>, std::io::Error> {
    let result = validate_directory(dir_path, options)?;
    Ok(result
        .file_results
        .into_iter()
        .filter(|f| !f.valid)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_default() {
        let result = GasValidationResult::new();
        assert!(result.is_valid());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_add_error() {
        let mut result = GasValidationResult::new();
        result.add_error(GasValidationError {
            tx_index: 0,
            step_index: 0,
            pc: 0,
            opcode: "ADD".to_string(),
            expected_gas_cost: 3,
            reported_gas_cost: 5,
            gas_before: 100,
            gas_after: 97,
            message: "Test error".to_string(),
        });
        assert!(!result.is_valid());
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn test_directory_validation_result() {
        let result = DirectoryValidationResult::new();
        assert!(result.all_valid());
        assert_eq!(result.files_processed, 0);
    }
}
