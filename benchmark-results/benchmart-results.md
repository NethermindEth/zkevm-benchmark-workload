# zkEVM Benchmark Results

Generated on: 2025-09-23 10:04:39

Comparing 2 metrics folders:
- zkevm-metrics-risc0-1M (Gas: 1M)
- zkevm-metrics-sp1-1M (Gas: 1M)

## Gas Category: 1M

Source: zkevm-metrics-risc0-1M

## Proving Metrics

| Benchmark | Gas Category | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |
|---|---|---|---|---|---|---|---|
| risc0-v3.0.1/test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_a_to_a] |  | 223,662 | 46,759.0 | 46.76 | 286,972.0 | 286,972.0 | 286,972.0 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_EXTCODESIZE] |  | 223,662 | 358,499.0 | 358.50 | 205,660.0 | 203,562.3 | 181,020.0 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0 bytes with value-opcode_CREATE] |  | 223,662 | 7,365.0 | 7.37 | 176,988.0 | 176,988.0 | 176,988.0 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0 bytes without value-opcode_CREATE2] |  | 223,662 | 8,024.0 | 8.02 | 176,316.0 | 176,316.0 | 176,316.0 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.75x max code size with non-zero data-opcode_CREATE2] |  | 223,662 | 8,500.0 | 8.50 | 332,444.0 | 332,444.0 | 332,444.0 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.75x max code size with non-zero data-opcode_CREATE] |  | 223,662 | 7,507.0 | 7.51 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_bytecode.py::test_worst_create[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-0.75x max code size with zero data-opcode_CREATE2] |  | 223,662 | 7,277.0 | 7.28 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_LT-] |  | 223,662 | 33,859.0 | 33.86 | 249,788.0 | 249,314.5 | 247,548.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_OR-] |  | 223,662 | 34,455.0 | 34.45 | 287,196.0 | 287,196.0 | 287,196.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SDIV-1] |  | 223,662 | 253,076.0 | 253.08 | 287,196.0 | 287,196.0 | 287,196.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_binop_simple[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SUB-] |  | 223,662 | 38,784.0 | 38.78 | 218,428.0 | 217,547.6 | 215,740.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_blobhash[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-one blob but access non-existent index] |  | 223,662 | 36,616.0 | 36.62 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_calldataload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-zero-loop] |  | 223,662 | 86,867.0 | 86.87 | 176,316.0 | 176,102.1 | 175,644.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_calldatasize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-calldata_length_0] |  | 223,662 | 34,745.0 | 34.74 | 278,236.0 | 278,044.6 | 278,012.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_calldatasize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-calldata_length_10000] |  | 223,662 | 34,425.0 | 34.42 | 177,212.0 | 177,212.0 | 177,212.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_callvalue[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-from_origin_False-non_zero_value_True] |  | 223,662 | 29,404.0 | 29.40 | 287,644.0 | 287,644.0 | 287,644.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_callvalue[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-from_origin_True-non_zero_value_True] |  | 223,662 | 32,813.0 | 32.81 | 180,124.0 | 179,972.1 | 178,556.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_dup[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_DUP14] |  | 223,662 | 29,055.0 | 29.05 | 287,644.0 | 287,644.0 | 287,644.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_jumps[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test] |  | 223,662 | 19,650.0 | 19.65 | 221,340.0 | 220,605.8 | 218,428.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_False-offset_initialized_False-offset_0-opcode_MLOAD] |  | 223,662 | 52,633.0 | 52.63 | 288,092.0 | 288,092.0 | 288,092.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_False-offset_initialized_True-offset_31-opcode_MSTORE] |  | 223,662 | 68,336.0 | 68.34 | 286,972.0 | 286,972.0 | 286,972.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_0-opcode_MLOAD] |  | 223,662 | 52,619.0 | 52.62 | 118,076.0 | 116,342.3 | 107,324.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_1-opcode_MLOAD] |  | 223,662 | 53,672.0 | 53.67 | 125,244.0 | 122,997.6 | 118,300.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_mod[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_MOD-mod_bits_127] |  | 223,662 | 235,773.0 | 235.77 | 287,644.0 | 287,644.0 | 287,644.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_mod[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_SMOD-mod_bits_63] |  | 223,662 | 156,765.0 | 156.76 | 154,140.0 | 150,243.6 | 138,908.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modarith[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_ADDMOD-mod_bits_63] |  | 223,662 | 170,012.0 | 170.01 | 278,012.0 | 277,400.2 | 275,100.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modarith[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_MULMOD-mod_bits_191] |  | 223,662 | 443,439.0 | 443.44 | 281,820.0 | 280,849.2 | 278,236.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_600_gas_exp_heavy] |  | 223,662 | 1,302,889.0 | 1,302.89 | 177,212.0 | 177,212.0 | 177,212.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_765_gas_exp_heavy] |  | 223,662 | 1,055,416.0 | 1,055.42 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_867_gas_base_heavy] |  | 223,662 | 4,277,679.0 | 4,277.68 | 286,972.0 | 286,621.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_512b_exp_1024] |  | 223,662 | 6,801.0 | 6.80 | 303,324.0 | 303,194.3 | 302,428.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_8b_exp_896] |  | 223,662 | 2,634,370.0 | 2,634.37 | 176,988.0 | 176,612.5 | 176,316.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_exp_298_gas_exp_heavy] |  | N/A | N/A | N/A | N/A | N/A | N/A |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_pawel_2] |  | 223,662 | 1,233,415.0 | 1,233.41 | 175,644.0 | 173,237.6 | 169,596.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1349n1] |  | 223,662 | 794,409.0 | 794.41 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1360n1] |  | 223,662 | 941,290.0 | 941.29 | 286,524.0 | 286,034.6 | 281,820.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1360n2] |  | 223,662 | 856,588.0 | 856.59 | 288,092.0 | 287,984.9 | 287,644.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_example_1] |  | 223,662 | 1,074,133.0 | 1,074.13 | 332,444.0 | 329,928.4 | 312,732.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_marius_1_even] |  | 223,662 | 1,051,108.0 | 1,051.11 | 177,212.0 | 177,131.0 | 176,988.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_1_qube] |  | 223,662 | 567,434.0 | 567.43 | 269,724.0 | 266,445.5 | 251,804.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_3_pow_0x10001] |  | 223,662 | 761,002.0 | 761.00 | 178,556.0 | 177,835.6 | 177,212.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_3_square] |  | 223,662 | 3,454,406.0 | 3,454.41 | 287,196.0 | 287,004.5 | 286,972.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_4_pow_0x10001] |  | 223,662 | 651,506.0 | 651.51 | 275,100.0 | 272,898.7 | 270,172.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_pawel_1_exp_heavy] |  | 223,662 | 2,181,519.0 | 2,181.52 | 169,372.0 | 166,806.3 | 154,140.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_pawel_3_exp_heavy] |  | 223,662 | 1,077,253.0 | 1,077.25 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_msize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mem_size_100000] |  | 223,662 | 44,480.0 | 44.48 | 287,644.0 | 287,644.0 | 287,644.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_msize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mem_size_1000] |  | 223,662 | 48,023.0 | 48.02 | 270,172.0 | 270,172.0 | 270,172.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_msize[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mem_size_1] |  | 223,662 | 48,838.0 | 48.84 | 286,972.0 | 286,972.0 | 286,972.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-blake2f] |  | N/A | N/A | N/A | N/A | N/A | N/A |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_add] |  | 223,662 | 91,892.0 | 91.89 | 302,204.0 | 300,053.6 | 295,932.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_mul] |  | 223,662 | 367,432.0 | 367.43 | 176,316.0 | 176,316.0 | 176,316.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_mul_1_2_2_scalar] |  | 223,662 | 12,524.0 | 12.52 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_two_pairings_empty] |  | 223,662 | 6,614.0 | 6.61 | 270,172.0 | 270,172.0 | 270,172.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_only_data_input[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-IDENTITY] |  | 223,662 | 28,623.0 | 28.62 | 287,196.0 | 287,196.0 | 287,196.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_precompile_only_data_input[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-SHA2-256] |  | 223,662 | 11,356.0 | 11.36 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH15] |  | 223,662 | 44,766.0 | 44.77 | 287,196.0 | 287,196.0 | 287,196.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH16] |  | 223,662 | 44,058.0 | 44.06 | 286,972.0 | 286,972.0 | 286,972.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH20] |  | 223,662 | 49,179.0 | 49.18 | 181,020.0 | 181,020.0 | 180,124.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH23] |  | 223,662 | 55,889.0 | 55.89 | 292,796.0 | 292,426.7 | 290,556.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH25] |  | 223,662 | 58,959.0 | 58.96 | 230,972.0 | 228,389.7 | 221,340.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH28] |  | 223,662 | 69,759.0 | 69.76 | 177,212.0 | 177,212.0 | 177,212.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH29] |  | 223,662 | 62,329.0 | 62.33 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_push[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_PUSH7] |  | 223,662 | 43,139.0 | 43.14 | 287,196.0 | 287,196.0 | 287,196.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1MiB of zero data-opcode_RETURN] |  | 223,662 | 6,638.0 | 6.64 | 290,556.0 | 290,444.0 | 288,988.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1MiB of zero data-opcode_REVERT] |  | 223,662 | 6,215.0 | 6.21 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_returndatasize_nonzero[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-returned_size_1-return_data_style_ReturnDataStyle.REVERT] |  | 223,662 | 28,625.0 | 28.62 | 295,484.0 | 294,303.7 | 292,796.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP5] |  | 223,662 | 75,406.0 | 75.41 | 312,732.0 | 310,980.3 | 306,236.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_swap[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_SWAP9] |  | 223,662 | 68,442.0 | 68.44 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_tload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-val_mut_False-key_mut_False] |  | 223,662 | 11,351.0 | 11.35 | 177,212.0 | 177,212.0 | 177,212.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_tload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-val_mut_False-key_mut_True] |  | 223,662 | 10,525.0 | 10.53 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_tstore[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-dense_val_mut_False-key_mut_True] |  | 223,662 | 26,332.0 | 26.33 | 287,644.0 | 287,644.0 | 287,644.0 |
| risc0-v3.0.1/test_worst_compute.py::test_worst_zero_param[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-opcode_ORIGIN] |  | 223,662 | 69,247.0 | 69.25 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_False-1MiB-call] |  | 223,662 | 5,327.0 | 5.33 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_True-100 bytes-call] |  | 223,662 | 28,180.0 | 28.18 | 235,004.0 | 233,956.4 | 230,972.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_True-fixed_src_dst_False-100 bytes-transaction] |  | 223,662 | 51,406.0 | 51.41 | 287,420.0 | 287,420.0 | 287,420.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-0.50x max code size] |  | 223,662 | 22,653.0 | 22.65 | 177,212.0 | 177,212.0 | 177,212.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_False-max code size] |  | 223,662 | 25,342.0 | 25.34 | 205,660.0 | 205,660.0 | 205,660.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_True-0.75x max code size] |  | 223,662 | 17,102.0 | 17.10 | 176,316.0 | 176,316.0 | 176,316.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-0 bytes] |  | 223,662 | 40,561.0 | 40.56 | 176,316.0 | 176,316.0 | 176,316.0 |
| risc0-v3.0.1/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-1MiB] |  | 223,662 | 8,248.0 | 8.25 | 215,740.0 | 215,183.2 | 213,052.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-0_bytes_data-log2] |  | 223,662 | 7,788.0 | 7.79 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-0_bytes_data-log4] |  | 223,662 | 7,825.0 | 7.83 | 287,644.0 | 287,644.0 | 287,644.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_non_zero_data-log2] |  | 223,662 | 6,314.0 | 6.31 | 128,828.0 | 127,564.0 | 125,244.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-0_bytes_data-log2] |  | 223,662 | 7,390.0 | 7.39 | 105,920.0 | 104,181.3 | 97,856.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_non_zero_data-log0] |  | 223,662 | 6,620.0 | 6.62 | 270,172.0 | 270,172.0 | 270,172.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log1] |  | 223,662 | 6,626.0 | 6.63 | 131,292.0 | 130,411.6 | 128,828.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log3] |  | 223,662 | 6,239.0 | 6.24 | 138,908.0 | 138,369.2 | 136,220.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-0_bytes_data-log2] |  | 223,662 | 7,308.0 | 7.31 | 135,772.0 | 134,560.4 | 131,516.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-0_bytes_data-log4] |  | 223,662 | 7,405.0 | 7.41 | 178,556.0 | 178,556.0 | 178,556.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-1_MiB_zeros_data-log1] |  | 223,662 | 6,370.0 | 6.37 | 286,972.0 | 286,972.0 | 286,972.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_non_zero_data-log3] |  | 223,662 | 6,038.0 | 6.04 | 176,316.0 | 176,316.0 | 176,316.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_zeros_data-log0] |  | 223,662 | 6,653.0 | 6.65 | 288,988.0 | 288,377.1 | 288,092.0 |
| risc0-v3.0.1/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_zeros_data-log4] |  | 223,662 | 6,716.0 | 6.72 | 288,092.0 | 288,092.0 | 288,092.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_address_state_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-absent_target_True-opcode_DELEGATECALL] |  | 223,662 | 71,120.0 | 71.12 | 286,524.0 | 286,524.0 | 286,524.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_blockhash[fork_Prague-benchmark-gas-value_1M-blockchain_test] |  | 223,662 | 43,954.0 | 43.95 | 213,052.0 | 212,787.4 | 212,604.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_extcodecopy_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-512] |  | 223,662 | 21,076.0 | 21.08 | 239,708.0 | 238,591.2 | 235,004.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_selfdestruct_created[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-value_bearing_False] |  | 223,662 | 6,342.0 | 6.34 | 177,212.0 | 177,212.0 | 177,212.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_selfdestruct_existing[fork_Prague-benchmark-gas-value_1M-blockchain_test-value_bearing_False] |  | 223,662 | 14,742.0 | 14.74 | 287,644.0 | 287,644.0 | 287,644.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_selfdestruct_existing[fork_Prague-benchmark-gas-value_1M-blockchain_test-value_bearing_True] |  | 223,662 | 16,177.0 | 16.18 | 251,580.0 | 251,407.1 | 249,788.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_False-SSTORE new value, out of gas] |  | 223,662 | 14,755.0 | 14.76 | 288,092.0 | 288,092.0 | 288,092.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_True-SSTORE same value, revert] |  | 223,662 | 6,582.0 | 6.58 | 176,316.0 | 176,316.0 | 176,316.0 |
| risc0-v3.0.1/test_worst_stateful_opcodes.py::test_worst_storage_access_warm[fork_Prague-benchmark-gas-value_1M-blockchain_test-SLOAD] |  | 223,662 | 23,675.0 | 23.68 | 306,236.0 | 305,729.1 | 303,324.0 |

