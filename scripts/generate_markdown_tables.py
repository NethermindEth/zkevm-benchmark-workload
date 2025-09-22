#!/usr/bin/env python3
"""
Script to generate markdown tables from zkEVM benchmark results.

This script reads benchmark metrics from JSON files and presents them in
well-formatted markdown tables for easy viewing and analysis.

Usage:
    python3 generate_markdown_tables.py [OPTIONS] <metrics_folder> [<metrics_folder2> ...]

Options:
    --output, -o <file>     Output markdown file (default: benchmark_results.md)
    --format, -f <format>   Output format: markdown, html, csv (default: markdown)
    --compare               Compare metrics between multiple folders
    --execution-only        Only show execution metrics
    --proving-only          Only show proving metrics
    --gas-categories        Group results by gas categories
    --statistics            Include statistical analysis
    --help, -h              Show this help message

Examples:
    # Generate markdown table from single metrics folder
    python3 generate_markdown_tables.py zkevm-metrics-1M

    # Compare multiple gas categories
    python3 generate_markdown_tables.py --compare --gas-categories zkevm-metrics-1M zkevm-metrics-10M zkevm-metrics-100M

    # Generate with statistics and save to file
    python3 generate_markdown_tables.py --statistics --output results.md zkevm-metrics-1M

    # Show only execution metrics
    python3 generate_markdown_tables.py --execution-only zkevm-metrics-1M
"""

import json
import os
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Optional, Any
import statistics
from datetime import datetime
import csv

class BenchmarkMetrics:
    """Container for benchmark metrics data."""
    
    def __init__(self, name: str, gas_category: str = ""):
        self.name = name
        self.gas_category = gas_category
        self.timestamp = None
        self.metadata = {}
        self.execution_metrics = None
        self.proving_metrics = None
    
    def has_execution(self) -> bool:
        return self.execution_metrics is not None
    
    def has_proving(self) -> bool:
        return self.proving_metrics is not None
    
    def get_total_cycles(self) -> Optional[int]:
        if self.execution_metrics and "success" in self.execution_metrics:
            return self.execution_metrics["success"].get("total_num_cycles")
        return None
    
    def get_execution_duration_ms(self) -> Optional[float]:
        if self.execution_metrics and "success" in self.execution_metrics:
            duration = self.execution_metrics["success"].get("execution_duration")
            if duration:
                # Convert to milliseconds
                return duration.get("secs", 0) * 1000 + duration.get("nanos", 0) / 1_000_000
        return None
    
    def get_region_cycles(self) -> Dict[str, int]:
        if self.execution_metrics and "success" in self.execution_metrics:
            return self.execution_metrics["success"].get("region_cycles", {})
        return {}
    
    def get_proof_size(self) -> Optional[int]:
        if self.proving_metrics and "success" in self.proving_metrics:
            return self.proving_metrics["success"].get("proof_size")
        return None
    
    def get_proving_time_ms(self) -> Optional[float]:
        if self.proving_metrics and "success" in self.proving_metrics:
            return self.proving_metrics["success"].get("proving_time_ms")
        return None

def load_metrics_from_folder(folder_path: str) -> Dict[str, BenchmarkMetrics]:
    """Load all metric files from a folder."""
    metrics = {}
    folder = Path(folder_path)
    
    if not folder.exists():
        print(f"Warning: {folder} does not exist", file=sys.stderr)
        return metrics
    
    # Find all JSON files in subdirectories
    json_files = list(folder.rglob("*.json"))
    
    if not json_files:
        print(f"Warning: No JSON files found in {folder}", file=sys.stderr)
        return metrics
    
    print(f"Loading {len(json_files)} metric files from {folder}")
    
    for file_path in json_files:
        try:
            with open(file_path, 'r') as f:
                data = json.load(f)
                
                # Handle both single benchmark runs and arrays of runs
                if isinstance(data, list):
                    for i, run_data in enumerate(data):
                        name = run_data.get("name", f"{file_path.stem}_{i}")
                        metric = parse_benchmark_run(run_data, name)
                        metrics[f"{file_path.parent.name}/{name}"] = metric
                else:
                    name = data.get("name", file_path.stem)
                    metric = parse_benchmark_run(data, name)
                    metrics[f"{file_path.parent.name}/{name}"] = metric
                    
        except (json.JSONDecodeError, FileNotFoundError) as e:
            print(f"Error loading {file_path}: {e}", file=sys.stderr)
    
    return metrics

