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
    --statistics            Include statistical analysis
    --help, -h              Show this help message

Examples:
    # Generate markdown table from single metrics folder
    python3 generate_markdown_tables.py zkevm-metrics-1M

    # Compare multiple folders
    python3 generate_markdown_tables.py --compare zkevm-metrics-1M zkevm-metrics-10M zkevm-metrics-100M

    # Generate with statistics and save to file
    python3 generate_markdown_tables.py --statistics --output results.md zkevm-metrics-1M

    # Show only execution metrics
    python3 generate_markdown_tables.py --execution-only zkevm-metrics-1M
    
    # Use simplified names in tables
    python3 generate_markdown_tables.py --name-format simplified zkevm-metrics-1M
    
    # Use human-readable display names
    python3 generate_markdown_tables.py --name-format display zkevm-metrics-1M
"""

import json
import sys
import argparse
from pathlib import Path
from typing import Dict, List, Optional, NamedTuple
import statistics
from datetime import datetime

# Import the test name parser
try:
    from test_name_parser import get_display_name, get_simplified_name, get_category, parse_test_name, TestInfo
except ImportError:
    # Fallback if the parser is not available
    class TestInfo(NamedTuple):  # type: ignore[no-redef]
        """Container for parsed test information."""
        category: str
        function: str
        parameters: List[str]
        simplified_name: str
        display_name: str
    
    def get_display_name(filename: str) -> str:
        return filename
    def get_simplified_name(filename: str) -> str:
        return filename
    def get_category(filename: str) -> str:
        return "unknown"
    def parse_test_name(filename: str) -> TestInfo:
        return TestInfo(
            category="unknown",
            function="",
            parameters=[],
            simplified_name=filename,
            display_name=filename
        )

class HardwareInfo:
    """Container for hardware information."""
    
    def __init__(self):
        self.cpu_model: Optional[str] = None
        self.total_ram_gib: Optional[int] = None
        self.gpus: List[Dict[str, str]] = []
    
    def __str__(self) -> str:
        """Format hardware info as a readable string."""
        parts = []
        if self.cpu_model:
            parts.append(f"CPU: {self.cpu_model}")
        if self.total_ram_gib:
            parts.append(f"RAM: {self.total_ram_gib} GiB")
        if self.gpus:
            gpu_models = [gpu.get("model", "Unknown") for gpu in self.gpus]
            parts.append(f"GPU: {', '.join(gpu_models)}")
        return " | ".join(parts) if parts else "Hardware info not available"


class BenchmarkMetrics:
    """Container for benchmark metrics data."""
    
    def __init__(self, name: str):
        self.name = name
        self.timestamp = None
        self.metadata: Dict[str, int] = {}
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
    
    def get_peak_memory_bytes(self) -> Optional[int]:
        if self.proving_metrics and "success" in self.proving_metrics:
            return self.proving_metrics["success"].get("peak_memory_usage_bytes")
        return None
    
    def get_average_memory_bytes(self) -> Optional[int]:
        if self.proving_metrics and "success" in self.proving_metrics:
            return self.proving_metrics["success"].get("average_memory_usage_bytes")
        return None
    
    def get_initial_memory_bytes(self) -> Optional[int]:
        if self.proving_metrics and "success" in self.proving_metrics:
            return self.proving_metrics["success"].get("initial_memory_usage_bytes")
        return None
    
    def get_gas_used(self) -> Optional[int]:
        """Get gas used from metadata."""
        return self.metadata.get("block_used_gas")


def load_hardware_info(folder_path: str) -> Optional[HardwareInfo]:
    """Load hardware information from a folder's hardware.json file."""
    folder = Path(folder_path)
    hardware_file = folder / "hardware.json"
    
    if not hardware_file.exists():
        return None
    
    try:
        with open(hardware_file, 'r') as f:
            data = json.load(f)
            
            hardware = HardwareInfo()
            hardware.cpu_model = data.get("cpu_model")
            hardware.total_ram_gib = data.get("total_ram_gib")
            hardware.gpus = data.get("gpus", [])
            
            return hardware
    except (json.JSONDecodeError, FileNotFoundError) as e:
        print(f"Warning: Could not load hardware info from {hardware_file}: {e}", file=sys.stderr)
        return None


def format_benchmark_name(name: str, name_format: str = "original") -> str:
    """
    Format a benchmark name based on the specified format.
    
    Args:
        name: The original benchmark name (may include parent directory)
        name_format: Format to use ("original", "display", "simplified", "category")
        
    Returns:
        Formatted benchmark name
    """
    if name_format == "original":
        return name
    
    # Extract just the filename part if it includes parent directory
    filename = name
    if '/' in name:
        filename = name.split('/', 1)[1]
    
    if name_format == "display":
        return get_display_name(filename)
    elif name_format == "simplified":
        return get_simplified_name(filename)
    elif name_format == "category":
        category = get_category(filename)
        simplified = get_simplified_name(filename)
        return f"[{category}] {simplified}"
    else:
        return name

