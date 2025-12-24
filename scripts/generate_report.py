#!/usr/bin/env python3
"""
Generate a markdown report from benchmark CSV results.

Usage:
    python generate_report.py --input results.csv --output report.md
"""

import argparse
import io
import base64
from pathlib import Path
from typing import Dict, Optional, List
from datetime import datetime

import pandas as pd
import numpy as np
from scipy import stats
import matplotlib.pyplot as plt


def compute_regression(df: pd.DataFrame, opcode: str, metric: str) -> Optional[Dict]:
    """Compute linear regression for an opcode on a given metric."""
    sub = df[df['opcode'] == opcode].dropna(subset=[metric])
    if len(sub) < 3:
        return None
    
    x = sub['op_count'].values
    y = sub[metric].values
    
    try:
        slope, intercept, r_value, p_value, std_err = stats.linregress(x, y)
    except Exception:
        return None
    
    return {
        'opcode': opcode,
        'slope': slope,
        'intercept': intercept,
        'r_squared': r_value ** 2,
        'std_err': std_err,
        'n_points': len(sub),
        'min_op_count': int(sub['op_count'].min()),
        'max_op_count': int(sub['op_count'].max()),
    }


def create_scatter_plot(df: pd.DataFrame, opcode: str, regression: Dict, 
                        metric: str, y_label: str) -> str:
    """Create scatter plot with regression line, return as base64 PNG."""
    sub = df[df['opcode'] == opcode].dropna(subset=[metric])
    
    fig, ax = plt.subplots(figsize=(8, 5))
    
    # Determine scale factor and unit
    y_values = sub[metric].values
    max_y = y_values.max()
    
    if max_y >= 1e9:
        scale = 1e9
        unit = 'B' if 'cycles' in metric.lower() else 's'
        unit_name = 'billion' if 'cycles' in metric.lower() else 'seconds'
    elif max_y >= 1e6:
        scale = 1e6
        unit = 'M' if 'cycles' in metric.lower() else 'ms'
        unit_name = 'million' if 'cycles' in metric.lower() else 'milliseconds'
    elif max_y >= 1e3:
        scale = 1e3
        unit = 'K' if 'cycles' in metric.lower() else 'μs'
        unit_name = 'thousand' if 'cycles' in metric.lower() else 'microseconds'
    else:
        scale = 1
        unit = '' if 'cycles' in metric.lower() else 'ns'
        unit_name = '' if 'cycles' in metric.lower() else 'nanoseconds'
    
    # Scatter points
    ax.scatter(sub['op_count'], y_values / scale, alpha=0.6, s=50, label='Data points')
    
    # Regression line
    x_line = np.linspace(sub['op_count'].min(), sub['op_count'].max(), 100)
    y_line = (regression['slope'] * x_line + regression['intercept']) / scale
    
    slope_scaled = regression['slope'] / scale
    ax.plot(x_line, y_line, 'r-', linewidth=2, 
            label=f'Slope: {slope_scaled:.4f} {unit}/op')
    
    ax.set_xlabel('Op Count', fontsize=12)
    ax.set_ylabel(f'{y_label} ({unit})', fontsize=12)
    ax.set_title(f'{opcode} (R² = {regression["r_squared"]:.4f})', fontsize=14)
    ax.legend(loc='best')
    ax.grid(True, alpha=0.3)
    
    # Save to base64
    buf = io.BytesIO()
    fig.tight_layout()
    fig.savefig(buf, format='png', dpi=100)
    buf.seek(0)
    plt.close(fig)
    
    return base64.b64encode(buf.read()).decode('utf-8')


def format_time(ns: float) -> str:
    """Format nanoseconds to human-readable time."""
    if ns >= 1e9:
        return f"{ns/1e9:.2f}s"
    elif ns >= 1e6:
        return f"{ns/1e6:.2f}ms"
    elif ns >= 1e3:
        return f"{ns/1e3:.2f}μs"
    else:
        return f"{ns:.2f}ns"


def format_cycles(cycles: float) -> str:
    """Format cycles to human-readable format."""
    if cycles >= 1e9:
        return f"{cycles/1e9:.2f}B"
    elif cycles >= 1e6:
        return f"{cycles/1e6:.2f}M"
    elif cycles >= 1e3:
        return f"{cycles/1e3:.2f}K"
    else:
        return f"{cycles:.2f}"


def format_memory(bytes_val: float) -> str:
    """Format bytes to human-readable format."""
    if bytes_val >= 1e9:
        return f"{bytes_val/1e9:.2f}GB"
    elif bytes_val >= 1e6:
        return f"{bytes_val/1e6:.2f}MB"
    elif bytes_val >= 1e3:
        return f"{bytes_val/1e3:.2f}KB"
    else:
        return f"{bytes_val:.0f}B"