## Gas Category: 1M

Source: zkevm-metrics-sp1-1M

## Proving Metrics

| Benchmark | Gas Category | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |
|---|---|---|---|---|---|---|---|
| sp1-v5.1.0/test_worst_compute.py::test_worst_calldataload[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-zero-loop] |  | N/A | N/A | N/A | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_0-opcode_MLOAD] |  | 1,477,259 | 29,152.0 | 29.15 | 116,424.0 | 114,198.0 | 109,032.0 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_memory_access[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-big_memory_expansion_True-offset_initialized_False-offset_1-opcode_MLOAD] |  | 1,477,259 | 27,507.0 | 27.51 | 122,024.0 | 120,903.4 | 116,872.0 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_mod[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-op_SMOD-mod_bits_63] |  | 1,477,259 | 66,080.0 | 66.08 | 148,904.0 | 147,166.9 | 142,184.0 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_pawel_2] |  | 1,477,259 | 453,794.0 | 453.79 | 166,376.0 | 164,840.8 | 158,760.0 |
| sp1-v5.1.0/test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_pawel_1_exp_heavy] |  | N/A | N/A | N/A | N/A | N/A | N/A |
| sp1-v5.1.0/test_worst_memory.py::test_worst_codecopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_src_dst_True-0.75x max code size] |  | 1,477,259 | 18,632.0 | 18.63 | 182,280.0 | 179,774.3 | 175,112.0 |
| sp1-v5.1.0/test_worst_memory.py::test_worst_returndatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_dst_True-0 bytes] |  | 1,477,259 | 26,770.0 | 26.77 | 174,664.0 | 174,048.9 | 171,752.0 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_non_zero_data-log2] |  | 1,477,259 | 12,642.0 | 12.64 | 126,952.0 | 125,478.7 | 122,248.0 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-0_bytes_data-log2] |  | 1,477,259 | 13,020.0 | 13.02 | 108,360.0 | 105,235.9 | 96,264.0 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log1] |  | 1,477,259 | 12,976.0 | 12.98 | 131,880.0 | 130,569.5 | 127,624.0 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-zeros_topic-1_MiB_zeros_data-log3] |  | 1,477,259 | 13,111.0 | 13.11 | 141,960.0 | 139,929.6 | 136,360.0 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-0_bytes_data-log2] |  | 1,477,259 | 13,408.0 | 13.41 | 136,136.0 | 134,857.1 | 131,880.0 |
| sp1-v5.1.0/test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_non_zero_data-log3] |  | 1,477,259 | 13,311.0 | 13.31 | 171,304.0 | 170,999.0 | 168,840.0 |

