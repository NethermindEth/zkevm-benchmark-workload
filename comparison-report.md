# SP1 vs RISC0 Metrics Comparison Summary

**RISC0 Folder:** `zkevm-metrics-risc0-1M-1`  
**SP1 Folder:** `zkevm-metrics-sp1-1M`  
**Common Tests:** 503

---

## Executive Summary

| Metric | Winner | Performance Advantage |
|--------|--------|----------------------|
| **Proving Speed** | ✅ **RISC0** | **4.80x faster** on average |
| **Proof Size** | ✅ **RISC0** | **6.60x smaller** proofs |
| **Memory Usage** | ✅ **SP1** | **1.46x less** memory |

---

## Detailed Performance Analysis

### 🚀 Proving Time Performance

**RISC0 is significantly faster at proving:**

- **Average speedup:** 0.21x
- **Median speedup:** 0.20x
- **Range:** 0.01x to 0.61x

**Time Comparison:**
- **Total RISC0 time:** 96,067 seconds (26.7 hours)
- **Total SP1 time:** 498,745 seconds (138.5 hours)
- **Time difference:** 402,678 seconds (111.9 hours)

### 📦 Proof Size Analysis

**RISC0 generates smaller proofs:**

- **Average ratio:** 0.15x
- **Median ratio:** 0.15x

### 💾 Memory Usage Analysis

**SP1 is more memory efficient:**

- **Average ratio:** 1.46x
- **Median ratio:** 1.42x

---

## Top Performance Winners

### 🏆 Top 10 Tests Where SP1 Dominates (Fastest Proving)

1. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-ecrecover]**: **0.61x faster**, saved -139s
2. **test_worst_compute.py::test_worst_keccak[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test]**: **0.51x faster**, saved -128s
3. **test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_diff_acc_to_diff_acc]**: **0.46x faster**, saved -58s
4. **test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_diff_acc_to_b]**: **0.44x faster**, saved -63s
5. **test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_a_to_b]**: **0.43x faster**, saved -62s
6. **test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_a_to_a]**: **0.42x faster**, saved -65s
7. **test_worst_blocks.py::test_block_full_of_ether_transfers[fork_Prague-benchmark-gas-value_1M-blockchain_test-case_id_a_to_diff_acc]**: **0.41x faster**, saved -68s
8. **test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_CALLCODE]**: **0.35x faster**, saved -520s
9. **test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_EXTCODESIZE]**: **0.35x faster**, saved -515s
10. **test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_EXTCODEHASH]**: **0.35x faster**, saved -516s

### 🏆 Top 10 Tests Where RISC0 Dominates (Fastest Proving)

1. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_fp_to_g1]**: **71.84x faster** than SP1, saved 5,574s
2. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_fp_to_g2]**: **65.25x faster** than SP1, saved 3,971s
3. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_g1add]**: **52.78x faster** than SP1, saved 3,151s
4. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_g2add]**: **50.19x faster** than SP1, saved 3,387s
5. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_pairing_check]**: **45.42x faster** than SP1, saved 5,044s
6. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_g1msm]**: **27.51x faster** than SP1, saved 2,303s
7. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_g2msm]**: **27.25x faster** than SP1, saved 1,675s
8. **test_worst_compute.py::test_worst_precompile_only_data_input[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-SHA2-256]**: **20.81x faster** than SP1, saved 396s
9. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-point_evaluation]**: **10.60x faster** than SP1, saved 2,469s
10. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bn128_mul_32_byte_coord_and_scalar]**: **6.21x faster** than SP1, saved 1,235s

---

## Test Coverage

| Category | Count | Notes |
|----------|-------|-------|
| **Common tests** | 503 | Tests executed by both systems |
| **RISC0 only** | 0 | Tests only in RISC0 |
| **SP1 only** | 0 | Tests only in SP1 |
