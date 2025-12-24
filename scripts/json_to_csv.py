#!/usr/bin/env python3
"""
Convert raw JSON benchmark results to CSV format.

Usage:
    python json_to_csv.py --input /path/to/results-dir --output results.csv
"""

import argparse
import json
import re
from pathlib import Path
from typing import Dict, Optional

import pandas as pd


def parse_test_name(name: str) -> Optional[Dict]:
    """
    Parse test name to extract opcode and op_count.
    
    Supports formats:
    - test_worstcase_bytecode[...-OPCODE_WC_COUNT]
    - test_precompile_worstcase_bytecode[...-OPCODE_WC_COUNT]
    - test_gas_cost_bytecode[...-OPCODE_COUNT]
    """
    # Pattern for worst-case tests (opcodes and precompiles)
    match = re.search(r'-([A-Z][A-Z0-9_]+)_WC_(\d+)\]', name)
    if match:
        return {'opcode': f"{match.group(1)}_WC", 'op_count': int(match.group(2))}
    
    # Pattern for regular tests
    match = re.search(r'-([A-Z][A-Z0-9]+)_(\d+)\]', name)
    if match:
        return {'opcode': match.group(1), 'op_count': int(match.group(2))}
    
    return None


def process_json_file(json_path: Path, sample_id: int) -> Optional[Dict]:
    """Process a single JSON result file."""
    try:
        with open(json_path) as f:
            data = json.load(f)
    except (json.JSONDecodeError, IOError) as e:
        print(f"Warning: Could not read {json_path}: {e}")
        return None
    
    # Parse test name
    test_name = data.get('name', json_path.stem)
    parsed = parse_test_name(test_name)
    if not parsed:
        return None
    
    # Extract metrics from execution or proving results
    execution = data.get('execution', {})
    proving = data.get('proving', {}) or data.get('prove', {})
    
    # Try execution first, then proving
    success = execution.get('success', {}) or proving.get('success', {})
    
    if not success:
        return None
    
    # Extract cycles
    cycles = success.get('total_num_cycles')
    
    # Extract execution duration
    exec_duration = success.get('execution_duration', {})
    if exec_duration:
        exec_time_ns = exec_duration.get('secs', 0) * 1e9 + exec_duration.get('nanos', 0)
    else:
        exec_time_ns = None
    
    # Extract prove duration (if available)
    # Can be in prove_duration {secs, nanos} or proving_time_ms format
    prove_duration = success.get('prove_duration', {})
    proving_time_ms = success.get('proving_time_ms')
    if prove_duration:
        prove_time_ns = prove_duration.get('secs', 0) * 1e9 + prove_duration.get('nanos', 0)
    elif proving_time_ms is not None:
        prove_time_ns = proving_time_ms * 1e6  # ms to ns
    else:
        prove_time_ns = None
    
    # Extract memory usage (if available)
    peak_memory_bytes = success.get('peak_memory_bytes')
    
    return {
        'opcode': parsed['opcode'],
        'op_count': parsed['op_count'],
        'sample_id': sample_id,
        'total_num_cycles': cycles,
        'execution_time_ns': exec_time_ns,
        'prove_time_ns': prove_time_ns,
        'peak_memory_bytes': peak_memory_bytes,
    }


def load_benchmark_results(results_dir: Path) -> pd.DataFrame:
    """
    Load all benchmark results from a directory structure.
    
    Expected structure:
        results_dir/
            run_1/
                reth/sp1-*/
                    *.json
            run_2/
                ...
    """
    records = []
    
    for run_dir in sorted(results_dir.glob('run_*')):
        run_match = re.search(r'run_(\d+)', run_dir.name)
        if not run_match:
            continue
        run_num = int(run_match.group(1))
        
        # Find JSON files in various possible locations
        json_files = list(run_dir.glob('**/*.json'))
        
        for json_file in json_files:
            # Skip log files
            if 'log' in json_file.name.lower():
                continue
                
            record = process_json_file(json_file, run_num)
            if record:
                records.append(record)
    
    return pd.DataFrame(records)


def main():
    parser = argparse.ArgumentParser(
        description='Convert JSON benchmark results to CSV'
    )
    parser.add_argument(
        '--input', '-i',
        type=Path,
        required=True,
        help='Directory containing benchmark results (with run_1/, run_2/, etc.)'
    )
    parser.add_argument(
        '--output', '-o',
        type=Path,
        default=Path('results.csv'),
        help='Output CSV file path'
    )
    
    args = parser.parse_args()
    
    if not args.input.exists():
        print(f"Error: Input directory not found: {args.input}")
        return 1
    
    df = load_benchmark_results(args.input)
    
    if len(df) == 0:
        print("Error: No benchmark results found")
        return 1
    
    # Sort by opcode and op_count
    df = df.sort_values(['opcode', 'op_count', 'sample_id'])
    
    # Save to CSV
    df.to_csv(args.output, index=False)
    
    print(f"Processed {len(df)} records from {df['opcode'].nunique()} opcodes/precompiles")
    
    return 0


if __name__ == '__main__':
    exit(main())

