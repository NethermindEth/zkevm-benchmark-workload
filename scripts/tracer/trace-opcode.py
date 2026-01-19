import json
from typing import Dict, List, Tuple, Optional

# Standard Ethereum precompiles (address -> name)
STANDARD_PRECOMPILES = {
    0x01: "ECRECOVER",
    0x02: "SHA256",
    0x03: "RIPEMD160",
    0x04: "IDENTITY",
    0x05: "MODEXP",
    0x06: "ECADD",
    0x07: "ECMUL",
    0x08: "ECPAIRING",
    0x09: "BLAKE2F",
    # Berlin hardfork additions
    0x0a: "BLS12_G1ADD",
    0x0b: "BLS12_G1MUL", 
    0x0c: "BLS12_G1MULTIEXP",
    0x0d: "BLS12_G2ADD",
    0x0e: "BLS12_G2MUL",
    0x0f: "BLS12_G2MULTIEXP",
    0x10: "BLS12_PAIRING",
    0x11: "BLS12_MAP_FP_TO_G1",
    0x12: "BLS12_MAP_FP2_TO_G2",
    # Cancun hardfork
    0x13: "POINT_EVALUATION",
    # Chain-specific or test precompiles
    0x14: "SHA256_PARALLEL",  # Some forks
}

# Common chain-specific precompiles
CHAIN_SPECIFIC_PRECOMPILES = {
    # Polygon (formerly Matic)
    0x64: "POLYGON_VALIDATOR",
    0x65: "POLYGON_STATE_SYNC",
    
    # Avalanche
    0x0100000000000000000000000000000000000000: "AVAX_NATIVE_MINT",
    
    # Arbitrum
    0x6c: "ARB_SYS",
    0x6e: "ARB_INFO",
    0x70: "ARB_ADDRESS_TABLE",
    
    # Optimism
    0x4200000000000000000000000000000000000000: "OP_L1_BLOCK",
    0x4200000000000000000000000000000000000002: "OP_L2_TO_L1_MESSAGE",
    
    # Base
    0x4200000000000000000000000000000000000015: "BASE_L1_BLOCK",
}

