#!/usr/bin/env python3
"""
Script to compare metrics between SP1 and RISC0 proving systems.
Compares proving_time_ms, proof_size, and memory usage.
"""

import json
import os
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Tuple
import statistics

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

def extract_metrics(metrics_data: Dict) -> Dict[str, float]:
    """Extract all relevant metrics from data."""
    try:
        proving_data = metrics_data["proving"]["success"]
        return {
            "proving_time_s": float(proving_data["proving_time_ms"]) / 1000.0,
            "proving_time_ms": float(proving_data["proving_time_ms"]),
            "proof_size_kb": float(proving_data["proof_size"]) / 1024.0,
            "proof_size_bytes": float(proving_data["proof_size"]),
            "peak_memory_gb": float(proving_data["peak_memory_usage_bytes"]) / (1024**3),
            "peak_memory_bytes": float(proving_data["peak_memory_usage_bytes"]),
        }
    except KeyError:
        return None

def compare_metrics(risc0_metrics: Dict[str, Dict], sp1_metrics: Dict[str, Dict]) -> Dict:
    """Compare metrics between RISC0 and SP1."""
    comparison_data = {
        "common_tests": [],
        "risc0_only": [],
        "sp1_only": [],
        "detailed_comparisons": []
    }
    
    # Find common tests and unique tests
    risc0_tests = set(risc0_metrics.keys())
    sp1_tests = set(sp1_metrics.keys())
    common_tests = risc0_tests & sp1_tests
    
    comparison_data["risc0_only"] = sorted(list(risc0_tests - sp1_tests))
    comparison_data["sp1_only"] = sorted(list(sp1_tests - common_tests))
    
    # Compare common tests
    for test_name in sorted(common_tests):
        risc0_data = extract_metrics(risc0_metrics[test_name])
        sp1_data = extract_metrics(sp1_metrics[test_name])
        
        if risc0_data and sp1_data:
            comparison = {
                "test_name": test_name,
                "risc0": risc0_data,
                "sp1": sp1_data,
                "speedup": risc0_data["proving_time_s"] / sp1_data["proving_time_s"],
                "proof_size_ratio": risc0_data["proof_size_kb"] / sp1_data["proof_size_kb"],
                "memory_ratio": risc0_data["peak_memory_gb"] / sp1_data["peak_memory_gb"],
            }
            comparison_data["detailed_comparisons"].append(comparison)
            comparison_data["common_tests"].append(test_name)
    
    return comparison_data

def print_summary_statistics(comparison_data: Dict):
    """Print summary statistics."""
    comparisons = comparison_data["detailed_comparisons"]
    
    if not comparisons:
        print("No common tests to compare")
        return
    
    print("\n" + "="*80)
    print("SUMMARY STATISTICS")
    print("="*80)
    
    # Proving Time Analysis
    speedups = [c["speedup"] for c in comparisons]
    avg_speedup = statistics.mean(speedups)
    median_speedup = statistics.median(speedups)
    
    print(f"\n📊 PROVING TIME (Lower is better, speedup = RISC0 time / SP1 time):")
    print(f"  Average speedup: {avg_speedup:.2f}x")
    print(f"  Median speedup: {median_speedup:.2f}x")
    print(f"  Min speedup: {min(speedups):.2f}x")
    print(f"  Max speedup: {max(speedups):.2f}x")
    
    if avg_speedup > 1.0:
        print(f"  → SP1 is {avg_speedup:.2f}x FASTER on average")
    else:
        print(f"  → RISC0 is {1/avg_speedup:.2f}x FASTER on average")
    
    # Total time comparison
    total_risc0_time = sum(c["risc0"]["proving_time_s"] for c in comparisons)
    total_sp1_time = sum(c["sp1"]["proving_time_s"] for c in comparisons)
    print(f"\n  Total proving time (all common tests):")
    print(f"    RISC0: {total_risc0_time:,.0f} seconds ({total_risc0_time/3600:.1f} hours)")
    print(f"    SP1:   {total_sp1_time:,.0f} seconds ({total_sp1_time/3600:.1f} hours)")
    print(f"    Time difference: {abs(total_risc0_time - total_sp1_time):,.0f} seconds")
    
    # Proof Size Analysis
    proof_ratios = [c["proof_size_ratio"] for c in comparisons]
    avg_proof_ratio = statistics.mean(proof_ratios)
    
    print(f"\n📦 PROOF SIZE (Lower is better, ratio = RISC0 size / SP1 size):")
    print(f"  Average ratio: {avg_proof_ratio:.2f}x")
    print(f"  Median ratio: {statistics.median(proof_ratios):.2f}x")
    
    if avg_proof_ratio > 1.0:
        print(f"  → SP1 proofs are {avg_proof_ratio:.2f}x SMALLER on average")
    else:
        print(f"  → RISC0 proofs are {1/avg_proof_ratio:.2f}x SMALLER on average")
    
    # Memory Usage Analysis
    memory_ratios = [c["memory_ratio"] for c in comparisons]
    avg_memory_ratio = statistics.mean(memory_ratios)
    
    print(f"\n💾 MEMORY USAGE (Lower is better, ratio = RISC0 memory / SP1 memory):")
    print(f"  Average ratio: {avg_memory_ratio:.2f}x")
    print(f"  Median ratio: {statistics.median(memory_ratios):.2f}x")
    
    if avg_memory_ratio > 1.0:
        print(f"  → SP1 uses {avg_memory_ratio:.2f}x LESS memory on average")
    else:
        print(f"  → RISC0 uses {1/avg_memory_ratio:.2f}x LESS memory on average")

