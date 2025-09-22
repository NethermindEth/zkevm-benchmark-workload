# zkEVM Benchmark Results

Generated on: 2025-09-22 13:42:53

Comparing 2 metrics folders:
- zkevm-metrics-risc0-1M (Gas: 1M)
- zkevm-metrics-sp1-1M (Gas: 1M)

## Gas Category: 1M

Source: zkevm-metrics-risc0-1M

## Proving Metrics

| Benchmark | Gas Category | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) |
|---|---|---|---|---|
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_EXTCODESIZE] |  | 223,662 | 345,054.0 | 345.05 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0 bytes with value-opcode_CREATE] |  | 223,662 | 6,944.0 | 6.94 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0 bytes without value-opcode_CREATE2] |  | 223,662 | 6,906.0 | 6.91 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SUB-] |  | 223,662 | 42,189.0 | 42.19 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_calldataload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-zero-loop] |  | 223,662 | 97,786.0 | 97.79 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_calldatasize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-calldata_length_10000] |  | 223,662 | 29,477.0 | 29.48 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_callvalue[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-from_origin_True-non_zero_value_True] |  | 223,662 | 28,147.0 | 28.15 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_jumps[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test] |  | 223,662 | 22,223.0 | 22.22 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_0-opcode_MLOAD] |  | 223,662 | 45,316.0 | 45.32 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_1-opcode_MLOAD] |  | 223,662 | 51,779.0 | 51.78 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_mod[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_SMOD-mod_bits_63] |  | 223,662 | 164,655.0 | 164.66 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_600_gas_exp_heavy] |  | 223,662 | 1,466,185.0 | 1,466.18 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_8b_exp_896] |  | 223,662 | 2,561,696.0 | 2,561.70 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_pawel_2] |  | 223,662 | 1,385,688.0 | 1,385.69 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_marius_1_even] |  | 223,662 | 1,042,709.0 | 1,042.71 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_3_pow_0x10001] |  | 223,662 | 744,491.0 | 744.49 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_pawel_1_exp_heavy] |  | 223,662 | 2,061,242.0 | 2,061.24 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_mul] |  | 223,662 | 359,648.0 | 359.65 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH20] |  | 223,662 | 54,134.0 | 54.13 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH25] |  | 223,662 | 64,835.0 | 64.83 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH28] |  | 223,662 | 59,734.0 | 59.73 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_tload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-val_mut_False-key_mut_False] |  | 223,662 | 9,700.0 | 9.70 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_True-100 bytes-call] |  | 223,662 | 29,394.0 | 29.39 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-0.50x max code size] |  | 223,662 | 22,069.0 | 22.07 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-max code size] |  | 223,662 | 24,120.0 | 24.12 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_True-0.75x max code size] |  | 223,662 | 15,282.0 | 15.28 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-0 bytes] |  | 223,662 | 34,837.0 | 34.84 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-1MiB] |  | 223,662 | 7,575.0 | 7.58 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_non_zero_data-log2] |  | 223,662 | 5,843.0 | 5.84 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-0_bytes_data-log2] |  | 223,662 | 7,615.0 | 7.62 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log1] |  | 223,662 | 5,726.0 | 5.73 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log3] |  | 223,662 | 6,060.0 | 6.06 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-0_bytes_data-log2] |  | 223,662 | 7,075.0 | 7.08 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-0_bytes_data-log4] |  | 223,662 | 6,586.0 | 6.59 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_non_zero_data-log3] |  | 223,662 | 5,643.0 | 5.64 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_blockhash[fork_Prague-benchmark-gas-value_1M-blockchain_test] |  | 223,662 | 42,904.0 | 42.90 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_extcodecopy_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-512] |  | 223,662 | 20,116.0 | 20.12 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_selfdestruct_created[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-value_bearing_False] |  | 223,662 | 6,279.0 | 6.28 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_True-SSTORE same value, revert] |  | 223,662 | 5,838.0 | 5.84 |

## Gas Category: 1M

Source: zkevm-metrics-sp1-1M

## Proving Metrics