def parse_benchmark_run(data: Dict, name: str) -> BenchmarkMetrics:
    """Parse a single benchmark run from JSON data."""
    metric = BenchmarkMetrics(name)
    
    metric.timestamp = data.get("timestamp_completed")
    metric.metadata = data.get("metadata", {})
    metric.execution_metrics = data.get("execution")
    metric.proving_metrics = data.get("proving")
    
    return metric

def extract_gas_category_from_path(folder_path: str) -> str:
    """Extract gas category from folder path."""
    path = Path(folder_path)
    name = path.name
    
    # Look for gas value patterns like "1M", "10M", "100M"
    if "1M" in name:
        return "1M"
    elif "10M" in name:
        return "10M"
    elif "30M" in name:
        return "30M"
    elif "45M" in name:
        return "45M"
    elif "60M" in name:
        return "60M"
    elif "100M" in name:
        return "100M"
    elif "500M" in name:
        return "500M"
    else:
        return "Unknown"

def generate_execution_table(metrics_dict: Dict[str, BenchmarkMetrics], 
                           show_regions: bool = True) -> str:
    """Generate markdown table for execution metrics."""
    if not any(m.has_execution() for m in metrics_dict.values()):
        return "## Execution Metrics\n\nNo execution metrics found.\n\n"
    
    # Collect all unique regions
    all_regions = set()
    for metric in metrics_dict.values():
        all_regions.update(metric.get_region_cycles().keys())
    
    # Sort regions, putting total_num_cycles last if it exists
    sorted_regions = sorted(all_regions)
    if "total_num_cycles" in sorted_regions:
        sorted_regions.remove("total_num_cycles")
        sorted_regions.append("total_num_cycles")
    
    # Build table header
    header = "| Benchmark | Gas Category | Total Cycles | Duration (ms)"
    if show_regions and sorted_regions:
        header += " | " + " | ".join(region.replace("_", " ").title() for region in sorted_regions)
    header += " |\n"
    
    # Add separator
    separator = "|" + "---|" * (4 + len(sorted_regions)) + "\n"
    
    # Build table rows
    rows = []
    for name, metric in sorted(metrics_dict.items()):
        if not metric.has_execution():
            continue
            
        total_cycles = metric.get_total_cycles()
        duration = metric.get_execution_duration_ms()
        region_cycles = metric.get_region_cycles()
        
        row = f"| {name} | {metric.gas_category} | "
        row += f"{total_cycles:,}" if total_cycles else "N/A"
        row += " | "
        row += f"{duration:,.1f}" if duration else "N/A"
        
        if show_regions and sorted_regions:
            for region in sorted_regions:
                cycles = region_cycles.get(region, 0)
                row += f" | {cycles:,}" if cycles > 0 else " | -"
        
        row += " |\n"
        rows.append(row)
    
    return "## Execution Metrics\n\n" + header + separator + "".join(rows) + "\n"

def generate_proving_table(metrics_dict: Dict[str, BenchmarkMetrics]) -> str:
    """Generate markdown table for proving metrics."""
    if not any(m.has_proving() for m in metrics_dict.values()):
        return "## Proving Metrics\n\nNo proving metrics found.\n\n"
    
    header = "| Benchmark | Gas Category | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) |\n"
    separator = "|---|---|---|---|---|\n"
    
    rows = []
    for name, metric in sorted(metrics_dict.items()):
        if not metric.has_proving():
            continue
            
        proof_size = metric.get_proof_size()
        proving_time_ms = metric.get_proving_time_ms()
        proving_time_s = proving_time_ms / 1000.0 if proving_time_ms else None
        
        row = f"| {name} | {metric.gas_category} | "
        row += f"{proof_size:,}" if proof_size else "N/A"
        row += " | "
        row += f"{proving_time_ms:,.1f}" if proving_time_ms else "N/A"
        row += " | "
        row += f"{proving_time_s:,.2f}" if proving_time_s else "N/A"
        row += " |\n"
        rows.append(row)
    
    return "## Proving Metrics\n\n" + header + separator + "".join(rows) + "\n"

