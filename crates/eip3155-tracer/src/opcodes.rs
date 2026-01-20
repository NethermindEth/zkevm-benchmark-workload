//! EVM opcode definitions and name mappings.
//!
//! This module provides complete opcode byte-to-name mappings for all EVM opcodes
//! (0x00-0xFF) as well as precompile address-to-name mappings.

/// Get human-readable opcode name from byte value.
///
/// Returns the standard EVM opcode name for the given byte value.
/// Unknown opcodes return "UNKNOWN".
///
/// # Examples
///
/// ```
/// use eip3155_tracer::opcodes::get_opcode_name;
///
/// assert_eq!(get_opcode_name(0x01), "ADD");
/// assert_eq!(get_opcode_name(0x60), "PUSH1");
/// assert_eq!(get_opcode_name(0xfa), "STATICCALL");
/// ```
pub fn get_opcode_name(opcode: u8) -> &'static str {
    match opcode {
        // Stop and Arithmetic Operations (0x00-0x0b)
        0x00 => "STOP",
        0x01 => "ADD",
        0x02 => "MUL",
        0x03 => "SUB",
        0x04 => "DIV",
        0x05 => "SDIV",
        0x06 => "MOD",
        0x07 => "SMOD",
        0x08 => "ADDMOD",
        0x09 => "MULMOD",
        0x0a => "EXP",
        0x0b => "SIGNEXTEND",

        // Comparison & Bitwise Logic Operations (0x10-0x1d)
        0x10 => "LT",
        0x11 => "GT",
        0x12 => "SLT",
        0x13 => "SGT",
        0x14 => "EQ",
        0x15 => "ISZERO",
        0x16 => "AND",
        0x17 => "OR",
        0x18 => "XOR",
        0x19 => "NOT",
        0x1a => "BYTE",
        0x1b => "SHL",
        0x1c => "SHR",
        0x1d => "SAR",

        // SHA3 (0x20)
        0x20 => "SHA3",

        // Environmental Information (0x30-0x3f)
        0x30 => "ADDRESS",
        0x31 => "BALANCE",
        0x32 => "ORIGIN",
        0x33 => "CALLER",
        0x34 => "CALLVALUE",
        0x35 => "CALLDATALOAD",
        0x36 => "CALLDATASIZE",
        0x37 => "CALLDATACOPY",
        0x38 => "CODESIZE",
        0x39 => "CODECOPY",
        0x3a => "GASPRICE",
        0x3b => "EXTCODESIZE",
        0x3c => "EXTCODECOPY",
        0x3d => "RETURNDATASIZE",
        0x3e => "RETURNDATACOPY",
        0x3f => "EXTCODEHASH",

        // Block Information (0x40-0x4a)
        0x40 => "BLOCKHASH",
        0x41 => "COINBASE",
        0x42 => "TIMESTAMP",
        0x43 => "NUMBER",
        0x44 => "PREVRANDAO", // Previously DIFFICULTY
        0x45 => "GASLIMIT",
        0x46 => "CHAINID",
        0x47 => "SELFBALANCE",
        0x48 => "BASEFEE",
        0x49 => "BLOBHASH",
        0x4a => "BLOBBASEFEE",

        // Stack, Memory, Storage and Flow Operations (0x50-0x5b)
        0x50 => "POP",
        0x51 => "MLOAD",
        0x52 => "MSTORE",
        0x53 => "MSTORE8",
        0x54 => "SLOAD",
        0x55 => "SSTORE",
        0x56 => "JUMP",
        0x57 => "JUMPI",
        0x58 => "PC",
        0x59 => "MSIZE",
        0x5a => "GAS",
        0x5b => "JUMPDEST",

        // Transient Storage (EIP-1153)
        0x5c => "TLOAD",
        0x5d => "TSTORE",

        // Memory Copy (EIP-5656)
        0x5e => "MCOPY",

        // Push Operations (0x5f-0x7f)
        0x5f => "PUSH0",
        0x60 => "PUSH1",
        0x61 => "PUSH2",
        0x62 => "PUSH3",
        0x63 => "PUSH4",
        0x64 => "PUSH5",
        0x65 => "PUSH6",
        0x66 => "PUSH7",
        0x67 => "PUSH8",
        0x68 => "PUSH9",
        0x69 => "PUSH10",
        0x6a => "PUSH11",
        0x6b => "PUSH12",
        0x6c => "PUSH13",
        0x6d => "PUSH14",
        0x6e => "PUSH15",
        0x6f => "PUSH16",
        0x70 => "PUSH17",
        0x71 => "PUSH18",
        0x72 => "PUSH19",
        0x73 => "PUSH20",
        0x74 => "PUSH21",
        0x75 => "PUSH22",
        0x76 => "PUSH23",
        0x77 => "PUSH24",
        0x78 => "PUSH25",
        0x79 => "PUSH26",
        0x7a => "PUSH27",
        0x7b => "PUSH28",
        0x7c => "PUSH29",
        0x7d => "PUSH30",
        0x7e => "PUSH31",
        0x7f => "PUSH32",

        // Duplication Operations (0x80-0x8f)
        0x80 => "DUP1",
        0x81 => "DUP2",
        0x82 => "DUP3",
        0x83 => "DUP4",
        0x84 => "DUP5",
        0x85 => "DUP6",
        0x86 => "DUP7",
        0x87 => "DUP8",
        0x88 => "DUP9",
        0x89 => "DUP10",
        0x8a => "DUP11",
        0x8b => "DUP12",
        0x8c => "DUP13",
        0x8d => "DUP14",
        0x8e => "DUP15",
        0x8f => "DUP16",

        // Exchange Operations (0x90-0x9f)
        0x90 => "SWAP1",
        0x91 => "SWAP2",
        0x92 => "SWAP3",
        0x93 => "SWAP4",
        0x94 => "SWAP5",
        0x95 => "SWAP6",
        0x96 => "SWAP7",
        0x97 => "SWAP8",
        0x98 => "SWAP9",
        0x99 => "SWAP10",
        0x9a => "SWAP11",
        0x9b => "SWAP12",
        0x9c => "SWAP13",
        0x9d => "SWAP14",
        0x9e => "SWAP15",
        0x9f => "SWAP16",

        // Logging Operations (0xa0-0xa4)
        0xa0 => "LOG0",
        0xa1 => "LOG1",
        0xa2 => "LOG2",
        0xa3 => "LOG3",
        0xa4 => "LOG4",

        // System Operations (0xf0-0xff)
        0xf0 => "CREATE",
        0xf1 => "CALL",
        0xf2 => "CALLCODE",
        0xf3 => "RETURN",
        0xf4 => "DELEGATECALL",
        0xf5 => "CREATE2",
        0xfa => "STATICCALL",
        0xfd => "REVERT",
        0xfe => "INVALID",
        0xff => "SELFDESTRUCT",

        // Unknown/Undefined opcodes
        _ => "UNKNOWN",
    }
}