def print_detailed_comparison_table(comparison_data: Dict, limit: int = None):
    """Print detailed comparison table."""
    comparisons = comparison_data["detailed_comparisons"]
    
    if not comparisons:
        return
    
    print("\n" + "="*80)
    print("DETAILED COMPARISON (Common Tests)")
    print("="*80)
    
    # Sort by speedup
    sorted_comparisons = sorted(comparisons, key=lambda x: x["speedup"], reverse=True)
    
    if limit:
        sorted_comparisons = sorted_comparisons[:limit]
    
    # Print header
    print(f"\n{'Test Name':<50} {'RISC0 Time':<15} {'SP1 Time':<15} {'Speedup':<10}")
    print(f"{'':50} {'(seconds)':<15} {'(seconds)':<15} {'(R/S)':<10}")
    print("-" * 100)
    
    for comp in sorted_comparisons:
        test_name = comp["test_name"]
        if len(test_name) > 47:
            test_name = test_name[:44] + "..."
        
        risc0_time = comp["risc0"]["proving_time_s"]
        sp1_time = comp["sp1"]["proving_time_s"]
        speedup = comp["speedup"]
        
        print(f"{test_name:<50} {risc0_time:>12,.0f}   {sp1_time:>12,.0f}   {speedup:>8.2f}x")

def print_top_performers(comparison_data: Dict, top_n: int = 10):
    """Print top performers for each metric."""
    comparisons = comparison_data["detailed_comparisons"]
    
    if not comparisons:
        return
    
    print("\n" + "="*80)
    print(f"TOP {top_n} BEST SPEEDUPS (Where SP1 is Fastest)")
    print("="*80)
    
    sorted_by_speedup = sorted(comparisons, key=lambda x: x["speedup"], reverse=True)
    
    for i, comp in enumerate(sorted_by_speedup[:top_n], 1):
        test_name = comp["test_name"]
        risc0_time = comp["risc0"]["proving_time_s"]
        sp1_time = comp["sp1"]["proving_time_s"]
        speedup = comp["speedup"]
        time_saved = risc0_time - sp1_time
        
        print(f"\n{i}. {test_name}")
        print(f"   Speedup: {speedup:.2f}x")
        print(f"   RISC0: {risc0_time:,.0f}s | SP1: {sp1_time:,.0f}s | Time saved: {time_saved:,.0f}s")
    
    print("\n" + "="*80)
    print(f"TOP {top_n} WORST SPEEDUPS (Where RISC0 is Fastest or SP1 is Slowest)")
    print("="*80)
    
    for i, comp in enumerate(sorted_by_speedup[-top_n:][::-1], 1):
        test_name = comp["test_name"]
        risc0_time = comp["risc0"]["proving_time_s"]
        sp1_time = comp["sp1"]["proving_time_s"]
        speedup = comp["speedup"]
        time_saved = sp1_time - risc0_time
        
        print(f"\n{i}. {test_name}")
        print(f"   Speedup: {speedup:.2f}x")
        print(f"   RISC0: {risc0_time:,.0f}s | SP1: {sp1_time:,.0f}s | Time saved: {time_saved:,.0f}s")

def print_test_coverage(comparison_data: Dict):
    """Print test coverage information."""
    print("\n" + "="*80)
    print("TEST COVERAGE")
    print("="*80)
    
    print(f"\nCommon tests: {len(comparison_data['common_tests'])}")
    print(f"RISC0 only: {len(comparison_data['risc0_only'])}")
    print(f"SP1 only: {len(comparison_data['sp1_only'])}")
    
    if comparison_data['risc0_only']:
        print(f"\nSample of RISC0-only tests (first 5):")
        for test in comparison_data['risc0_only'][:5]:
            print(f"  - {test}")
        if len(comparison_data['risc0_only']) > 5:
            print(f"  ... and {len(comparison_data['risc0_only']) - 5} more")
    
    if comparison_data['sp1_only']:
        print(f"\nSample of SP1-only tests (first 5):")
        for test in comparison_data['sp1_only'][:5]:
            print(f"  - {test}")
        if len(comparison_data['sp1_only']) > 5:
            print(f"  ... and {len(comparison_data['sp1_only']) - 5} more")