| Benchmark | Gas Category | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) |
|---|---|---|---|---|
| sp1-v5.1.0/test_worst_blocks.py::test_block_full_data[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-zero_byte_False] |  | 1,477,259 | 11,017.0 | 11.02 |
| sp1-v5.1.0/test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_a_to_a] |  | 1,477,259 | 21,972.0 | 21.97 |
| sp1-v5.1.0/test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_a_to_b] |  | 1,477,259 | 20,403.0 | 20.40 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_EXTCODESIZE] |  | 1,477,259 | 107,333.0 | 107.33 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_STATICCALL] |  | 1,477,259 | 102,459.0 | 102.46 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0 bytes with value-opcode_CREATE] |  | 1,477,259 | 12,169.0 | 12.17 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0 bytes without value-opcode_CREATE2] |  | 1,477,259 | 11,473.0 | 11.47 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.25x max code size with non-zero data-opcode_CREATE2] |  | 1,477,259 | 11,736.0 | 11.74 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.75x max code size with non-zero data-opcode_CREATE2] |  | 1,477,259 | 12,011.0 | 12.01 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.75x max code size with non-zero data-opcode_CREATE] |  | 1,477,259 | 12,451.0 | 12.45 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.75x max code size with zero data-opcode_CREATE2] |  | 1,477,259 | 12,259.0 | 12.26 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.75x max code size with zero data-opcode_CREATE] |  | 1,477,259 | 11,645.0 | 11.64 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-max code size with zero data-opcode_CREATE] |  | 1,477,259 | 12,300.0 | 12.30 |
| sp1-v5.1.0/test_worst_bytecode.py::test_worst_initcode_jumpdest_analysis[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-00] |  | 1,477,259 | 19,829.0 | 19.83 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DIV-1] |  | 1,477,259 | 82,582.0 | 82.58 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_LT-] |  | 1,477,259 | 20,744.0 | 20.74 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_OR-] |  | 1,477,259 | 21,502.0 | 21.50 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SDIV-0] |  | 1,477,259 | 97,563.0 | 97.56 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SDIV-1] |  | 1,477,259 | 95,100.0 | 95.10 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SHL-] |  | 1,477,259 | 32,461.0 | 32.46 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SUB-] |  | 1,477,259 | 23,862.0 | 23.86 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_blobhash[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-no blobs] |  | 1,477,259 | 20,069.0 | 20.07 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_blobhash[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-one blob and accessed] |  | 1,477,259 | 27,979.0 | 27.98 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_blobhash[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-one blob but access non-existent index] |  | 1,477,259 | 20,762.0 | 20.76 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_calldataload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-zero-loop] |  | 1,477,259 | 40,811.0 | 40.81 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_calldatasize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-calldata_length_0] |  | 1,477,259 | 21,751.0 | 21.75 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_calldatasize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-calldata_length_10000] |  | 1,477,259 | 19,885.0 | 19.89 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_callvalue[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-from_origin_False-non_zero_value_True] |  | 1,477,259 | 19,676.0 | 19.68 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_callvalue[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-from_origin_True-non_zero_value_True] |  | 1,477,259 | 20,401.0 | 20.40 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DUP13] |  | 1,477,259 | 20,038.0 | 20.04 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DUP14] |  | 1,477,259 | 21,170.0 | 21.17 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DUP16] |  | 1,477,259 | 19,600.0 | 19.60 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DUP3] |  | 1,477,259 | 19,136.0 | 19.14 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DUP4] |  | 1,477,259 | 19,838.0 | 19.84 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DUP5] |  | 1,477,259 | 19,820.0 | 19.82 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_jumpdests[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test] |  | 1,477,259 | 20,998.0 | 21.00 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_jumps[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test] |  | 1,477,259 | 17,131.0 | 17.13 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_keccak[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test] |  | 1,477,259 | 38,030.0 | 38.03 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_False-offset_initialized_False-offset_0-opcode_MLOAD] |  | 1,477,259 | 24,999.0 | 25.00 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_False-offset_initialized_False-offset_0-opcode_MSTORE8] |  | 1,477,259 | 23,547.0 | 23.55 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_False-offset_initialized_True-offset_31-opcode_MSTORE8] |  | 1,477,259 | 22,764.0 | 22.76 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_False-offset_initialized_True-offset_31-opcode_MSTORE] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_0-opcode_MLOAD] |  | 1,477,259 | 25,368.0 | 25.37 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_0-opcode_MSTORE8] |  | 1,477,259 | 23,780.0 | 23.78 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_1-opcode_MLOAD] |  | 1,477,259 | 24,994.0 | 24.99 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_1-opcode_MSTORE8] |  | 1,477,259 | 23,209.0 | 23.21 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_31-opcode_MLOAD] |  | 1,477,259 | 25,743.0 | 25.74 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_31-opcode_MSTORE8] |  | 1,477,259 | 22,777.0 | 22.78 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_31-opcode_MSTORE] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_True-offset_0-opcode_MLOAD] |  | 1,477,259 | 25,310.0 | 25.31 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_True-offset_31-opcode_MSTORE] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_mod[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_MOD-mod_bits_127] |  | 1,477,259 | 79,755.0 | 79.75 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_mod[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_SMOD-mod_bits_191] |  | 1,477,259 | 107,669.0 | 107.67 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_mod[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_SMOD-mod_bits_63] |  | 1,477,259 | 59,104.0 | 59.10 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modarith[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_ADDMOD-mod_bits_63] |  | 1,477,259 | 66,249.0 | 66.25 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modarith[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_MULMOD-mod_bits_191] |  | 1,477,259 | 162,845.0 | 162.84 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modarith[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_MULMOD-mod_bits_255] |  | 1,477,259 | 155,924.0 | 155.92 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_600_gas_exp_heavy] |  | 1,477,259 | 446,645.0 | 446.64 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_616_gas_base_heavy] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_765_gas_exp_heavy] |  | 1,477,259 | 360,759.0 | 360.76 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_800_gas_base_heavy] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_800_gas_exp_heavy] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_867_gas_base_heavy] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_32b_exp_96] |  | 1,477,259 | 328,857.0 | 328.86 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_512b_exp_1024] |  | 1,477,259 | 11,072.0 | 11.07 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_8b_exp_896] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_exp_298_gas_exp_heavy] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_min_as_exp_heavy] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_odd_1024b_exp_1024] |  | 1,477,259 | 11,258.0 | 11.26 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_odd_32b_exp_96] |  | 1,477,259 | 310,723.0 | 310.72 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_odd_64b_exp_512] |  | 1,477,259 | 314,917.0 | 314.92 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_pawel_2] |  | 1,477,259 | 425,560.0 | 425.56 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1152n1] |  | 1,477,259 | 191,390.0 | 191.39 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1349n1] |  | 1,477,259 | 283,831.0 | 283.83 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1360n1] |  | 1,477,259 | 296,382.0 | 296.38 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1360n2] |  | 1,477,259 | 268,035.0 | 268.04 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_200n1] |  | 1,477,259 | 153,641.0 | 153.64 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_example_1] |  | 1,477,259 | 334,761.0 | 334.76 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_guido_1_even] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_guido_2_even] |  | 1,477,259 | 405,416.0 | 405.42 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_marius_1_even] |  | 1,477,259 | 368,528.0 | 368.53 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_1_qube] |  | 1,477,259 | 184,758.0 | 184.76 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_1_square] |  | 1,477,259 | 175,629.0 | 175.63 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_3_pow_0x10001] |  | 1,477,259 | 246,356.0 | 246.36 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_3_qube] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_3_square] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_4_pow_0x10001] |  | 1,477,259 | 235,376.0 | 235.38 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_5_pow_0x10001] |  | 1,477,259 | 217,267.0 | 217.27 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_5_square] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_pawel_1_exp_heavy] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_pawel_2_exp_heavy] |  | 1,477,259 | 407,163.0 | 407.16 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_pawel_3_exp_heavy] |  | 1,477,259 | 329,550.0 | 329.55 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_msize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mem_size_100000] |  | 1,477,259 | 24,139.0 | 24.14 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_msize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mem_size_1000] |  | 1,477,259 | 24,618.0 | 24.62 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_msize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mem_size_1] |  | 1,477,259 | 24,505.0 | 24.50 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-blake2f] |  | 1,477,259 | 252,270.0 | 252.27 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_g1msm] |  | 1,477,259 | 218,385.0 | 218.38 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_add] |  | 1,477,259 | 48,473.0 | 48.47 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_mul] |  | 1,477,259 | 150,210.0 | 150.21 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_mul_1_2_2_scalar] |  | 1,477,259 | 14,516.0 | 14.52 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_one_pairing] |  | 1,477,259 | 98,180.0 | 98.18 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_two_pairings] |  | 1,477,259 | 97,618.0 | 97.62 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_two_pairings_empty] |  | 1,477,259 | 12,119.0 | 12.12 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-ecrecover] |  | 1,477,259 | 65,265.0 | 65.27 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_only_data_input[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-IDENTITY] |  | 1,477,259 | 22,985.0 | 22.98 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_only_data_input[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-RIPEMD-160] |  | 1,477,259 | 16,581.0 | 16.58 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_precompile_only_data_input[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-SHA2-256] |  | 1,477,259 | 50,649.0 | 50.65 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH13] |  | 1,477,259 | 25,873.0 | 25.87 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH15] |  | 1,477,259 | 26,359.0 | 26.36 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH16] |  | 1,477,259 | 27,594.0 | 27.59 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH20] |  | 1,477,259 | 33,226.0 | 33.23 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH23] |  | 1,477,259 | 29,818.0 | 29.82 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH25] |  | 1,477,259 | 29,217.0 | 29.22 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH28] |  | 1,477,259 | 30,935.0 | 30.93 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH29] |  | 1,477,259 | 31,174.0 | 31.17 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH2] |  | 1,477,259 | 19,069.0 | 19.07 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH7] |  | 1,477,259 | 23,624.0 | 23.62 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1KiB of non-zero data-opcode_RETURN] |  | 1,477,259 | 28,411.0 | 28.41 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1KiB of non-zero data-opcode_REVERT] |  | 1,477,259 | 29,502.0 | 29.50 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1KiB of zero data-opcode_REVERT] |  | 1,477,259 | 34,604.0 | 34.60 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1MiB of zero data-opcode_RETURN] |  | 1,477,259 | 11,096.0 | 11.10 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1MiB of zero data-opcode_REVERT] |  | 1,477,259 | 11,679.0 | 11.68 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_returndatasize_nonzero[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-returned_size_0-return_data_style_ReturnDataStyle.REVERT] |  | 1,477,259 | 19,383.0 | 19.38 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_returndatasize_nonzero[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-returned_size_1-return_data_style_ReturnDataStyle.REVERT] |  | 1,477,259 | 19,584.0 | 19.58 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP3] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP4] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP5] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP6] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP8] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP9] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_tload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-val_mut_False-key_mut_False] |  | 1,477,259 | 12,571.0 | 12.57 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_tload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-val_mut_False-key_mut_True] |  | 1,477,259 | 13,279.0 | 13.28 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_tload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-val_mut_True-key_mut_True] |  | 1,477,259 | 13,522.0 | 13.52 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_tstore[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-dense_val_mut_False-key_mut_True] |  | 1,477,259 | 18,240.0 | 18.24 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_tstore[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-dense_val_mut_True-key_mut_True] |  | 1,477,259 | 26,775.0 | 26.77 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_zero_param[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_ADDRESS] |  | 1,477,259 | 33,644.0 | 33.64 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_zero_param[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_BASEFEE] |  | 1,477,259 | 22,231.0 | 22.23 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_zero_param[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_CHAINID] |  | 1,477,259 | 22,599.0 | 22.60 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_zero_param[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_GASLIMIT] |  | 1,477,259 | 22,272.0 | 22.27 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_zero_param[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_GASPRICE] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_zero_param[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_ORIGIN] |  | 1,477,259 | 32,043.0 | 32.04 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_False-0 bytes-call] |  | 1,477,259 | 28,800.0 | 28.80 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_False-0 bytes-transaction] |  | 1,477,259 | 28,752.0 | 28.75 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_False-1MiB-call] |  | 1,477,259 | 11,451.0 | 11.45 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_True-0 bytes-transaction] |  | 1,477,259 | 21,796.0 | 21.80 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_True-100 bytes-call] |  | 1,477,259 | 18,555.0 | 18.55 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_True-10KiB-transaction] |  | 1,477,259 | 14,165.0 | 14.16 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_True-fixed_src_dst_False-100 bytes-call] |  | 1,477,259 | 25,917.0 | 25.92 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_True-fixed_src_dst_False-100 bytes-transaction] |  | 1,477,259 | 26,796.0 | 26.80 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_True-fixed_src_dst_True-100 bytes-transaction] |  | 1,477,259 | 18,512.0 | 18.51 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_True-fixed_src_dst_True-10KiB-call] |  | 1,477,259 | 15,904.0 | 15.90 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-0 bytes] |  | 1,477,259 | 28,021.0 | 28.02 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-0.50x max code size] |  | 1,477,259 | 21,792.0 | 21.79 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-0.75x max code size] |  | 1,477,259 | 17,639.0 | 17.64 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-max code size] |  | 1,477,259 | 17,037.0 | 17.04 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_True-0 bytes] |  | 1,477,259 | 22,609.0 | 22.61 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_True-0.75x max code size] |  | 1,477,259 | 15,721.0 | 15.72 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_mcopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-1MiB] |  | 1,477,259 | 11,718.0 | 11.72 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_mcopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_True-100 bytes] |  | 1,477,259 | 20,225.0 | 20.23 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_mcopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_True-10KiB] |  | 1,477,259 | 18,516.0 | 18.52 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_False-1MiB] |  | 1,477,259 | 12,314.0 | 12.31 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-0 bytes] |  | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-100 bytes] |  | 1,477,259 | 19,795.0 | 19.80 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-1MiB] |  | 1,477,259 | 11,763.0 | 11.76 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-0_bytes_data-log0] |  | 1,477,259 | 12,620.0 | 12.62 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-0_bytes_data-log2] |  | 1,477,259 | 11,991.0 | 11.99 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-0_bytes_data-log4] |  | 1,477,259 | 12,279.0 | 12.28 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_non_zero_data-log1] |  | 1,477,259 | 11,927.0 | 11.93 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_non_zero_data-log2] |  | 1,477,259 | 11,054.0 | 11.05 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_non_zero_data-log4] |  | 1,477,259 | 11,244.0 | 11.24 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_zeros_data-log4] |  | 1,477,259 | 11,198.0 | 11.20 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-0_bytes_data-log1] |  | 1,477,259 | 13,061.0 | 13.06 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-0_bytes_data-log2] |  | 1,477,259 | 11,964.0 | 11.96 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_non_zero_data-log0] |  | 1,477,259 | 10,827.0 | 10.83 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_non_zero_data-log3] |  | 1,477,259 | 10,901.0 | 10.90 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_non_zero_data-log4] |  | 1,477,259 | 11,500.0 | 11.50 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log0] |  | 1,477,259 | 11,258.0 | 11.26 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log1] |  | 1,477,259 | 11,156.0 | 11.16 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log2] |  | 1,477,259 | 10,917.0 | 10.92 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log3] |  | 1,477,259 | 11,303.0 | 11.30 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log4] |  | 1,477,259 | 11,326.0 | 11.33 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-0_bytes_data-log2] |  | 1,477,259 | 12,376.0 | 12.38 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-0_bytes_data-log4] |  | 1,477,259 | 11,782.0 | 11.78 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-1_MiB_non_zero_data-log2] |  | 1,477,259 | 11,201.0 | 11.20 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-1_MiB_zeros_data-log1] |  | 1,477,259 | 11,899.0 | 11.90 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-0_bytes_data-log3] |  | 1,477,259 | 11,818.0 | 11.82 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_non_zero_data-log1] |  | 1,477,259 | 11,692.0 | 11.69 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_non_zero_data-log2] |  | 1,477,259 | 11,135.0 | 11.13 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_non_zero_data-log3] |  | 1,477,259 | 11,055.0 | 11.05 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_zeros_data-log0] |  | 1,477,259 | 11,546.0 | 11.55 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_zeros_data-log4] |  | 1,477,259 | 11,219.0 | 11.22 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_address_state_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_accounts_True-opcode_BALANCE] |  | 1,477,259 | 12,993.0 | 12.99 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_address_state_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-absent_target_False-opcode_CALL] |  | 1,477,259 | 40,478.0 | 40.48 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_address_state_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-absent_target_False-opcode_DELEGATECALL] |  | 1,477,259 | 34,001.0 | 34.00 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_address_state_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-absent_target_False-opcode_EXTCODEHASH] |  | 1,477,259 | 17,445.0 | 17.45 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_address_state_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-absent_target_False-opcode_EXTCODESIZE] |  | 1,477,259 | 16,512.0 | 16.51 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_address_state_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-absent_target_True-opcode_DELEGATECALL] |  | 1,477,259 | 32,619.0 | 32.62 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_address_state_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-absent_target_True-opcode_EXTCODESIZE] |  | 1,477,259 | 16,031.0 | 16.03 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_blockhash[fork_Prague-benchmark-gas-value_1M-blockchain_test] |  | 1,477,259 | 23,400.0 | 23.40 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_extcodecopy_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1KiB] |  | 1,477,259 | 16,771.0 | 16.77 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_extcodecopy_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-512] |  | 1,477,259 | 16,476.0 | 16.48 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_selfdestruct_created[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-value_bearing_False] |  | 1,477,259 | 11,929.0 | 11.93 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_selfdestruct_existing[fork_Prague-benchmark-gas-value_1M-blockchain_test-value_bearing_False] |  | 1,477,259 | 13,929.0 | 13.93 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_selfdestruct_existing[fork_Prague-benchmark-gas-value_1M-blockchain_test-value_bearing_True] |  | 1,477,259 | 13,550.0 | 13.55 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_False-SSTORE new value, out of gas] |  | 1,477,259 | 14,277.0 | 14.28 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_False-SSTORE new value] |  | 1,477,259 | 14,214.0 | 14.21 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_False-SSTORE same value, out of gas] |  | 1,477,259 | 15,691.0 | 15.69 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_True-SSTORE same value, revert] |  | 1,477,259 | 11,741.0 | 11.74 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_storage_access_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test-SLOAD] |  | 1,477,259 | 17,096.0 | 17.10 |
| sp1-v5.1.0/test_worst_stateful_opcodes.py::test_worst_storage_access_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test-SSTORE new value] |  | 1,477,259 | 29,888.0 | 29.89 |