/// Check if an opcode is a CALL-type opcode that transitions frames.
///
/// These opcodes are important for gas validation as they transfer
/// execution to a child frame with different gas accounting.
pub fn is_call_opcode(opcode: u8) -> bool {
    matches!(opcode, 0xf1 | 0xf2 | 0xf4 | 0xfa)
}

/// Check if an opcode is a CREATE-type opcode.
pub fn is_create_opcode(opcode: u8) -> bool {
    matches!(opcode, 0xf0 | 0xf5)
}

/// Check if an opcode terminates execution in the current frame.
pub fn is_terminating_opcode(opcode: u8) -> bool {
    matches!(opcode, 0x00 | 0xf3 | 0xfd | 0xfe | 0xff)
}

/// Get human-readable precompile name from address.
///
/// Returns the standard precompile name for addresses 0x01-0x11.
/// Unknown addresses return "UNKNOWN".
pub fn get_precompile_name(address: u8) -> &'static str {
    match address {
        // Original precompiles (Frontier, Byzantium, Istanbul)
        0x01 => "ECRECOVER",
        0x02 => "SHA256",
        0x03 => "RIPEMD160",
        0x04 => "IDENTITY",
        0x05 => "MODEXP",
        0x06 => "ECADD",
        0x07 => "ECMUL",
        0x08 => "ECPAIRING",
        0x09 => "BLAKE2F",

        // Cancun precompile (EIP-4844)
        0x0a => "KZG_POINT_EVALUATION",

        // BLS12-381 precompiles (EIP-2537, Pectra)
        0x0b => "BLS12_G1ADD",
        0x0c => "BLS12_G1MSM",
        0x0d => "BLS12_G2ADD",
        0x0e => "BLS12_G2MSM",
        0x0f => "BLS12_PAIRING_CHECK",
        0x10 => "BLS12_MAP_FP_TO_G1",
        0x11 => "BLS12_MAP_FP2_TO_G2",

        _ => "UNKNOWN",
    }
}

/// Check if an address is a known precompile.
pub fn is_precompile(address: u8) -> bool {
    (0x01..=0x11).contains(&address)
}