def load_metrics_from_folder(folder_path: str) -> Dict[str, BenchmarkMetrics]:
    """Load all metric files from a folder."""
    metrics: Dict[str, BenchmarkMetrics] = {}
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
                        # Use filename for display purposes
                        display_key = f"{file_path.parent.name}/{file_path.stem}"
                        metrics[display_key] = metric
                else:
                    name = data.get("name", file_path.stem)
                    metric = parse_benchmark_run(data, name)
                    # Use filename for display purposes
                    display_key = f"{file_path.parent.name}/{file_path.stem}"
                    metrics[display_key] = metric
                    
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


def generate_execution_table(metrics_dict: Dict[str, BenchmarkMetrics], 
                           show_regions: bool = True, name_format: str = "original") -> str:
    """Generate markdown table for execution metrics."""
    if not any(m.has_execution() for m in metrics_dict.values()):
        return "## Execution Metrics\n\nNo execution metrics found.\n\n"
    
    # Collect all unique regions
    all_regions: set[str] = set()
    for metric in metrics_dict.values():
        all_regions.update(metric.get_region_cycles().keys())
    
    # Sort regions, putting total_num_cycles last if it exists
    sorted_regions = sorted(all_regions)
    if "total_num_cycles" in sorted_regions:
        sorted_regions.remove("total_num_cycles")
        sorted_regions.append("total_num_cycles")
    
    # Build table header
    header = "| Benchmark | Gas Used | Total Cycles | Duration (ms)"
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
        gas_used = metric.get_gas_used()
        
        formatted_name = format_benchmark_name(name, name_format)
        row = f"| {formatted_name} | "
        row += f"{gas_used:,}" if gas_used else "N/A"
        row += " | "
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

def generate_proving_table(metrics_dict: Dict[str, BenchmarkMetrics], name_format: str = "original") -> str:
    """Generate markdown table for proving metrics."""
    if not any(m.has_proving() for m in metrics_dict.values()):
        return "## Proving Metrics\n\nNo proving metrics found.\n\n"
    
    header = "| Benchmark | Gas Used | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |\n"
    separator = "|---|---|---|---|---|---|---|---|\n"
    
    rows = []
    for name, metric in sorted(metrics_dict.items()):
        if not metric.has_proving():
            continue
            
        proof_size = metric.get_proof_size()
        proving_time_ms = metric.get_proving_time_ms()
        proving_time_s = proving_time_ms / 1000.0 if proving_time_ms else None
        gas_used = metric.get_gas_used()
        
        # Memory metrics
        peak_memory_bytes = metric.get_peak_memory_bytes()
        avg_memory_bytes = metric.get_average_memory_bytes()
        initial_memory_bytes = metric.get_initial_memory_bytes()
        
        # Convert bytes to MB
        peak_memory_mb = peak_memory_bytes / (1024 * 1024) if peak_memory_bytes else None
        avg_memory_mb = avg_memory_bytes / (1024 * 1024) if avg_memory_bytes else None
        initial_memory_mb = initial_memory_bytes / (1024 * 1024) if initial_memory_bytes else None
        
        formatted_name = format_benchmark_name(name, name_format)
        row = f"| {formatted_name} | "
        row += f"{gas_used:,}" if gas_used else "N/A"
        row += " | "
        row += f"{proof_size:,}" if proof_size else "N/A"
        row += " | "
        row += f"{proving_time_ms:,.1f}" if proving_time_ms else "N/A"
        row += " | "
        row += f"{proving_time_s:,.2f}" if proving_time_s else "N/A"
        row += " | "
        row += f"{peak_memory_mb:,.1f}" if peak_memory_mb else "N/A"
        row += " | "
        row += f"{avg_memory_mb:,.1f}" if avg_memory_mb else "N/A"
        row += " | "
        row += f"{initial_memory_mb:,.1f}" if initial_memory_mb else "N/A"
        row += " |\n"
        rows.append(row)
    
    return "## Proving Metrics\n\n" + header + separator + "".join(rows) + "\n"

