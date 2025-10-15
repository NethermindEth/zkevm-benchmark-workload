#!/usr/bin/env python3
"""
Script to simplify test file names by extracting and using only the additional parameters
from the complex naming structure.

Current pattern: test_file.py::test_function[fork_Prague-benchmark-gas-value_1M-blockchain_test-{additional_params}].json
Simplified pattern: {category}_{additional_params}.json
"""

import os
import re
import json
from pathlib import Path
from typing import Dict, List, Tuple, Optional


class TestNameSimplifier:
    def __init__(self):
        # Mapping of test categories to their prefixes
        self.category_prefixes = {
            'test_worst_blocks.py': 'blocks',
            'test_worst_bytecode.py': 'bytecode', 
            'test_worst_compute.py': 'compute',
            'test_worst_memory.py': 'memory',
            'test_worst_opcode.py': 'opcode',
            'test_worst_stateful_opcodes.py': 'stateful'
        }
        
        # Common parameter patterns to simplify
        self.param_simplifications = {
            'fork_Prague': '',
            'benchmark-gas-value_1M': '',
            'blockchain_test': '',
            'blockchain_test_from_state_test': '',
            'big_memory_expansion_True': 'big_mem',
            'big_memory_expansion_False': 'small_mem',
            'offset_initialized_True': 'init_offset',
            'offset_initialized_False': 'uninit_offset',
            'non_zero_data_True': 'non_zero',
            'non_zero_data_False': 'zero_data',
            'fixed_offset_True': 'fixed',
            'fixed_offset_False': 'dynamic',
            'zeros_topic': 'zero_topic',
            'non_zero_topic': 'non_zero_topic',
            'value_bearing_True': 'with_value',
            'value_bearing_False': 'no_value',
            'absent_target_True': 'absent_target',
            'absent_target_False': 'present_target',
            'fixed_src_dst_True': 'fixed_src_dst',
            'fixed_src_dst_False': 'dynamic_src_dst',
            'zero_byte_True': 'zero_byte',
            'zero_byte_False': 'non_zero_byte'
        }
    
    def extract_parameters(self, filename: str) -> Tuple[str, str, List[str]]:
        """
        Extract test category, function, and parameters from filename.
        
        Returns:
            (category, function, parameters)
        """
        # Remove .json extension
        name = filename.replace('.json', '')
        
        # Split on :: to get test_file and test_function[params]
        if '::' not in name:
            return '', '', []
            
        test_file, rest = name.split('::', 1)
        
        # Extract function name and parameters
        if '[' in rest and ']' in rest:
            function_part = rest[:rest.index('[')]
            params_part = rest[rest.index('[')+1:rest.rindex(']')]
            # Split on '-' but be careful with benchmark-gas-value which contains hyphens
            parameters = []
            current_param = ""
            i = 0
            while i < len(params_part):
                if params_part[i] == '-':
                    # Check if this is part of "benchmark-gas-value"
                    if params_part[i:i+8] == '-gas-value':
                        current_param += params_part[i]
                    else:
                        # This is a parameter separator
                        if current_param.strip():
                            parameters.append(current_param.strip())
                        current_param = ""
                else:
                    current_param += params_part[i]
                i += 1
            if current_param.strip():
                parameters.append(current_param.strip())
        else:
            function_part = rest
            parameters = []
        
        return test_file, function_part, parameters
    
    def simplify_parameters(self, parameters: List[str]) -> List[str]:
        """
        Simplify parameter names by removing common prefixes and applying mappings.
        """
        simplified = []
        
        for param in parameters:
            # Skip common parameters that don't add value
            if param in ['fork_Prague', 'benchmark-gas-value_1M', 'blockchain_test', 'blockchain_test_from_state_test']:
                continue
            # Also skip any parameter that starts with benchmark-gas-value
            if param.startswith('benchmark-gas-value'):
                continue
            # Skip individual parts of benchmark-gas-value_1M
            if param in ['benchmark', 'gas', 'value_1M', '1M']:
                continue
                
            # Apply simplifications
            if param in self.param_simplifications:
                simplified_param = self.param_simplifications[param]
                if simplified_param:  # Only add if not empty
                    simplified.append(simplified_param)
            else:
                # For parameters not in our mapping, try to extract meaningful parts
                if param.startswith('opcode_'):
                    simplified.append(param.replace('opcode_', ''))
                elif param.startswith('case_id_'):
                    simplified.append(param.replace('case_id_', ''))
                elif param.startswith('offset_'):
                    simplified.append(param.replace('offset_', 'off_'))
                elif param.startswith('size_'):
                    simplified.append(param.replace('size_', ''))
                elif param.startswith('data_'):
                    simplified.append(param.replace('data_', ''))
                elif param.startswith('value_'):
                    simplified.append(param.replace('value_', ''))
                elif param.startswith('benchmark-gas-value_'):
                    # Skip this common parameter
                    continue
                elif param.startswith('0 bytes'):
                    simplified.append('0bytes')
                elif param.startswith('100 bytes'):
                    simplified.append('100bytes')
                elif param.startswith('1 MiB'):
                    simplified.append('1MiB')
                elif param.startswith('0.25x max code size'):
                    simplified.append('0.25x_max_code')
                elif param.startswith('max code size'):
                    simplified.append('max_code')
                elif param.startswith('with value'):
                    simplified.append('with_value')
                elif param.startswith('without value'):
                    simplified.append('without_value')
                elif param.startswith('with non-zero data'):
                    simplified.append('non_zero_data')
                elif param.startswith('with zero data'):
                    simplified.append('zero_data')
                else:
                    # Keep the parameter as is, but make it shorter if possible
                    simplified.append(param)
        
        return simplified
    
    def generate_simplified_name(self, filename: str) -> str:
        """
        Generate a simplified filename based on the additional parameters.
        """
        test_file, function, parameters = self.extract_parameters(filename)
        
        # Get category prefix
        category = self.category_prefixes.get(test_file, 'unknown')
        
        # Simplify parameters
        simplified_params = self.simplify_parameters(parameters)
        
        # Create the new name
        if simplified_params:
            param_str = '_'.join(simplified_params)
            new_name = f"{category}_{param_str}.json"
        else:
            # Fallback to function name if no meaningful parameters
            function_clean = function.replace('test_worst_', '').replace('test_', '')
            new_name = f"{category}_{function_clean}.json"
        
        return new_name
    
    def process_directory(self, directory_path: str, dry_run: bool = True) -> Dict[str, str]:
        """
        Process all JSON files in a directory and generate simplified names.
        
        Args:
            directory_path: Path to directory containing test files
            dry_run: If True, only show what would be renamed without actually renaming
            
        Returns:
            Dictionary mapping original names to simplified names
        """
        directory = Path(directory_path)
        if not directory.exists():
            print(f"Directory {directory_path} does not exist")
            return {}
        
        json_files = list(directory.glob("*.json"))
        if not json_files:
            print(f"No JSON files found in {directory_path}")
            return {}
        
        rename_mapping = {}
        
        print(f"Found {len(json_files)} JSON files to process")
        print("\nProposed renames:")
        print("-" * 80)
        
        for file_path in json_files:
            original_name = file_path.name
            simplified_name = self.generate_simplified_name(original_name)
            rename_mapping[original_name] = simplified_name
            
            print(f"Original: {original_name}")
            print(f"Simplified: {simplified_name}")
            print("-" * 80)
        
        if not dry_run:
            # Actually perform the renames
            for original, simplified in rename_mapping.items():
                original_path = directory / original
                simplified_path = directory / simplified
                
                if simplified_path.exists():
                    print(f"Warning: {simplified} already exists, skipping {original}")
                    continue
                    
                original_path.rename(simplified_path)
                print(f"Renamed: {original} -> {simplified}")
        
        return rename_mapping
    
    def create_rename_script(self, directory_path: str, output_script: str = "rename_tests.sh"):
        """
        Create a shell script that can be used to rename the files.
        """
        rename_mapping = self.process_directory(directory_path, dry_run=True)
        
        script_content = ["#!/bin/bash", "# Auto-generated script to rename test files", ""]
        
        for original, simplified in rename_mapping.items():
            script_content.append(f'mv "{original}" "{simplified}"')
        
        with open(output_script, 'w') as f:
            f.write('\n'.join(script_content))
        
        # Make the script executable
        os.chmod(output_script, 0o700)
        
        print(f"\nRename script created: {output_script}")
        print("Run it with: ./rename_tests.sh")


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Simplify test file names based on parameters")
    parser.add_argument("directory", help="Directory containing test JSON files")
    parser.add_argument("--dry-run", action="store_true", default=True, 
                       help="Show what would be renamed without actually renaming")
    parser.add_argument("--execute", action="store_true", 
                       help="Actually perform the renames (overrides --dry-run)")
    parser.add_argument("--create-script", action="store_true",
                       help="Create a shell script for renaming")
    
    args = parser.parse_args()
    
    simplifier = TestNameSimplifier()
    
    if args.create_script:
        simplifier.create_rename_script(args.directory)
    else:
        dry_run = args.dry_run and not args.execute
        simplifier.process_directory(args.directory, dry_run=dry_run)


if __name__ == "__main__":
    main()
