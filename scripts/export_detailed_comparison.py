#!/usr/bin/env python3
"""
Script to export detailed per-test comparison between SP1 and RISC0 to CSV.
"""

import json
import csv
import argparse
from pathlib import Path
from typing import Dict, List

def load_metrics(folder_path: str) -> Dict[str, Dict]:
    """Load all metric files from a folder, keyed by test name."""
    metrics = {}
    folder = Path(folder_path)
    
    if not folder.exists():
        print(f"Warning: {folder} does not exist")
        return metrics
    
    # Find all subfolders that contain JSON files
    for subfolder in folder.iterdir():
        if subfolder.is_dir():
            for file_path in subfolder.glob("*.json"):
                try:
                    with open(file_path, 'r') as f:
                        data = json.load(f)
                        # Use the test name from the file as the key
                        test_name = file_path.stem
                        metrics[test_name] = data
                except (json.JSONDecodeError, FileNotFoundError) as e:
                    print(f"Error loading {file_path}: {e}")
    
    return metrics

def extract_metrics(metrics_data: Dict) -> Dict[str, any]:
    """Extract all relevant metrics from data."""
    try:
        proving_data = metrics_data["proving"]["success"]
        return {
            "proving_time_ms": float(proving_data["proving_time_ms"]),
            "proving_time_s": float(proving_data["proving_time_ms"]) / 1000.0,
            "proof_size_bytes": float(proving_data["proof_size"]),
            "proof_size_kb": float(proving_data["proof_size"]) / 1024.0,
            "peak_memory_bytes": float(proving_data["peak_memory_usage_bytes"]),
            "peak_memory_gb": float(proving_data["peak_memory_usage_bytes"]) / (1024**3),
        }
    except KeyError:
        return None