def generate_markdown_summary(comparison_data: Dict, risc0_folder: str, sp1_folder: str) -> str:
    """Generate a markdown-formatted summary of the comparison."""
    comparisons = comparison_data["detailed_comparisons"]
    
    if not comparisons:
        return "# SP1 vs RISC0 Comparison\n\nNo common tests to compare."
    
    # Calculate statistics
    speedups = [c["speedup"] for c in comparisons]
    proof_ratios = [c["proof_size_ratio"] for c in comparisons]
    memory_ratios = [c["memory_ratio"] for c in comparisons]
    
    avg_speedup = statistics.mean(speedups)
    median_speedup = statistics.median(speedups)
    avg_proof_ratio = statistics.mean(proof_ratios)
    avg_memory_ratio = statistics.mean(memory_ratios)
    
    total_risc0_time = sum(c["risc0"]["proving_time_s"] for c in comparisons)
    total_sp1_time = sum(c["sp1"]["proving_time_s"] for c in comparisons)
    
    # Build markdown
    md = []
    md.append("# SP1 vs RISC0 Metrics Comparison Summary")
    md.append("")
    md.append(f"**RISC0 Folder:** `{risc0_folder}`  ")
    md.append(f"**SP1 Folder:** `{sp1_folder}`  ")
    md.append(f"**Common Tests:** {len(comparison_data['common_tests'])}")
    md.append("")
    md.append("---")
    md.append("")
    
    # Executive Summary
    md.append("## Executive Summary")
    md.append("")
    md.append("| Metric | Winner | Performance Advantage |")
    md.append("|--------|--------|----------------------|")
    
    # Proving speed
    if avg_speedup > 1.0:
        md.append(f"| **Proving Speed** | ✅ **SP1** | **{avg_speedup:.2f}x faster** on average |")
    else:
        md.append(f"| **Proving Speed** | ✅ **RISC0** | **{1/avg_speedup:.2f}x faster** on average |")
    
    # Proof size
    if avg_proof_ratio > 1.0:
        md.append(f"| **Proof Size** | ✅ **SP1** | **{avg_proof_ratio:.2f}x smaller** proofs |")
    else:
        md.append(f"| **Proof Size** | ✅ **RISC0** | **{1/avg_proof_ratio:.2f}x smaller** proofs |")
    
    # Memory usage
    if avg_memory_ratio > 1.0:
        md.append(f"| **Memory Usage** | ✅ **SP1** | **{avg_memory_ratio:.2f}x less** memory |")
    else:
        md.append(f"| **Memory Usage** | ✅ **RISC0** | **{1/avg_memory_ratio:.2f}x less** memory |")
    
    md.append("")
    md.append("---")
    md.append("")
    
    # Detailed Performance Analysis
    md.append("## Detailed Performance Analysis")
    md.append("")
    md.append("### 🚀 Proving Time Performance")
    md.append("")
    if avg_speedup > 1.0:
        md.append(f"**SP1 is significantly faster at proving:**")
    else:
        md.append(f"**RISC0 is significantly faster at proving:**")
    md.append("")
    md.append(f"- **Average speedup:** {avg_speedup:.2f}x")
    md.append(f"- **Median speedup:** {median_speedup:.2f}x")
    md.append(f"- **Range:** {min(speedups):.2f}x to {max(speedups):.2f}x")
    md.append("")
    md.append("**Time Comparison:**")
    md.append(f"- **Total RISC0 time:** {total_risc0_time:,.0f} seconds ({total_risc0_time/3600:.1f} hours)")
    md.append(f"- **Total SP1 time:** {total_sp1_time:,.0f} seconds ({total_sp1_time/3600:.1f} hours)")
    md.append(f"- **Time difference:** {abs(total_risc0_time - total_sp1_time):,.0f} seconds ({abs(total_risc0_time - total_sp1_time)/3600:.1f} hours)")
    md.append("")
    
    # Proof Size Analysis
    md.append("### 📦 Proof Size Analysis")
    md.append("")
    if avg_proof_ratio > 1.0:
        md.append(f"**SP1 generates smaller proofs:**")
    else:
        md.append(f"**RISC0 generates smaller proofs:**")
    md.append("")
    md.append(f"- **Average ratio:** {avg_proof_ratio:.2f}x")
    md.append(f"- **Median ratio:** {statistics.median(proof_ratios):.2f}x")
    md.append("")
    
    # Memory Usage Analysis
    md.append("### 💾 Memory Usage Analysis")
    md.append("")
    if avg_memory_ratio > 1.0:
        md.append(f"**SP1 is more memory efficient:**")
    else:
        md.append(f"**RISC0 is more memory efficient:**")
    md.append("")
    md.append(f"- **Average ratio:** {avg_memory_ratio:.2f}x")
    md.append(f"- **Median ratio:** {statistics.median(memory_ratios):.2f}x")
    md.append("")
    md.append("---")
    md.append("")
    
    # Top Performers
    md.append("## Top Performance Winners")
    md.append("")
    
    sorted_by_speedup = sorted(comparisons, key=lambda x: x["speedup"], reverse=True)
    
    md.append("### 🏆 Top 10 Tests Where SP1 Dominates (Fastest Proving)")
    md.append("")
    for i, comp in enumerate(sorted_by_speedup[:10], 1):
        test_name = comp["test_name"]
        risc0_time = comp["risc0"]["proving_time_s"]
        sp1_time = comp["sp1"]["proving_time_s"]
        speedup = comp["speedup"]
        time_saved = risc0_time - sp1_time
        
        md.append(f"{i}. **{test_name}**: **{speedup:.2f}x faster**, saved {time_saved:,.0f}s")
    
    md.append("")
    md.append("### 🏆 Top 10 Tests Where RISC0 Dominates (Fastest Proving)")
    md.append("")
    for i, comp in enumerate(sorted_by_speedup[-10:][::-1], 1):
        test_name = comp["test_name"]
        risc0_time = comp["risc0"]["proving_time_s"]
        sp1_time = comp["sp1"]["proving_time_s"]
        speedup = comp["speedup"]
        time_saved = sp1_time - risc0_time
        
        md.append(f"{i}. **{test_name}**: **{1/speedup:.2f}x faster** than SP1, saved {time_saved:,.0f}s")
    
    md.append("")
    md.append("---")
    md.append("")
    
    # Test Coverage
    md.append("## Test Coverage")
    md.append("")
    md.append("| Category | Count | Notes |")
    md.append("|----------|-------|-------|")
    md.append(f"| **Common tests** | {len(comparison_data['common_tests'])} | Tests executed by both systems |")
    md.append(f"| **RISC0 only** | {len(comparison_data['risc0_only'])} | Tests only in RISC0 |")
    md.append(f"| **SP1 only** | {len(comparison_data['sp1_only'])} | Tests only in SP1 |")
    md.append("")
    
    return "\n".join(md)