def generate_comparison_table(metrics_folders: List[str], 
                            metrics_type: str = "execution") -> str:
    """Generate comparison table across multiple gas categories."""
    all_metrics = {}
    
    for folder in metrics_folders:
        gas_category = extract_gas_category_from_path(folder)
        folder_metrics = load_metrics_from_folder(folder)
        
        for name, metric in folder_metrics.items():
            metric.gas_category = gas_category
            # Create unique key combining name and gas category
            key = f"{name}_{gas_category}"
            all_metrics[key] = metric
    
    if metrics_type == "execution":
        return generate_execution_table(all_metrics, show_regions=False)
    else:
        return generate_proving_table(all_metrics)

def generate_statistics_section(metrics_dict: Dict[str, BenchmarkMetrics]) -> str:
    """Generate statistical analysis section."""
    if not metrics_dict:
        return "## Statistics\n\nNo data available for statistical analysis.\n\n"
    
    stats = []
    
    # Execution statistics
    execution_metrics = [m for m in metrics_dict.values() if m.has_execution()]
    if execution_metrics:
        total_cycles = [m.get_total_cycles() for m in execution_metrics if m.get_total_cycles()]
        durations = [m.get_execution_duration_ms() for m in execution_metrics if m.get_execution_duration_ms()]
        
        if total_cycles:
            stats.append("### Execution Statistics")
            stats.append(f"- **Total Cycles**: Min: {min(total_cycles):,}, Max: {max(total_cycles):,}, Avg: {statistics.mean(total_cycles):,.0f}")
            stats.append(f"- **Duration**: Min: {min(durations):.1f}ms, Max: {max(durations):.1f}ms, Avg: {statistics.mean(durations):.1f}ms")
            stats.append("")
    
    # Proving statistics
    proving_metrics = [m for m in metrics_dict.values() if m.has_proving()]
    if proving_metrics:
        proof_sizes = [m.get_proof_size() for m in proving_metrics if m.get_proof_size()]
        proving_times = [m.get_proving_time_ms() for m in proving_metrics if m.get_proving_time_ms()]
        
        if proof_sizes:
            stats.append("### Proving Statistics")
            stats.append(f"- **Proof Size**: Min: {min(proof_sizes):,} bytes, Max: {max(proof_sizes):,} bytes, Avg: {statistics.mean(proof_sizes):,.0f} bytes")
            stats.append(f"- **Proving Time**: Min: {min(proving_times):.1f}ms, Max: {max(proving_times):.1f}ms, Avg: {statistics.mean(proving_times):.1f}ms")
            stats.append("")
    
    return "## Statistics\n\n" + "\n".join(stats) + "\n" if stats else "## Statistics\n\nNo statistical data available.\n\n"

def generate_summary_table(metrics_folders: List[str]) -> str:
    """Generate summary table showing metrics per gas category."""
    summary_data = []
    
    for folder in metrics_folders:
        gas_category = extract_gas_category_from_path(folder)
        metrics = load_metrics_from_folder(folder)
        
        execution_count = sum(1 for m in metrics.values() if m.has_execution())
        proving_count = sum(1 for m in metrics.values() if m.has_proving())
        total_count = len(metrics)
        
        # Calculate averages
        avg_cycles = 0
        avg_duration = 0
        avg_proof_size = 0
        avg_proving_time = 0
        
        if execution_count > 0:
            cycles = [m.get_total_cycles() for m in metrics.values() if m.get_total_cycles()]
            durations = [m.get_execution_duration_ms() for m in metrics.values() if m.get_execution_duration_ms()]
            avg_cycles = statistics.mean(cycles) if cycles else 0
            avg_duration = statistics.mean(durations) if durations else 0
        
        if proving_count > 0:
            proof_sizes = [m.get_proof_size() for m in metrics.values() if m.get_proof_size()]
            proving_times = [m.get_proving_time_ms() for m in metrics.values() if m.get_proving_time_ms()]
            avg_proof_size = statistics.mean(proof_sizes) if proof_sizes else 0
            avg_proving_time = statistics.mean(proving_times) if proving_times else 0
        
        summary_data.append({
            'gas_category': gas_category,
            'total_benchmarks': total_count,
            'execution_benchmarks': execution_count,
            'proving_benchmarks': proving_count,
            'avg_cycles': avg_cycles,
            'avg_duration': avg_duration,
            'avg_proof_size': avg_proof_size,
            'avg_proving_time': avg_proving_time
        })
    
    if not summary_data:
        return "## Summary\n\nNo data available.\n\n"
    
    header = "| Gas Category | Total Benchmarks | Execution | Proving | Avg Cycles | Avg Duration (ms) | Avg Proof Size (bytes) | Avg Proving Time (ms) |\n"
    separator = "|---|---|---|---|---|---|---|---|\n"
    
    rows = []
    for data in sorted(summary_data, key=lambda x: x['gas_category']):
        row = f"| {data['gas_category']} | {data['total_benchmarks']} | "
        row += f"{data['execution_benchmarks']} | {data['proving_benchmarks']} | "
        row += f"{data['avg_cycles']:,.0f}" if data['avg_cycles'] > 0 else "N/A"
        row += " | "
        row += f"{data['avg_duration']:,.1f}" if data['avg_duration'] > 0 else "N/A"
        row += " | "
        row += f"{data['avg_proof_size']:,.0f}" if data['avg_proof_size'] > 0 else "N/A"
        row += " | "
        row += f"{data['avg_proving_time']:,.1f}" if data['avg_proving_time'] > 0 else "N/A"
        row += " |\n"
        rows.append(row)
    
    return "## Summary by Gas Category\n\n" + header + separator + "".join(rows) + "\n"