def generate_comparison_table(metrics_folders: List[str], 
                            metrics_type: str = "execution", name_format: str = "original") -> str:
    """Generate comparison table across multiple folders."""
    all_metrics = {}
    
    for folder in metrics_folders:
        folder_metrics = load_metrics_from_folder(folder)
        
        for name, metric in folder_metrics.items():
            # Create unique key combining name and folder
            key = f"{name}_{folder}"
            all_metrics[key] = metric
    
    if metrics_type == "execution":
        return generate_execution_table(all_metrics, show_regions=False, name_format=name_format)
    else:
        return generate_proving_table(all_metrics, name_format=name_format)

def generate_statistics_section(metrics_dict: Dict[str, BenchmarkMetrics]) -> str:
    """Generate statistical analysis section."""
    if not metrics_dict:
        return "## Statistics\n\nNo data available for statistical analysis.\n\n"
    
    stats = []
    
    # Execution statistics
    execution_metrics = [m for m in metrics_dict.values() if m.has_execution()]
    if execution_metrics:
        total_cycles = [m.get_total_cycles() for m in execution_metrics if m.get_total_cycles() is not None]
        durations = [m.get_execution_duration_ms() for m in execution_metrics if m.get_execution_duration_ms() is not None]
        
        if total_cycles and durations:
            stats.append("### Execution Statistics")
            # Type assertion: we know these are not None due to the filter above
            total_cycles_clean = [c for c in total_cycles if c is not None]
            durations_clean = [d for d in durations if d is not None]
            stats.append(f"- **Total Cycles**: Min: {min(total_cycles_clean):,}, Max: {max(total_cycles_clean):,}, Avg: {statistics.mean(total_cycles_clean):,.0f}")
            stats.append(f"- **Duration**: Min: {min(durations_clean):.1f}ms, Max: {max(durations_clean):.1f}ms, Avg: {statistics.mean(durations_clean):.1f}ms")
            stats.append("")
    
    # Proving statistics
    proving_metrics = [m for m in metrics_dict.values() if m.has_proving()]
    if proving_metrics:
        proof_sizes = [m.get_proof_size() for m in proving_metrics if m.get_proof_size() is not None]
        proving_times = [m.get_proving_time_ms() for m in proving_metrics if m.get_proving_time_ms() is not None]
        peak_memories = [m.get_peak_memory_bytes() for m in proving_metrics if m.get_peak_memory_bytes() is not None]
        
        if proof_sizes and proving_times:
            stats.append("### Proving Statistics")
            # Type assertion: we know these are not None due to the filter above
            proof_sizes_clean = [p for p in proof_sizes if p is not None]
            proving_times_clean = [t for t in proving_times if t is not None]
            stats.append(f"- **Proof Size**: Min: {min(proof_sizes_clean):,} bytes, Max: {max(proof_sizes_clean):,} bytes, Avg: {statistics.mean(proof_sizes_clean):,.0f} bytes")
            stats.append(f"- **Proving Time**: Min: {min(proving_times_clean):.1f}ms, Max: {max(proving_times_clean):.1f}ms, Avg: {statistics.mean(proving_times_clean):.1f}ms")
            if peak_memories:
                peak_memories_clean = [m for m in peak_memories if m is not None]
                peak_memories_mb = [m / (1024 * 1024) for m in peak_memories_clean]
                stats.append(f"- **Peak Memory**: Min: {min(peak_memories_mb):.1f}MB, Max: {max(peak_memories_mb):.1f}MB, Avg: {statistics.mean(peak_memories_mb):.1f}MB")
            stats.append("")
    
    return "## Statistics\n\n" + "\n".join(stats) + "\n" if stats else "## Statistics\n\nNo statistical data available.\n\n"