def main():
    """Main function."""
    parser = argparse.ArgumentParser(
        description="Compare metrics between SP1 and RISC0 proving systems."
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
        help="Output file for markdown summary (optional, prints to stdout if not specified)"
    )
    parser.add_argument(
        "--format",
        type=str,
        choices=["text", "markdown"],
        default="text",
        help="Output format: text (default) or markdown"
    )
    parser.add_argument(
        "--top-n",
        type=int,
        default=10,
        help="Number of top performers to show (default: 10)"
    )
    
    args = parser.parse_args()
    
    if args.format == "text":
        print("="*80)
        print("SP1 vs RISC0 METRICS COMPARISON")
        print("="*80)
    
    if args.format == "text":
        print(f"\nLoading RISC0 metrics from: {args.risc0_folder}")
    risc0_metrics = load_metrics(args.risc0_folder)
    if args.format == "text":
        print(f"Loaded {len(risc0_metrics)} RISC0 test results")
    
    if args.format == "text":
        print(f"\nLoading SP1 metrics from: {args.sp1_folder}")
    sp1_metrics = load_metrics(args.sp1_folder)
    if args.format == "text":
        print(f"Loaded {len(sp1_metrics)} SP1 test results")
    
    if args.format == "text":
        print("\nComparing metrics...")
    comparison_data = compare_metrics(risc0_metrics, sp1_metrics)
    
    if args.format == "markdown":
        markdown_output = generate_markdown_summary(comparison_data, args.risc0_folder, args.sp1_folder)
        
        if args.output:
            with open(args.output, 'w') as f:
                f.write(markdown_output)
            print(f"✅ Markdown summary saved to: {args.output}")
        else:
            print(markdown_output)
    else:
        # Text format output
        print_test_coverage(comparison_data)
        print_summary_statistics(comparison_data)
        print_top_performers(comparison_data, top_n=args.top_n)
        print_detailed_comparison_table(comparison_data, limit=20)
        
        print("\n" + "="*80)
        print("COMPARISON COMPLETE")
        print("="*80)

if __name__ == "__main__":
    main()