class PrecompileAnalyzer:
    def __init__(self, trace_data: dict):
        self.trace = trace_data
        self.precompile_calls = []
        self.memory_state = []
        
    def analyze_trace(self):
        """Main analysis function"""
        if 'trace' not in self.trace or 'structLogs' not in self.trace['trace']:
            return []
            
        struct_logs = self.trace['trace']['structLogs']
        
        for i, log_entry in enumerate(struct_logs):
            opcode = log_entry.get('op', '')
            
            # Track memory state
            if 'memory' in log_entry:
                self.memory_state = log_entry['memory']
            
            # Look for CALL, STATICCALL, DELEGATECALL, CALLCODE
            if opcode in ['CALL', 'STATICCALL', 'DELEGATECALL', 'CALLCODE']:
                self._analyze_call_opcode(log_entry, i)
                
        return self.precompile_calls
    
    def _analyze_call_opcode(self, log_entry: dict, index: int):
        """Analyze a call opcode to identify precompiles"""
        stack = log_entry.get('stack', [])
        
        if len(stack) < 6:  # Need at least 6 items for CALL/STATICCALL
            return
            
        # Stack layout varies by opcode:
        # CALL: gas, addr, value, inOffset, inSize, outOffset, outSize (7 items)
        # STATICCALL: gas, addr, inOffset, inSize, outOffset, outSize (6 items)
        # DELEGATECALL: gas, addr, inOffset, inSize, outOffset, outSize (6 items)
        # CALLCODE: same as CALL
        
        opcode = log_entry['op']
        
        if opcode == 'CALL' or opcode == 'CALLCODE':
            if len(stack) < 7:
                return
            # For CALL: stack[-7] = gas, stack[-6] = addr, stack[-5] = value, 
            # stack[-4] = inOffset, stack[-3] = inSize, stack[-2] = outOffset, stack[-1] = outSize
            addr_hex = stack[-6]
            in_offset_hex = stack[-4]
            in_size_hex = stack[-3]
        else:  # STATICCALL or DELEGATECALL
            # stack[-6] = gas, stack[-5] = addr, stack[-4] = inOffset, 
            # stack[-3] = inSize, stack[-2] = outOffset, stack[-1] = outSize
            addr_hex = stack[-5]
            in_offset_hex = stack[-4]
            in_size_hex = stack[-3]
        
        # Parse address
        try:
            addr_int = int(addr_hex, 16)
        except ValueError:
            return
            
        # Check if it's a precompile address
        if addr_int == 0:
            return  # Not a precompile
            
        # Parse input parameters
        in_offset = int(in_offset_hex, 16) if in_offset_hex else 0
        in_size = int(in_size_hex, 16) if in_size_hex else 0
        
        # Get input data from memory
        input_data = self._extract_input_data(in_offset, in_size)
        
        # Identify the precompile
        precompile_info = self._identify_precompile(addr_int, input_data)
        
        # Store the call info
        call_info = {
            'pc': log_entry.get('pc', 0),
            'opcode': opcode,
            'address': f"0x{addr_int:02x}" if addr_int < 256 else f"0x{addr_int:x}",
            'address_decimal': addr_int,
            'input_offset': in_offset,
            'input_size': in_size,
            'input_data': input_data,
            'gas_used': log_entry.get('gasCost', 0),
            'depth': log_entry.get('depth', 1),
            'identified_as': precompile_info,
            'is_standard_precompile': addr_int in STANDARD_PRECOMPILES,
            'is_chain_specific': addr_int in CHAIN_SPECIFIC_PRECOMPILES,
        }
        
        self.precompile_calls.append(call_info)
    
    def _extract_input_data(self, offset: int, size: int) -> str:
        """Extract input data from memory at given offset"""
        if not self.memory_state or size == 0:
            return "0x"
        
        # Convert memory array to continuous hex string
        full_memory = ''.join(self.memory_state)
        
        # Calculate hex string positions (2 hex chars per byte)
        start = offset * 2
        end = start + (size * 2)
        
        if start >= len(full_memory):
            return "0x"
            
        end = min(end, len(full_memory))
        return f"0x{full_memory[start:end]}"
    
    def _identify_precompile(self, address: int, input_data: str) -> str:
        """Identify what precompile this is based on address and input"""
        # Check standard precompiles
        if address in STANDARD_PRECOMPILES:
            name = STANDARD_PRECOMPILES[address]
            
            # Add more specific identification based on input
            if address == 0x01:  # ECRECOVER
                if len(input_data) >= 130:  # 32+32+32+1+32+1 = 130 hex chars
                    return f"{name} (signature recovery)"
                else:
                    return f"{name} (invalid input length)"
                    
            elif address == 0x02:  # SHA256
                return f"{name} (hash function)"
                
            elif address == 0x03:  # RIPEMD160
                return f"{name} (hash function)"
                
            elif address == 0x04:  # IDENTITY
                return f"{name} (data copy)"
                
            elif address == 0x05:  # MODEXP
                return f"{name} (modular exponentiation)"
                
            elif address == 0x08:  # ECPAIRING
                return f"{name} (elliptic curve pairing)"
                
            return name
        
        # Check chain-specific precompiles
        if address in CHAIN_SPECIFIC_PRECOMPILES:
            return f"Chain-specific: {CHAIN_SPECIFIC_PRECOMPILES[address]}"
        
        # Analyze based on address range
        if 1 <= address <= 9:
            return f"Standard precompile range (unmapped: {address})"
        elif 10 <= address <= 255:
            # Could be experimental or chain-specific
            return f"Extended precompile address: {address}"
        elif address >= 256:
            # Large addresses are usually not precompiles but contracts
            # But some chains use large addresses for system contracts
            if address > 0xffffffffffffffffffffffffffffffffffffffff:
                return "Invalid address (too large)"
            return f"Potential system contract: {address}"
        
        return "Unknown"
    
    def print_analysis(self):
        """Print formatted analysis results"""
        if not self.precompile_calls:
            print("No precompile calls found in trace.")
            return
            
        print(f"\n{'='*60}")
        print(f"Precompile Call Analysis")
        print(f"{'='*60}")
        
        for i, call in enumerate(self.precompile_calls):
            print(f"\nCall #{i+1}:")
            print(f"  PC: {call['pc']}")
            print(f"  Opcode: {call['opcode']}")
            print(f"  Address: {call['address']} (decimal: {call['address_decimal']})")
            print(f"  Identification: {call['identified_as']}")
            print(f"  Input size: {call['input_size']} bytes")
            print(f"  Input data: {call['input_data'][:100]}..." if len(call['input_data']) > 100 else f"  Input data: {call['input_data']}")
            print(f"  Gas used: {call['gas_used']}")
            print(f"  Call depth: {call['depth']}")
            print(f"  Standard precompile: {call['is_standard_precompile']}")
            print(f"  Chain-specific: {call['is_chain_specific']}")
        
        print(f"\n{'='*60}")
        print(f"Summary: {len(self.precompile_calls)} precompile calls found")
        
        # Group by address
        address_counts = {}
        for call in self.precompile_calls:
            addr = call['address']
            address_counts[addr] = address_counts.get(addr, 0) + 1
        
        if address_counts:
            print("\nCalls by address:")
            for addr, count in address_counts.items():
                print(f"  {addr}: {count} call(s)")
        
        print(f"{'='*60}")


def analyze_trace_file(filename: str):
    """Analyze a trace from a JSON file"""
    with open(filename, 'r') as f:
        data = json.load(f)
    
    analyzer = PrecompileAnalyzer(data)
    calls = analyzer.analyze_trace()
    analyzer.print_analysis()
    return calls


def analyze_trace_jsonl(filename: str):
    """Analyze a trace from JSON Lines format (like in your example)"""
    calls = []
    
    with open(filename, 'r') as f:
        for line in f:
            if not line.strip():
                continue
                
            data = json.loads(line)
            if data.get('type') == 'transaction_trace':
                analyzer = PrecompileAnalyzer(data)
                calls.extend(analyzer.analyze_trace())
                analyzer.print_analysis()
    
    return calls


def interactive_analysis():
    """Interactive analysis for pasted trace data"""
    print("Paste your trace JSON (end with empty line or Ctrl+D):")
    lines = []
    
    try:
        while True:
            line = input()
            if line.strip() == '':
                break
            lines.append(line)
    except EOFError:
        pass
    
    if not lines:
        print("No input provided.")
        return
    
    trace_json = '\n'.join(lines)
    
    try:
        data = json.loads(trace_json)
        analyzer = PrecompileAnalyzer(data)
        calls = analyzer.analyze_trace()
        analyzer.print_analysis()
    except json.JSONDecodeError:
        print("Invalid JSON provided.")
        return


if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1:
        filename = sys.argv[1]
        if filename.endswith('.jsonl'):
            analyze_trace_jsonl(filename)
        else:
            analyze_trace_file(filename)
    else:
        print("Usage: python precompile_analyzer.py <trace_file.json>")
        print("\nOr run interactive mode:")
        interactive_analysis()