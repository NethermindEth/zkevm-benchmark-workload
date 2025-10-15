# SP1 vs RISC0 Metrics Comparison Summary

**RISC0 Folder:** `zkevm-metrics-risc0-1M`  
**SP1 Folder:** `zkevm-metrics-sp1-1M`  
**Common Tests:** 75

---

## Executive Summary

| Metric | Winner | Performance Advantage |
|--------|--------|----------------------|
| **Proving Speed** | ✅ **SP1** | **1.47x faster** on average |
| **Proof Size** | ✅ **RISC0** | **6.60x smaller** proofs |
| **Memory Usage** | ✅ **RISC0** | **1.09x less** memory |

---

## Detailed Performance Analysis

### 🚀 Proving Time Performance

**SP1 is significantly faster at proving:**

- **Average speedup:** 1.47x
- **Median speedup:** 1.45x
- **Range:** 0.39x to 2.87x

**Time Comparison:**
- **Total RISC0 time:** 14,655 seconds (4.1 hours)
- **Total SP1 time:** 6,445 seconds (1.8 hours)
- **Time difference:** 8,210 seconds (2.3 hours)

### 📦 Proof Size Analysis

**RISC0 generates smaller proofs:**

- **Average ratio:** 0.15x
- **Median ratio:** 0.15x

### 💾 Memory Usage Analysis

**RISC0 is more memory efficient:**

- **Average ratio:** 0.92x
- **Median ratio:** 0.88x

---

## Top Performance Winners

### 🏆 Top 10 Tests Where SP1 Dominates (Fastest Proving)

1. **test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_400_gas_exp_heavy]**: **2.87x faster**, saved 875s
2. **test_worst_bytecode.py::test_worst_bytecode_single_opcode[fork_Prague-benchmark-gas-value_1M-blockchain_test-opcode_CALL]**: **2.86x faster**, saved 234s
3. **test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1360n2]**: **2.84x faster**, saved 526s
4. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-blake2f]**: **2.78x faster**, saved 528s
5. **test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_common_1360n1]**: **2.75x faster**, saved 586s
6. **test_worst_compute.py::test_amortized_bn128_pairings[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test]**: **2.70x faster**, saved 200s
7. **test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_32b_exp_256]**: **2.63x faster**, saved 605s
8. **test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_even_16b_exp_320]**: **2.63x faster**, saved 824s
9. **test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_vul_nagydani_1_qube]**: **2.59x faster**, saved 337s
10. **test_worst_compute.py::test_worst_modexp[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-mod_pawel_3]**: **2.58x faster**, saved 620s

### 🏆 Top 10 Tests Where RISC0 Dominates (Fastest Proving)

1. **test_worst_memory.py::test_worst_calldatacopy[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-non_zero_data_False-fixed_src_dst_False-1MiB-call]**: **2.55x faster** than SP1, saved 9s
2. **test_worst_compute.py::test_worst_precompile_fixed_cost[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-bls12_g2msm]**: **2.36x faster** than SP1, saved 108s
3. **test_worst_compute.py::test_worst_return_revert[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-1MiB of non-zero data-opcode_REVERT]**: **2.36x faster** than SP1, saved 10s
4. **test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_zeros_data-log4]**: **2.29x faster** than SP1, saved 9s
5. **test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_zeros_data-log4]**: **2.29x faster** than SP1, saved 8s
6. **test_worst_stateful_opcodes.py::test_worst_storage_access_cold[fork_Prague-benchmark-gas-value_1M-blockchain_test-absent_slots_True-SSTORE new value, out of gas]**: **2.29x faster** than SP1, saved 9s
7. **test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_non_zero_data-log4]**: **2.29x faster** than SP1, saved 8s
8. **test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-zeros_topic-1_MiB_non_zero_data-log2]**: **2.19x faster** than SP1, saved 8s
9. **test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_False-non_zero_topic-1_MiB_zeros_data-log0]**: **2.18x faster** than SP1, saved 8s
10. **test_worst_opcode.py::test_worst_log_opcodes[fork_Prague-benchmark-gas-value_1M-blockchain_test_from_state_test-fixed_offset_True-non_zero_topic-1_MiB_non_zero_data-log2]**: **2.18x faster** than SP1, saved 8s

---

## Test Coverage

| Category | Count | Notes |
|----------|-------|-------|
| **Common tests** | 75 | Tests executed by both systems |
| **RISC0 only** | 0 | Tests only in RISC0 |
| **SP1 only** | 129 | Tests only in SP1 |
