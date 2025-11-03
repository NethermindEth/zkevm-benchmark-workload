#!/usr/bin/env python3
"""
Script to export proving time and memory speedup comparison to markdown.
"""

import json
import argparse
import sys
from pathlib import Path
from typing import Dict
import statistics

# Add scripts directory to path for imports
sys.path.insert(0, str(Path(__file__).parent))

try:
    from test_name_formatter import TestNameFormatter
    FORMATTER_AVAILABLE = True
except ImportError:
    FORMATTER_AVAILABLE = False
    print("Warning: test_name_formatter not available, using original names")

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
            "proving_time_s": float(proving_data["proving_time_ms"]) / 1000.0,
            "peak_memory_gb": float(proving_data["peak_memory_usage_bytes"]) / (1024**3),
        }
    except KeyError:
        return None

def generate_speedup_markdown(risc0_metrics: Dict, sp1_metrics: Dict, output_file: str, use_display_names: bool = True):
    """Generate markdown with proving time and memory speedup comparison."""
    
    # Initialize formatter if requested
    formatter = None
    if use_display_names and FORMATTER_AVAILABLE:
        formatter = TestNameFormatter()
        print("Using display name formatter for test names")
    
    # Find common tests
    risc0_tests = set(risc0_metrics.keys())
    sp1_tests = set(sp1_metrics.keys())
    common_tests = sorted(risc0_tests & sp1_tests)
    
    print(f"Found {len(common_tests)} common tests")
    
    # Prepare comparison data
    comparisons = []
    for test_name in common_tests:
        risc0_data = extract_metrics(risc0_metrics[test_name])
        sp1_data = extract_metrics(sp1_metrics[test_name])
        
        if risc0_data and sp1_data:
            time_speedup = risc0_data["proving_time_s"] / sp1_data["proving_time_s"]
            memory_ratio = risc0_data["peak_memory_gb"] / sp1_data["peak_memory_gb"]
            
            comparisons.append({
                "test_name": test_name,
                "risc0_time": risc0_data["proving_time_s"],
                "sp1_time": sp1_data["proving_time_s"],
                "time_speedup": time_speedup,
                "risc0_memory": risc0_data["peak_memory_gb"],
                "sp1_memory": sp1_data["peak_memory_gb"],
                "memory_ratio": memory_ratio,
            })
    
    # Calculate statistics
    time_speedups = [c["time_speedup"] for c in comparisons]
    memory_ratios = [c["memory_ratio"] for c in comparisons]
    
    avg_time_speedup = statistics.mean(time_speedups)
    median_time_speedup = statistics.median(time_speedups)
    avg_memory_ratio = statistics.mean(memory_ratios)
    median_memory_ratio = statistics.median(memory_ratios)
    
    total_risc0_time = sum(c["risc0_time"] for c in comparisons)
    total_sp1_time = sum(c["sp1_time"] for c in comparisons)
    
    # Generate markdown
    md = []
    md.append("# RISC0 vs SP1 Performance Comparison")
    md.append("")
    md.append(f"**Comparing:** `zkevm-metrics-risc0-1M-1` vs `zkevm-metrics-sp1-1M`")
    md.append("")
    md.append(f"**Total Tests Compared:** {len(comparisons)}")
    md.append("")
    md.append("---")
    md.append("")
    
    # Summary Statistics
    md.append("## Summary Statistics")
    md.append("")
    md.append("### ⏱️ Proving Time Performance")
    md.append("")
    md.append("| Metric | Value |")
    md.append("|--------|-------|")
    md.append(f"| **Average Speedup** (RISC0/SP1) | **{avg_time_speedup:.3f}x** |")
    md.append(f"| **Median Speedup** | **{median_time_speedup:.3f}x** |")
    md.append(f"| **Min Speedup** | {min(time_speedups):.3f}x |")
    md.append(f"| **Max Speedup** | {max(time_speedups):.3f}x |")
    md.append(f"| **Total RISC0 Time** | {total_risc0_time:,.0f}s ({total_risc0_time/3600:.1f} hours) |")
    md.append(f"| **Total SP1 Time** | {total_sp1_time:,.0f}s ({total_sp1_time/3600:.1f} hours) |")
    md.append(f"| **Time Saved** | {total_sp1_time - total_risc0_time:,.0f}s ({(total_sp1_time - total_risc0_time)/3600:.1f} hours) |")
    md.append("")
    
    if avg_time_speedup < 1.0:
        md.append(f"✅ **RISC0 is {1/avg_time_speedup:.2f}x faster** on average")
    else:
        md.append(f"✅ **SP1 is {avg_time_speedup:.2f}x faster** on average")
    md.append("")
    
    md.append("### 💾 Memory Usage")
    md.append("")
    md.append("| Metric | Value |")
    md.append("|--------|-------|")
    md.append(f"| **Average Memory Ratio** (RISC0/SP1) | **{avg_memory_ratio:.3f}x** |")
    md.append(f"| **Median Memory Ratio** | **{median_memory_ratio:.3f}x** |")
    md.append(f"| **Min Memory Ratio** | {min(memory_ratios):.3f}x |")
    md.append(f"| **Max Memory Ratio** | {max(memory_ratios):.3f}x |")
    md.append("")
    
    if avg_memory_ratio < 1.0:
        md.append(f"✅ **RISC0 uses {1/avg_memory_ratio:.2f}x less memory** on average")
    else:
        md.append(f"✅ **SP1 uses {avg_memory_ratio:.2f}x less memory** on average")
    md.append("")
    md.append("---")
    md.append("")
    
    # Detailed comparison table
    md.append("## Detailed Per-Test Comparison")
    md.append("")
    md.append("| Test Name | RISC0 Time (s) | SP1 Time (s) | Proving Time Winner | RISC0 Memory (GB) | SP1 Memory (GB) | Memory Winner |")
    md.append("|-----------|----------------|--------------|---------------------|-------------------|-----------------|---------------|")
    
    # Sort by time speedup (best RISC0 performance first)
    comparisons.sort(key=lambda x: x["time_speedup"])
    
    for comp in comparisons:
        test_name = comp["test_name"]
        
        # Format test name if formatter is available
        if formatter:
            display_name = formatter.format_test_name(test_name)
        else:
            display_name = test_name
            # Truncate long names
            if len(display_name) > 80:
                display_name = display_name[:77] + "..."
        
        risc0_time = comp["risc0_time"]
        sp1_time = comp["sp1_time"]
        time_speedup = comp["time_speedup"]
        risc0_memory = comp["risc0_memory"]
        sp1_memory = comp["sp1_memory"]
        memory_ratio = comp["memory_ratio"]
        
        # Format time speedup - show winner and magnitude
        if time_speedup < 1.0:
            # RISC0 is faster
            speedup_magnitude = 1.0 / time_speedup
            time_winner_str = f"**RISC0 is {speedup_magnitude:.1f}x faster**"
        else:
            # SP1 is faster
            speedup_magnitude = time_speedup
            time_winner_str = f"SP1 is {speedup_magnitude:.1f}x faster"
        
        # Format memory ratio - show winner and magnitude
        if memory_ratio < 1.0:
            # RISC0 uses less memory
            memory_magnitude = 1.0 / memory_ratio
            memory_winner_str = f"**RISC0 uses {memory_magnitude:.1f}x less**"
        else:
            # SP1 uses less memory
            memory_magnitude = memory_ratio
            memory_winner_str = f"SP1 uses {memory_magnitude:.1f}x less"
        
        md.append(f"| {display_name} | {risc0_time:,.1f} | {sp1_time:,.1f} | {time_winner_str} | {risc0_memory:.2f} | {sp1_memory:.2f} | {memory_winner_str} |")
    
    md.append("")
    md.append("---")
    md.append("")
    md.append("## Notes")
    md.append("")
    md.append("- **Proving Time Winner** shows which system is faster and by how much")
    md.append("- **Memory Winner** shows which system uses less memory and by how much")
    md.append("- Table is sorted by proving time performance (best RISC0 performance at top)")
    md.append("- Bold entries indicate RISC0 wins for that metric")
    md.append("")
    
    # Write to file
    markdown_content = "\n".join(md)
    with open(output_file, 'w') as f:
        f.write(markdown_content)
    
    print(f"\n✅ Markdown comparison saved to: {output_file}")
    print(f"   - {len(comparisons)} test comparisons")
    print(f"   - Overall: RISC0 is {1/avg_time_speedup:.2f}x faster")
    print(f"   - Memory: SP1 uses {avg_memory_ratio:.2f}x less memory")