def generate_report(df: pd.DataFrame, output_path: Path, title: str) -> None:
    """Generate markdown report with tables and plots."""
    
    report_lines = [
        f"# {title}",
        "",
        f"*Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*",
        "",
        "## Summary",
        "",
    ]
    
    # Compute regressions for execution time
    opcodes = sorted(df['opcode'].unique())
    
    results_time = []
    results_cycles = []
    results_prove = []
    
    for opcode in opcodes:
        # Execution time regression
        if 'execution_time_ns' in df.columns and df['execution_time_ns'].notna().any():
            reg = compute_regression(df, opcode, 'execution_time_ns')
            if reg:
                results_time.append(reg)
        
        # Proving time regression
        if 'prove_time_ns' in df.columns and df['prove_time_ns'].notna().any():
            reg = compute_regression(df, opcode, 'prove_time_ns')
            if reg:
                results_prove.append(reg)
        
        # Cycles regression
        if 'total_num_cycles' in df.columns and df['total_num_cycles'].notna().any():
            reg = compute_regression(df, opcode, 'total_num_cycles')
            if reg:
                results_cycles.append(reg)
    
    # Sort by slope (most expensive first)
    results_time.sort(key=lambda x: x['slope'], reverse=True)
    results_prove.sort(key=lambda x: x['slope'], reverse=True)
    results_cycles.sort(key=lambda x: x['slope'], reverse=True)
    
    # === Execution Time Table ===
    if results_time:
        report_lines.extend([
            "### Marginal Cost by Execution Time",
            "",
            f"Total: **{len(results_time)}** opcodes/precompiles with positive slopes.",
            "",
            "| Rank | Opcode/Precompile | Marginal Time (per op) | R² |",
            "|------|-------------------|------------------------|-----|"
        ])
        
        for i, reg in enumerate(results_time, 1):
            time_str = format_time(reg['slope'])
            report_lines.append(
                f"| {i} | {reg['opcode']} | {time_str} | {reg['r_squared']:.4f} |"
            )
        
        report_lines.append("")
    
    # === Proving Time Table ===
    if results_prove:
        report_lines.extend([
            "### Marginal Cost by Proving Time",
            "",
            f"Total: **{len(results_prove)}** opcodes/precompiles with positive slopes.",
            "",
            "| Rank | Opcode/Precompile | Marginal Proving Time (per op) | R² |",
            "|------|-------------------|--------------------------------|-----|"
        ])
        
        for i, reg in enumerate(results_prove, 1):
            time_str = format_time(reg['slope'])
            report_lines.append(
                f"| {i} | {reg['opcode']} | {time_str} | {reg['r_squared']:.4f} |"
            )
        
        report_lines.append("")
    
    # === ZK Cycles Table ===
    if results_cycles:
        report_lines.extend([
            "### Marginal Cost by ZK Cycles",
            "",
            f"Total: **{len(results_cycles)}** opcodes/precompiles with positive slopes.",
            "",
            "| Rank | Opcode/Precompile | Marginal Cycles (per op) | R² |",
            "|------|-------------------|--------------------------|-----|"
        ])
        
        for i, reg in enumerate(results_cycles, 1):
            cycles_str = format_cycles(reg['slope'])
            report_lines.append(
                f"| {i} | {reg['opcode']} | {cycles_str} | {reg['r_squared']:.4f} |"
            )
        
        report_lines.append("")
    
    # === Memory Usage Summary (if available) ===
    if 'peak_memory_bytes' in df.columns and df['peak_memory_bytes'].notna().any():
        report_lines.extend([
            "### Peak Memory Usage",
            "",
            "| Opcode/Precompile | Avg Peak Memory | Max Peak Memory |",
            "|-------------------|-----------------|-----------------|"
        ])
        
        for opcode in opcodes:
            sub = df[df['opcode'] == opcode]['peak_memory_bytes'].dropna()
            if len(sub) > 0:
                avg_mem = sub.mean()
                max_mem = sub.max()
                report_lines.append(
                    f"| {opcode} | {format_memory(avg_mem)} | {format_memory(max_mem)} |"
                )
        
        report_lines.append("")
    
    # === Scatter Plots ===
    report_lines.extend([
        "---",
        "",
        "## Regression Plots",
        "",
    ])
    
    # Choose best metric for plots (execution time > proving time > cycles)
    if results_time:
        plot_results = results_time
        plot_metric = 'execution_time_ns'
        plot_label = 'Execution Time'
    elif results_prove:
        plot_results = results_prove
        plot_metric = 'prove_time_ns'
        plot_label = 'Proving Time'
    else:
        plot_results = results_cycles
        plot_metric = 'total_num_cycles'
        plot_label = 'Total Cycles'
    
    for reg in plot_results:
        opcode = reg['opcode']
        img_b64 = create_scatter_plot(df, opcode, reg, plot_metric, plot_label)
        
        slope_str = format_time(reg['slope']) if 'time' in plot_metric.lower() else format_cycles(reg['slope'])
        report_lines.extend([
            f"### {opcode}",
            "",
            f"- **Slope**: {slope_str}/op",
            f"- **R²**: {reg['r_squared']:.4f}",
            f"- **Data points**: {reg['n_points']}",
            f"- **Op count range**: {reg['min_op_count']} - {reg['max_op_count']}",
            "",
            f"![{opcode}](data:image/png;base64,{img_b64})",
            "",
        ])
    
    # Write report
    output_path.write_text('\n'.join(report_lines))
    print(f"Report generated: {output_path}")


def main():
    parser = argparse.ArgumentParser(
        description='Generate markdown report from benchmark CSV'
    )
    parser.add_argument(
        '--input', '-i',
        type=Path,
        required=True,
        help='Input CSV file with benchmark results'
    )
    parser.add_argument(
        '--output', '-o',
        type=Path,
        default=Path('report.md'),
        help='Output markdown file path'
    )
    parser.add_argument(
        '--title', '-t',
        type=str,
        default='ZK Benchmark Report',
        help='Report title'
    )
    
    args = parser.parse_args()
    
    if not args.input.exists():
        print(f"Error: Input file not found: {args.input}")
        return 1
    
    df = pd.read_csv(args.input)
    
    if len(df) == 0:
        print("Error: No data in CSV file")
        return 1
    
    generate_report(df, args.output, args.title)
    
    return 0


if __name__ == '__main__':
    exit(main())