def export_detailed_csv(risc0_metrics: Dict, sp1_metrics: Dict, output_file: str):
    """Export detailed comparison to CSV."""
    
    # Find common tests
    risc0_tests = set(risc0_metrics.keys())
    sp1_tests = set(sp1_metrics.keys())
    common_tests = sorted(risc0_tests & sp1_tests)
    
    print(f"Found {len(common_tests)} common tests")
    print(f"RISC0 only: {len(risc0_tests - sp1_tests)}")
    print(f"SP1 only: {len(sp1_tests - risc0_tests)}")
    
    # Prepare CSV data
    rows = []
    for test_name in common_tests:
        risc0_data = extract_metrics(risc0_metrics[test_name])
        sp1_data = extract_metrics(sp1_metrics[test_name])
        
        if risc0_data and sp1_data:
            row = {
                "test_name": test_name,
                # RISC0 metrics
                "risc0_proving_time_s": risc0_data["proving_time_s"],
                "risc0_proving_time_ms": risc0_data["proving_time_ms"],
                "risc0_proof_size_kb": risc0_data["proof_size_kb"],
                "risc0_proof_size_bytes": risc0_data["proof_size_bytes"],
                "risc0_peak_memory_gb": risc0_data["peak_memory_gb"],
                "risc0_peak_memory_bytes": risc0_data["peak_memory_bytes"],
                # SP1 metrics
                "sp1_proving_time_s": sp1_data["proving_time_s"],
                "sp1_proving_time_ms": sp1_data["proving_time_ms"],
                "sp1_proof_size_kb": sp1_data["proof_size_kb"],
                "sp1_proof_size_bytes": sp1_data["proof_size_bytes"],
                "sp1_peak_memory_gb": sp1_data["peak_memory_gb"],
                "sp1_peak_memory_bytes": sp1_data["peak_memory_bytes"],
                # Comparisons
                "time_speedup": risc0_data["proving_time_s"] / sp1_data["proving_time_s"],
                "time_diff_s": risc0_data["proving_time_s"] - sp1_data["proving_time_s"],
                "proof_size_ratio": risc0_data["proof_size_kb"] / sp1_data["proof_size_kb"],
                "proof_size_diff_kb": risc0_data["proof_size_kb"] - sp1_data["proof_size_kb"],
                "memory_ratio": risc0_data["peak_memory_gb"] / sp1_data["peak_memory_gb"],
                "memory_diff_gb": risc0_data["peak_memory_gb"] - sp1_data["peak_memory_gb"],
                # Winner indicators
                "time_winner": "RISC0" if risc0_data["proving_time_s"] < sp1_data["proving_time_s"] else "SP1",
                "proof_size_winner": "RISC0" if risc0_data["proof_size_kb"] < sp1_data["proof_size_kb"] else "SP1",
                "memory_winner": "RISC0" if risc0_data["peak_memory_gb"] < sp1_data["peak_memory_gb"] else "SP1",
            }
            rows.append(row)
    
    # Write to CSV
    if rows:
        fieldnames = rows[0].keys()
        with open(output_file, 'w', newline='') as csvfile:
            writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
            writer.writeheader()
            writer.writerows(rows)
        
        print(f"\n✅ Exported {len(rows)} test comparisons to: {output_file}")
        
        # Print summary statistics
        print("\n" + "="*80)
        print("SUMMARY")
        print("="*80)
        
        risc0_wins = sum(1 for r in rows if r["time_winner"] == "RISC0")
        sp1_wins = sum(1 for r in rows if r["time_winner"] == "SP1")
        
        print(f"\nProving Time Winners:")
        print(f"  RISC0: {risc0_wins} tests ({risc0_wins/len(rows)*100:.1f}%)")
        print(f"  SP1:   {sp1_wins} tests ({sp1_wins/len(rows)*100:.1f}%)")
        
        total_risc0_time = sum(r["risc0_proving_time_s"] for r in rows)
        total_sp1_time = sum(r["sp1_proving_time_s"] for r in rows)
        
        print(f"\nTotal Proving Time:")
        print(f"  RISC0: {total_risc0_time:,.0f}s ({total_risc0_time/3600:.1f} hours)")
        print(f"  SP1:   {total_sp1_time:,.0f}s ({total_sp1_time/3600:.1f} hours)")
        print(f"  Overall speedup: {total_risc0_time/total_sp1_time:.2f}x (RISC0/SP1)")
        
        risc0_proof_wins = sum(1 for r in rows if r["proof_size_winner"] == "RISC0")
        sp1_proof_wins = sum(1 for r in rows if r["proof_size_winner"] == "SP1")
        
        print(f"\nProof Size Winners:")
        print(f"  RISC0: {risc0_proof_wins} tests ({risc0_proof_wins/len(rows)*100:.1f}%)")
        print(f"  SP1:   {sp1_proof_wins} tests ({sp1_proof_wins/len(rows)*100:.1f}%)")
        
        risc0_mem_wins = sum(1 for r in rows if r["memory_winner"] == "RISC0")
        sp1_mem_wins = sum(1 for r in rows if r["memory_winner"] == "SP1")
        
        print(f"\nMemory Usage Winners:")
        print(f"  RISC0: {risc0_mem_wins} tests ({risc0_mem_wins/len(rows)*100:.1f}%)")
        print(f"  SP1:   {sp1_mem_wins} tests ({sp1_mem_wins/len(rows)*100:.1f}%)")
    else:
        print("No common tests found to export")

def main():
    parser = argparse.ArgumentParser(
        description="Export detailed per-test comparison between SP1 and RISC0 to CSV."
    )
    parser.add_argument(
        "--risc0-folder",
        type=str,
        required=True,
        help="Path to the folder containing RISC0 metrics"
    )
    parser.add_argument(
        "--sp1-folder",
        type=str,
        required=True,
        help="Path to the folder containing SP1 metrics"
    )
    parser.add_argument(
        "--output",
        type=str,
        default="detailed_comparison.csv",
        help="Output CSV file (default: detailed_comparison.csv)"
    )
    
    args = parser.parse_args()
    
    print("="*80)
    print("DETAILED COMPARISON EXPORT")
    print("="*80)
    
    print(f"\nLoading RISC0 metrics from: {args.risc0_folder}")
    risc0_metrics = load_metrics(args.risc0_folder)
    print(f"Loaded {len(risc0_metrics)} RISC0 test results")
    
    print(f"\nLoading SP1 metrics from: {args.sp1_folder}")
    sp1_metrics = load_metrics(args.sp1_folder)
    print(f"Loaded {len(sp1_metrics)} SP1 test results")
    
    print("\nGenerating detailed comparison CSV...")
    export_detailed_csv(risc0_metrics, sp1_metrics, args.output)
    
    print("\n" + "="*80)
    print("EXPORT COMPLETE")
    print("="*80)

if __name__ == "__main__":
    main()