def main():
    parser = argparse.ArgumentParser(
        description="Export proving time and memory speedup comparison to markdown."
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
        default="speedup_comparison.md",
        help="Output markdown file (default: speedup_comparison.md)"
    )
    parser.add_argument(
        "--use-display-names",
        action="store_true",
        default=True,
        help="Use display name formatter for test names (default: True)"
    )
    parser.add_argument(
        "--no-display-names",
        action="store_true",
        help="Disable display name formatting, use original test names"
    )
    
    args = parser.parse_args()
    
    # Handle display names flag
    use_display_names = args.use_display_names and not args.no_display_names
    
    print("="*80)
    print("SPEEDUP COMPARISON EXPORT")
    print("="*80)
    
    print(f"\nLoading RISC0 metrics from: {args.risc0_folder}")
    risc0_metrics = load_metrics(args.risc0_folder)
    print(f"Loaded {len(risc0_metrics)} RISC0 test results")
    
    print(f"\nLoading SP1 metrics from: {args.sp1_folder}")
    sp1_metrics = load_metrics(args.sp1_folder)
    print(f"Loaded {len(sp1_metrics)} SP1 test results")
    
    print("\nGenerating speedup comparison markdown...")
    generate_speedup_markdown(risc0_metrics, sp1_metrics, args.output, use_display_names)
    
    print("\n" + "="*80)
    print("EXPORT COMPLETE")
    print("="*80)

if __name__ == "__main__":
    main()