def main():
    """Main function."""
    parser = argparse.ArgumentParser(
        description="Generate markdown tables from zkEVM benchmark results",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=__doc__
    )
    
    parser.add_argument("metrics_folders", nargs="+", 
                       help="One or more metrics folders to process")
    parser.add_argument("--output", "-o", default="benchmark_results.md",
                       help="Output markdown file (default: benchmark_results.md)")
    parser.add_argument("--format", "-f", choices=["markdown", "html", "csv"], 
                       default="markdown", help="Output format (default: markdown)")
    parser.add_argument("--compare", action="store_true",
                       help="Compare metrics between multiple folders")
    parser.add_argument("--execution-only", action="store_true",
                       help="Only show execution metrics")
    parser.add_argument("--proving-only", action="store_true",
                       help="Only show proving metrics")
    parser.add_argument("--gas-categories", action="store_true",
                       help="Group results by gas categories")
    parser.add_argument("--statistics", action="store_true",
                       help="Include statistical analysis")
    
    args = parser.parse_args()
    
    # Validate arguments
    if args.execution_only and args.proving_only:
        print("Error: Cannot specify both --execution-only and --proving-only", file=sys.stderr)
        sys.exit(1)
    
    # Check if folders exist
    for folder in args.metrics_folders:
        if not Path(folder).exists():
            print(f"Error: Folder {folder} does not exist", file=sys.stderr)
            sys.exit(1)
    
    # Generate content
    content_parts = []
    
    # Add header
    content_parts.append(f"# zkEVM Benchmark Results\n\n")
    content_parts.append(f"Generated on: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
    
    if len(args.metrics_folders) > 1:
        content_parts.append(f"Comparing {len(args.metrics_folders)} metrics folders:\n")
        for folder in args.metrics_folders:
            gas_category = extract_gas_category_from_path(folder)
            content_parts.append(f"- {folder} (Gas: {gas_category})\n")
        content_parts.append("\n")
    
    # Generate summary table if comparing multiple folders
    if args.compare and len(args.metrics_folders) > 1:
        content_parts.append(generate_summary_table(args.metrics_folders))
    
    # Process each folder or generate comparison
    if args.compare and len(args.metrics_folders) > 1:
        # Generate comparison tables
        if not args.proving_only:
            content_parts.append(generate_comparison_table(args.metrics_folders, "execution"))
        if not args.execution_only:
            content_parts.append(generate_comparison_table(args.metrics_folders, "proving"))
    else:
        # Process individual folders
        for folder in args.metrics_folders:
            gas_category = extract_gas_category_from_path(folder)
            content_parts.append(f"## Gas Category: {gas_category}\n\n")
            content_parts.append(f"Source: {folder}\n\n")
            
            metrics = load_metrics_from_folder(folder)
            
            if not args.proving_only:
                content_parts.append(generate_execution_table(metrics))
            if not args.execution_only:
                content_parts.append(generate_proving_table(metrics))
            
            if args.statistics:
                content_parts.append(generate_statistics_section(metrics))
    
    # Combine all content
    full_content = "".join(content_parts)
    
    # Write output
    if args.output == "-":
        print(full_content)
    else:
        with open(args.output, 'w') as f:
            f.write(full_content)
        print(f"Results written to {args.output}")

if __name__ == "__main__":
    main()