def generate_summary_table(metrics_folders: List[str]) -> str:
    """Generate summary table showing metrics per folder."""
    summary_data = []
    
    for folder in metrics_folders:
        metrics = load_metrics_from_folder(folder)
        hardware = load_hardware_info(folder)
        
        execution_count = sum(1 for m in metrics.values() if m.has_execution())
        proving_count = sum(1 for m in metrics.values() if m.has_proving())
        total_count = len(metrics)
        
        # Calculate averages
        avg_cycles = 0
        avg_duration = 0.0
        avg_proof_size = 0
        avg_proving_time = 0.0
        
        if execution_count > 0:
            cycles = [m.get_total_cycles() for m in metrics.values() if m.get_total_cycles() is not None]
            durations = [m.get_execution_duration_ms() for m in metrics.values() if m.get_execution_duration_ms() is not None]
            if cycles:
                cycles_clean = [c for c in cycles if c is not None]
                avg_cycles = int(statistics.mean(cycles_clean))
            else:
                avg_cycles = 0
            if durations:
                durations_clean = [d for d in durations if d is not None]
                avg_duration = statistics.mean(durations_clean)
            else:
                avg_duration = 0
        
        if proving_count > 0:
            proof_sizes = [m.get_proof_size() for m in metrics.values() if m.get_proof_size() is not None]
            proving_times = [m.get_proving_time_ms() for m in metrics.values() if m.get_proving_time_ms() is not None]
            if proof_sizes:
                proof_sizes_clean = [p for p in proof_sizes if p is not None]
                avg_proof_size = int(statistics.mean(proof_sizes_clean))
            else:
                avg_proof_size = 0
            if proving_times:
                proving_times_clean = [t for t in proving_times if t is not None]
                avg_proving_time = statistics.mean(proving_times_clean)
            else:
                avg_proving_time = 0
        
        summary_data.append({
            'folder': folder,
            'total_benchmarks': total_count,
            'execution_benchmarks': execution_count,
            'proving_benchmarks': proving_count,
            'avg_cycles': avg_cycles,
            'avg_duration': avg_duration,
            'avg_proof_size': avg_proof_size,
            'avg_proving_time': avg_proving_time,
            'hardware': hardware
        })
    
    if not summary_data:
        return "## Summary\n\nNo data available.\n\n"
    
    header = "| Folder | Hardware | Total Benchmarks | Execution | Proving | Avg Cycles | Avg Duration (ms) | Avg Proof Size (bytes) | Avg Proving Time (ms) |\n"
    separator = "|---|---|---|---|---|---|---|---|---|\n"
    
    rows = []
    for data in sorted(summary_data, key=lambda x: str(x['folder'])):
        row = f"| {data['folder']} | "
        row += str(data['hardware']) if data['hardware'] else "N/A"
        row += f" | {data['total_benchmarks']} | "
        row += f"{data['execution_benchmarks']} | {data['proving_benchmarks']} | "
        row += f"{data['avg_cycles']:,.0f}" if isinstance(data['avg_cycles'], int) and data['avg_cycles'] > 0 else "N/A"
        row += " | "
        row += f"{data['avg_duration']:,.1f}" if isinstance(data['avg_duration'], (int, float)) and data['avg_duration'] > 0 else "N/A"
        row += " | "
        row += f"{data['avg_proof_size']:,.0f}" if isinstance(data['avg_proof_size'], int) and data['avg_proof_size'] > 0 else "N/A"
        row += " | "
        row += f"{data['avg_proving_time']:,.1f}" if isinstance(data['avg_proving_time'], (int, float)) and data['avg_proving_time'] > 0 else "N/A"
        row += " |\n"
        rows.append(row)
    
    return "## Summary by Folder\n\n" + header + separator + "".join(rows) + "\n"

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
    parser.add_argument("--statistics", action="store_true",
                       help="Include statistical analysis")
    parser.add_argument("--name-format", choices=["original", "display", "simplified", "category"],
                       default="original", help="Format for benchmark names (default: original)")
    
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
    content_parts.append("# zkEVM Benchmark Results\n\n")
    content_parts.append(f"Generated on: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
    
    if len(args.metrics_folders) > 1:
        content_parts.append(f"Comparing {len(args.metrics_folders)} metrics folders:\n")
        for folder in args.metrics_folders:
            content_parts.append(f"- {folder}\n")
        content_parts.append("\n")
    
    # Generate summary table if comparing multiple folders
    if args.compare and len(args.metrics_folders) > 1:
        # Add hardware information section
        content_parts.append("## Hardware Configurations\n\n")
        for folder in args.metrics_folders:
            hardware = load_hardware_info(folder)
            if hardware:
                content_parts.append(f"**{folder}**: {hardware}\n\n")
            else:
                content_parts.append(f"**{folder}**: Hardware info not available\n\n")
        content_parts.append("\n")
        
        content_parts.append(generate_summary_table(args.metrics_folders))
    
    # Process each folder or generate comparison
    if args.compare and len(args.metrics_folders) > 1:
        # Generate comparison tables
        if not args.proving_only:
            content_parts.append(generate_comparison_table(args.metrics_folders, "execution", args.name_format))
        if not args.execution_only:
            content_parts.append(generate_comparison_table(args.metrics_folders, "proving", args.name_format))
    else:
        # Process individual folders
        for folder in args.metrics_folders:
            content_parts.append(f"## Folder: {folder}\n\n")
            
            # Add hardware information
            hardware = load_hardware_info(folder)
            if hardware:
                content_parts.append(f"**Hardware Configuration:** {hardware}\n\n")
            
            metrics = load_metrics_from_folder(folder)
            
            if not args.proving_only:
                content_parts.append(generate_execution_table(metrics, name_format=args.name_format))
            if not args.execution_only:
                content_parts.append(generate_proving_table(metrics, name_format=args.name_format))
            
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
