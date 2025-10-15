#!/usr/bin/env python3
"""
Export SP1 vs RISC0 comparison to CSV for further analysis.
"""

import json
import csv
from pathlib import Path

def load_metrics(folder_path: str) -> dict:
    """Load all metric files from a folder, keyed by test name."""
    metrics = {}
    folder = Path(folder_path)
    
    for subfolder in folder.iterdir():
        if subfolder.is_dir():
            for file_path in subfolder.glob("*.json"):
                try:
                    with open(file_path, 'r') as f:
                        data = json.load(f)
                        test_name = file_path.stem
                        metrics[test_name] = data
                except (json.JSONDecodeError, FileNotFoundError) as e:
                    print(f"Error loading {file_path}: {e}")
    
    return metrics

def extract_metrics(metrics_data: dict) -> dict:
    """Extract all relevant metrics from data."""
    try:
        proving_data = metrics_data["proving"]["success"]
        return {
            "proving_time_s": float(proving_data["proving_time_ms"]) / 1000.0,
            "proof_size_kb": float(proving_data["proof_size"]) / 1024.0,
            "peak_memory_gb": float(proving_data["peak_memory_usage_bytes"]) / (1024**3),
        }
    except KeyError:
        return None

def main():
    risc0_folder = "/Users/wisemrmusa/code/projects/nethermind/zke/projects/new-profiling/zkevm-benchmark-workload/zkevm-metrics-risc0-1M"
    sp1_folder = "/Users/wisemrmusa/code/projects/nethermind/zke/projects/new-profiling/zkevm-benchmark-workload/zkevm-metrics-sp1-1M"
    
    print("Loading metrics...")
    risc0_metrics = load_metrics(risc0_folder)
    sp1_metrics = load_metrics(sp1_folder)
    
    # Find common tests
    common_tests = set(risc0_metrics.keys()) & set(sp1_metrics.keys())
    
    print(f"Exporting {len(common_tests)} common tests to CSV...")
    
    # Export to CSV
    output_file = "/Users/wisemrmusa/code/projects/nethermind/zke/projects/new-profiling/zkevm-benchmark-workload/sp1_vs_risc0_comparison.csv"
    
    with open(output_file, 'w', newline='') as csvfile:
        fieldnames = [
            'test_name',
            'risc0_proving_time_s', 'sp1_proving_time_s', 'speedup',
            'risc0_proof_size_kb', 'sp1_proof_size_kb', 'proof_size_ratio',
            'risc0_peak_memory_gb', 'sp1_peak_memory_gb', 'memory_ratio'
        ]
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        
        writer.writeheader()
        
        for test_name in sorted(common_tests):
            risc0_data = extract_metrics(risc0_metrics[test_name])
            sp1_data = extract_metrics(sp1_metrics[test_name])
            
            if risc0_data and sp1_data:
                row = {
                    'test_name': test_name,
                    'risc0_proving_time_s': f"{risc0_data['proving_time_s']:.2f}",
                    'sp1_proving_time_s': f"{sp1_data['proving_time_s']:.2f}",
                    'speedup': f"{risc0_data['proving_time_s'] / sp1_data['proving_time_s']:.2f}",
                    'risc0_proof_size_kb': f"{risc0_data['proof_size_kb']:.2f}",
                    'sp1_proof_size_kb': f"{sp1_data['proof_size_kb']:.2f}",
                    'proof_size_ratio': f"{risc0_data['proof_size_kb'] / sp1_data['proof_size_kb']:.2f}",
                    'risc0_peak_memory_gb': f"{risc0_data['peak_memory_gb']:.2f}",
                    'sp1_peak_memory_gb': f"{sp1_data['peak_memory_gb']:.2f}",
                    'memory_ratio': f"{risc0_data['peak_memory_gb'] / sp1_data['peak_memory_gb']:.2f}",
                }
                writer.writerow(row)
    
    print(f"✅ Exported to: {output_file}")

if __name__ == "__main__":
    main()

