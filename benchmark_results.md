# zkEVM Benchmark Results

Generated on: 2025-10-09 14:06:28

## Folder: ./zkevm-metrics-sp1-1M

## Proving Metrics

| Benchmark | Gas Used | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |
|---|---|---|---|---|---|---|---|
| Block Full Data (Non Zero Byte) | 1,000,000 | 1,477,259 | 15,078.0 | 15.08 | 284,940.0 | 284,940.0 | 284,940.0 |
| Bytecode Single Opcode (Call) | 1,000,000 | 1,477,259 | 126,038.0 | 126.04 | 282,700.0 | 282,684.1 | 266,572.0 |
| Create (Max Code, Create) | 1,000,000 | 1,477,259 | 14,872.0 | 14.87 | 211,244.0 | 211,194.5 | 209,676.0 |
| Creates Collisions (Create) | 1,000,000 | 1,477,259 | 16,300.0 | 16.30 | 282,700.0 | 282,700.0 | 282,700.0 |
| Initcode Jumpdest Analysis (605B5B) | 1,000,000 | 1,477,259 | 23,275.0 | 23.27 | 258,284.0 | 258,284.0 | 258,284.0 |
| Initcode Jumpdest Analysis (615B5B5B) | 1,000,000 | 1,477,259 | 22,027.0 | 22.03 | 293,228.0 | 293,228.0 | 293,228.0 |
| Amortized Bn128 Pairings | 1,000,000 | 1,477,259 | 117,593.0 | 117.59 | 184,812.0 | 182,167.0 | 175,404.0 |
| Binop Simple (Byte) | 1,000,000 | 1,477,259 | 26,708.0 | 26.71 | 260,748.0 | 260,678.2 | 260,300.0 |
| Binop Simple (Div, 1) | 1,000,000 | 1,477,259 | 93,288.0 | 93.29 | 196,460.0 | 194,556.4 | 191,756.0 |
| Binop Simple (Mul) | 1,000,000 | 1,477,259 | 39,586.0 | 39.59 | 284,940.0 | 284,940.0 | 284,940.0 |
| Binop Simple (Or) | 1,000,000 | 1,477,259 | 25,646.0 | 25.65 | 265,228.0 | 265,228.0 | 265,228.0 |
| Binop Simple (Sar) | 1,000,000 | 1,477,259 | 43,123.0 | 43.12 | 284,940.0 | 284,940.0 | 284,940.0 |
| Binop Simple (Slt) | 1,000,000 | 1,477,259 | 29,254.0 | 29.25 | 284,940.0 | 284,940.0 | 284,940.0 |
| Binop Simple (Smod) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Dup (Dup13) | 1,000,000 | 1,477,259 | 24,389.0 | 24.39 | 257,388.0 | 257,388.0 | 257,388.0 |
| Memory Access (Small Mem, Uninit Offset, Off 0, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Small Mem, Uninit Offset, Off 31, Mstore8) | 1,000,000 | 1,477,259 | 27,782.0 | 27.78 | 259,404.0 | 258,906.7 | 258,732.0 |
| Memory Access (Small Mem, Uninit Offset, Off 31, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Small Mem, Init Offset, Off 1, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Small Mem, Init Offset, Off 31, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Big Mem, Uninit Offset, Off 0, Mstore8) | 1,000,000 | 1,477,259 | 23,715.0 | 23.71 | 108,428.0 | 107,064.0 | 96,332.0 |
| Memory Access (Big Mem, Uninit Offset, Off 31, Mstore8) | 1,000,000 | 1,477,259 | 27,333.0 | 27.33 | 285,164.0 | 285,164.0 | 285,164.0 |
| Mod (Op Mod, Mod Bits 191) | 1,000,000 | 1,477,259 | 119,036.0 | 119.04 | 257,388.0 | 257,388.0 | 257,388.0 |
| Mod (Op Mod, Mod Bits 255) | 1,000,000 | 1,477,259 | 91,859.0 | 91.86 | 294,348.0 | 294,070.5 | 293,676.0 |
| Mod (Op Smod, Mod Bits 191) | 1,000,000 | 1,477,259 | 125,758.0 | 125.76 | 257,388.0 | 257,388.0 | 257,388.0 |
| Modarith (Op Addmod, Mod Bits 191) | 1,000,000 | 1,477,259 | 139,138.0 | 139.14 | 293,228.0 | 292,933.0 | 292,108.0 |
| Modarith (Op Addmod, Mod Bits 255) | 1,000,000 | 1,477,259 | 104,544.0 | 104.54 | 191,308.0 | 189,120.5 | 185,036.0 |
| Modarith (Op Mulmod, Mod Bits 63) | 1,000,000 | 1,477,259 | 101,935.0 | 101.94 | 266,572.0 | 266,572.0 | 266,572.0 |
| Modexp (Mod 400 Gas Exp Heavy) | 1,000,000 | 1,477,259 | 467,706.0 | 467.71 | 266,348.0 | 265,911.9 | 265,452.0 |
| Modexp (Mod Even 16B Exp 320) | 1,000,000 | 1,477,259 | 505,963.0 | 505.96 | 305,100.0 | 302,981.8 | 299,052.0 |
| Modexp (Mod Even 32B Exp 256) | 1,000,000 | 1,477,259 | 371,034.0 | 371.03 | 143,148.0 | 141,204.8 | 137,100.0 |
| Modexp (Mod Pawel 3) | 1,000,000 | 1,477,259 | 393,345.0 | 393.35 | 284,940.0 | 283,471.3 | 282,700.0 |
| Modexp (Mod Vul Common 1360N1) | 1,000,000 | 1,477,259 | 333,810.0 | 333.81 | 293,676.0 | 293,478.7 | 293,228.0 |
| Modexp (Mod Vul Common 1360N2) | 1,000,000 | 1,477,259 | 286,047.0 | 286.05 | 137,100.0 | 134,002.2 | 125,900.0 |
| Modexp (Mod Vul Example 1) | 1,000,000 | 1,477,259 | 387,750.0 | 387.75 | 266,572.0 | 266,572.0 | 266,348.0 |
| Modexp (Mod Vul Guido 1 Even) | 1,000,000 | 1,477,259 | 289,152.0 | 289.15 | 207,660.0 | 203,956.2 | 196,460.0 |
| Modexp (Mod Vul Nagydani 1 Qube) | 1,000,000 | 1,477,259 | 211,569.0 | 211.57 | 295,468.0 | 294,960.3 | 294,348.0 |
| Modexp (Mod Vul Nagydani 2 Square) | 1,000,000 | 1,477,259 | 579,904.0 | 579.90 | 265,228.0 | 264,156.9 | 261,868.0 |
| Modexp (Mod Vul Nagydani 5 Square) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Precompile Fixed Cost (Blake2F) | 1,000,000 | 1,477,259 | 296,391.0 | 296.39 | 257,836.0 | 257,785.2 | 257,388.0 |
| Precompile Fixed Cost (Bls12 G2Msm) | 1,000,000 | 1,477,259 | 187,029.0 | 187.03 | 290,316.0 | 289,626.7 | 287,180.0 |
| Precompile Fixed Cost (Bn128 Mul) | 1,000,000 | 1,477,259 | 166,347.0 | 166.35 | 156,364.0 | 154,055.2 | 148,076.0 |
| Push (Push13) | 1,000,000 | 1,477,259 | 30,040.0 | 30.04 | 282,700.0 | 282,700.0 | 282,700.0 |
| Push (Push15) | 1,000,000 | 1,477,259 | 31,158.0 | 31.16 | 223,340.0 | 221,317.6 | 218,636.0 |
| Push (Push31) | 1,000,000 | 1,477,259 | 37,907.0 | 37.91 | 257,836.0 | 257,836.0 | 257,836.0 |
| Push (Push8) | 1,000,000 | 1,477,259 | 26,824.0 | 26.82 | 265,228.0 | 265,228.0 | 265,228.0 |
| Return Revert (1Mib Of Non, Zero Data, Return) | 1,000,000 | 1,477,259 | 15,013.0 | 15.01 | 295,692.0 | 295,692.0 | 295,468.0 |
| Return Revert (1Mib Of Non, Zero Data, Revert) | 1,000,000 | 1,477,259 | 16,727.0 | 16.73 | 284,940.0 | 284,940.0 | 284,940.0 |
| Return Revert (1Mib Of Zero Data, Revert) | 1,000,000 | 1,477,259 | 13,297.0 | 13.30 | 158,380.0 | 157,840.1 | 156,588.0 |
| Return Revert (Empty, Revert) | 1,000,000 | 1,477,259 | 49,341.0 | 49.34 | 257,388.0 | 257,388.0 | 257,388.0 |
| Returndatasize Nonzero (Returned Size 1, Return Data Style Returndatastyle.Revert) | 1,000,000 | 1,477,259 | 23,898.0 | 23.90 | 265,228.0 | 265,228.0 | 265,228.0 |
| Swap (Swap10) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap14) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap8) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Tload (Val Mut True, Key Mut False) | 1,000,000 | 1,477,259 | 16,569.0 | 16.57 | 144,492.0 | 144,327.7 | 143,148.0 |
| Tstore (Dense Val Mut True, Key Mut False) | 1,000,000 | 1,477,259 | 34,016.0 | 34.02 | 266,572.0 | 266,572.0 | 266,572.0 |
| Zero Param (Address) | 1,000,000 | 1,477,259 | 38,521.0 | 38.52 | 265,452.0 | 265,296.9 | 265,228.0 |
| Zero Param (Caller) | 1,000,000 | 1,477,259 | 38,402.0 | 38.40 | 257,836.0 | 257,836.0 | 257,836.0 |
| Calldatacopy (Zero Data, Dynamic Src Dst, 0Bytes, Transaction) | 1,000,000 | 1,477,259 | 34,962.0 | 34.96 | 258,508.0 | 258,348.7 | 258,284.0 |
| Calldatacopy (Zero Data, Dynamic Src Dst, 1Mib, Call) | 1,000,000 | 1,477,259 | 14,697.0 | 14.70 | 217,740.0 | 216,791.7 | 214,828.0 |
| Calldatacopy (Zero Data, Fixed Src Dst, 0Bytes, Call) | 1,000,000 | 1,477,259 | 25,250.0 | 25.25 | 148,076.0 | 147,153.6 | 144,716.0 |
| Calldatacopy (Non Zero, Dynamic Src Dst, 100Bytes, Call) | 1,000,000 | 1,477,259 | 32,448.0 | 32.45 | 258,732.0 | 258,621.6 | 258,508.0 |
| Codecopy (Fixed Src Dst, 0.50X Max Code Size) | 1,000,000 | 1,477,259 | 20,154.0 | 20.15 | 227,148.0 | 225,606.4 | 223,340.0 |
| Returndatacopy (Fixed Dst False, 100Bytes) | 1,000,000 | 1,477,259 | 28,951.0 | 28.95 | 162,860.0 | 161,860.1 | 159,500.0 |
| Returndatacopy (Fixed Dst True, 0Bytes) | 1,000,000 | 1,477,259 | 29,870.0 | 29.87 | 285,164.0 | 285,164.0 | 284,940.0 |
| Returndatacopy (Fixed Dst True, 1Mib) | 1,000,000 | 1,477,259 | 15,573.0 | 15.57 | 299,052.0 | 298,921.9 | 297,484.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 0 Bytes Data, Log1) | 1,000,000 | 1,477,259 | 15,546.0 | 15.55 | 257,836.0 | 257,836.0 | 257,836.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 0 Bytes Data, Log4) | 1,000,000 | 1,477,259 | 15,680.0 | 15.68 | 231,852.0 | 230,555.2 | 227,596.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Non Zero Data, Log4) | 1,000,000 | 1,477,259 | 14,818.0 | 14.82 | 166,444.0 | 165,802.5 | 163,532.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Zeros Data, Log0) | 1,000,000 | 1,477,259 | 14,920.0 | 14.92 | 260,300.0 | 260,300.0 | 260,300.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Zeros Data, Log4) | 1,000,000 | 1,477,259 | 14,974.0 | 14.97 | 284,940.0 | 284,940.0 | 284,940.0 |
| Log Opcodes (Dynamic, Zero Topic, 0 Bytes Data, Log4) | 1,000,000 | 1,477,259 | 14,529.0 | 14.53 | 174,732.0 | 173,134.9 | 170,028.0 |
| Log Opcodes (Dynamic, Zero Topic, 1 Mib Non Zero Data, Log3) | 1,000,000 | 1,477,259 | 12,143.0 | 12.14 | 114,476.0 | 112,968.7 | 109,772.0 |
| Log Opcodes (Dynamic, Zero Topic, 1 Mib Zeros Data, Log4) | 1,000,000 | 1,477,259 | 14,893.0 | 14.89 | 287,180.0 | 287,038.6 | 285,164.0 |
| Log Opcodes (Fixed, Non Zero Topic, 1 Mib Non Zero Data, Log2) | 1,000,000 | 1,477,259 | 15,161.0 | 15.16 | 285,164.0 | 285,164.0 | 285,164.0 |
| Log Opcodes (Fixed, Zero Topic, 0 Bytes Data, Log1) | 1,000,000 | 1,477,259 | 15,061.0 | 15.06 | 285,164.0 | 285,164.0 | 285,164.0 |
| Log Opcodes (Fixed, Zero Topic, 1 Mib Non Zero Data, Log2) | 1,000,000 | 1,477,259 | 15,032.0 | 15.03 | 284,940.0 | 284,940.0 | 284,940.0 |
| Log Opcodes (Fixed, Zero Topic, 1 Mib Zeros Data, Log4) | 1,000,000 | 1,477,259 | 15,813.0 | 15.81 | 305,100.0 | 305,100.0 | 305,100.0 |
| Address State Warm (Present Target, Delegatecall) | 1,000,000 | 1,477,259 | 41,838.0 | 41.84 | 209,676.0 | 209,418.0 | 207,884.0 |
| Address State Warm (Absent Target, Call) | 1,000,000 | 1,477,259 | 44,549.0 | 44.55 | 284,940.0 | 284,940.0 | 284,940.0 |
| Address State Warm (Absent Target, Delegatecall) | 1,000,000 | 1,477,259 | 38,464.0 | 38.46 | 292,108.0 | 291,890.8 | 290,316.0 |
| Address State Warm (Absent Target, Staticcall) | 1,000,000 | 1,477,259 | 45,685.0 | 45.69 | 285,164.0 | 285,164.0 | 285,164.0 |
| Blockhash | 1,000,000 | 1,477,259 | 26,900.0 | 26.90 | 214,604.0 | 213,816.4 | 211,916.0 |
| Selfdestruct Created (No Value) | 965,720 | 1,477,259 | 13,653.0 | 13.65 | 125,900.0 | 124,223.1 | 120,524.0 |
| Storage Access Cold (Absent Slots True, Sstore New Value, Out Of Gas) | 1,000,000 | 1,477,259 | 15,477.0 | 15.48 | 297,484.0 | 297,476.3 | 295,916.0 |

## Statistics

### Proving Statistics
- **Proof Size**: Min: 1,477,259 bytes, Max: 1,477,259 bytes, Avg: 1,477,259 bytes
- **Proving Time**: Min: 12143.0ms, Max: 579904.0ms, Avg: 89908.9ms
- **Peak Memory**: Min: 108428.0MB, Max: 305100.0MB, Avg: 246656.6MB