/// Get opcode byte value from name.
///
/// Returns `None` if the opcode name is not recognized.
pub fn get_opcode_value(name: &str) -> Option<u8> {
    let upper = name.to_uppercase();
    match upper.as_str() {
        "STOP" => Some(0x00),
        "ADD" => Some(0x01),
        "MUL" => Some(0x02),
        "SUB" => Some(0x03),
        "DIV" => Some(0x04),
        "SDIV" => Some(0x05),
        "MOD" => Some(0x06),
        "SMOD" => Some(0x07),
        "ADDMOD" => Some(0x08),
        "MULMOD" => Some(0x09),
        "EXP" => Some(0x0a),
        "SIGNEXTEND" => Some(0x0b),
        "LT" => Some(0x10),
        "GT" => Some(0x11),
        "SLT" => Some(0x12),
        "SGT" => Some(0x13),
        "EQ" => Some(0x14),
        "ISZERO" => Some(0x15),
        "AND" => Some(0x16),
        "OR" => Some(0x17),
        "XOR" => Some(0x18),
        "NOT" => Some(0x19),
        "BYTE" => Some(0x1a),
        "SHL" => Some(0x1b),
        "SHR" => Some(0x1c),
        "SAR" => Some(0x1d),
        "SHA3" | "KECCAK256" => Some(0x20),
        "ADDRESS" => Some(0x30),
        "BALANCE" => Some(0x31),
        "ORIGIN" => Some(0x32),
        "CALLER" => Some(0x33),
        "CALLVALUE" => Some(0x34),
        "CALLDATALOAD" => Some(0x35),
        "CALLDATASIZE" => Some(0x36),
        "CALLDATACOPY" => Some(0x37),
        "CODESIZE" => Some(0x38),
        "CODECOPY" => Some(0x39),
        "GASPRICE" => Some(0x3a),
        "EXTCODESIZE" => Some(0x3b),
        "EXTCODECOPY" => Some(0x3c),
        "RETURNDATASIZE" => Some(0x3d),
        "RETURNDATACOPY" => Some(0x3e),
        "EXTCODEHASH" => Some(0x3f),
        "BLOCKHASH" => Some(0x40),
        "COINBASE" => Some(0x41),
        "TIMESTAMP" => Some(0x42),
        "NUMBER" => Some(0x43),
        "PREVRANDAO" | "DIFFICULTY" => Some(0x44),
        "GASLIMIT" => Some(0x45),
        "CHAINID" => Some(0x46),
        "SELFBALANCE" => Some(0x47),
        "BASEFEE" => Some(0x48),
        "BLOBHASH" => Some(0x49),
        "BLOBBASEFEE" => Some(0x4a),
        "POP" => Some(0x50),
        "MLOAD" => Some(0x51),
        "MSTORE" => Some(0x52),
        "MSTORE8" => Some(0x53),
        "SLOAD" => Some(0x54),
        "SSTORE" => Some(0x55),
        "JUMP" => Some(0x56),
        "JUMPI" => Some(0x57),
        "PC" => Some(0x58),
        "MSIZE" => Some(0x59),
        "GAS" => Some(0x5a),
        "JUMPDEST" => Some(0x5b),
        "TLOAD" => Some(0x5c),
        "TSTORE" => Some(0x5d),
        "MCOPY" => Some(0x5e),
        "PUSH0" => Some(0x5f),
        "PUSH1" => Some(0x60),
        "PUSH2" => Some(0x61),
        "PUSH3" => Some(0x62),
        "PUSH4" => Some(0x63),
        "PUSH5" => Some(0x64),
        "PUSH6" => Some(0x65),
        "PUSH7" => Some(0x66),
        "PUSH8" => Some(0x67),
        "PUSH9" => Some(0x68),
        "PUSH10" => Some(0x69),
        "PUSH11" => Some(0x6a),
        "PUSH12" => Some(0x6b),
        "PUSH13" => Some(0x6c),
        "PUSH14" => Some(0x6d),
        "PUSH15" => Some(0x6e),
        "PUSH16" => Some(0x6f),
        "PUSH17" => Some(0x70),
        "PUSH18" => Some(0x71),
        "PUSH19" => Some(0x72),
        "PUSH20" => Some(0x73),
        "PUSH21" => Some(0x74),
        "PUSH22" => Some(0x75),
        "PUSH23" => Some(0x76),
        "PUSH24" => Some(0x77),
        "PUSH25" => Some(0x78),
        "PUSH26" => Some(0x79),
        "PUSH27" => Some(0x7a),
        "PUSH28" => Some(0x7b),
        "PUSH29" => Some(0x7c),
        "PUSH30" => Some(0x7d),
        "PUSH31" => Some(0x7e),
        "PUSH32" => Some(0x7f),
        "DUP1" => Some(0x80),
        "DUP2" => Some(0x81),
        "DUP3" => Some(0x82),
        "DUP4" => Some(0x83),
        "DUP5" => Some(0x84),
        "DUP6" => Some(0x85),
        "DUP7" => Some(0x86),
        "DUP8" => Some(0x87),
        "DUP9" => Some(0x88),
        "DUP10" => Some(0x89),
        "DUP11" => Some(0x8a),
        "DUP12" => Some(0x8b),
        "DUP13" => Some(0x8c),
        "DUP14" => Some(0x8d),
        "DUP15" => Some(0x8e),
        "DUP16" => Some(0x8f),
        "SWAP1" => Some(0x90),
        "SWAP2" => Some(0x91),
        "SWAP3" => Some(0x92),
        "SWAP4" => Some(0x93),
        "SWAP5" => Some(0x94),
        "SWAP6" => Some(0x95),
        "SWAP7" => Some(0x96),
        "SWAP8" => Some(0x97),
        "SWAP9" => Some(0x98),
        "SWAP10" => Some(0x99),
        "SWAP11" => Some(0x9a),
        "SWAP12" => Some(0x9b),
        "SWAP13" => Some(0x9c),
        "SWAP14" => Some(0x9d),
        "SWAP15" => Some(0x9e),
        "SWAP16" => Some(0x9f),
        "LOG0" => Some(0xa0),
        "LOG1" => Some(0xa1),
        "LOG2" => Some(0xa2),
        "LOG3" => Some(0xa3),
        "LOG4" => Some(0xa4),
        "CREATE" => Some(0xf0),
        "CALL" => Some(0xf1),
        "CALLCODE" => Some(0xf2),
        "RETURN" => Some(0xf3),
        "DELEGATECALL" => Some(0xf4),
        "CREATE2" => Some(0xf5),
        "STATICCALL" => Some(0xfa),
        "REVERT" => Some(0xfd),
        "INVALID" => Some(0xfe),
        "SELFDESTRUCT" => Some(0xff),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_names() {
        assert_eq!(get_opcode_name(0x00), "STOP");
        assert_eq!(get_opcode_name(0x01), "ADD");
        assert_eq!(get_opcode_name(0x02), "MUL");
        assert_eq!(get_opcode_name(0x20), "SHA3");
        assert_eq!(get_opcode_name(0x5f), "PUSH0");
        assert_eq!(get_opcode_name(0x60), "PUSH1");
        assert_eq!(get_opcode_name(0x7f), "PUSH32");
        assert_eq!(get_opcode_name(0x80), "DUP1");
        assert_eq!(get_opcode_name(0x8f), "DUP16");
        assert_eq!(get_opcode_name(0x90), "SWAP1");
        assert_eq!(get_opcode_name(0x9f), "SWAP16");
        assert_eq!(get_opcode_name(0xf1), "CALL");
        assert_eq!(get_opcode_name(0xfa), "STATICCALL");
        assert_eq!(get_opcode_name(0xfd), "REVERT");
    }

    #[test]
    fn test_precompile_names() {
        assert_eq!(get_precompile_name(0x01), "ECRECOVER");
        assert_eq!(get_precompile_name(0x05), "MODEXP");
        assert_eq!(get_precompile_name(0x09), "BLAKE2F");
        assert_eq!(get_precompile_name(0x0a), "KZG_POINT_EVALUATION");
    }

    #[test]
    fn test_is_call_opcode() {
        assert!(is_call_opcode(0xf1)); // CALL
        assert!(is_call_opcode(0xf2)); // CALLCODE
        assert!(is_call_opcode(0xf4)); // DELEGATECALL
        assert!(is_call_opcode(0xfa)); // STATICCALL
        assert!(!is_call_opcode(0xf0)); // CREATE
        assert!(!is_call_opcode(0x01)); // ADD
    }

    #[test]
    fn test_opcode_value_lookup() {
        assert_eq!(get_opcode_value("ADD"), Some(0x01));
        assert_eq!(get_opcode_value("PUSH1"), Some(0x60));
        assert_eq!(get_opcode_value("STATICCALL"), Some(0xfa));
        assert_eq!(get_opcode_value("add"), Some(0x01)); // Case insensitive
        assert_eq!(get_opcode_value("NOTANOPCODE"), None);
    }
}
