# zkEVM Benchmark Results

Generated on: 2025-10-10 02:43:47

## Folder: zkevm-metrics-sp1-1M

## Proving Metrics

| Benchmark | Gas Used | Proof Size (bytes) | Proving Time (ms) | Proving Time (s) | Peak Memory (MB) | Avg Memory (MB) | Initial Memory (MB) |
|---|---|---|---|---|---|---|---|
| Block Full Data (Non Zero Byte) | 1,000,000 | 1,477,259 | 15,078.0 | 15.08 | 284,940.0 | 284,940.0 | 284,940.0 |
| Block Full Of Ether Transfers (A To A) | 987,000 | 1,477,259 | 28,098.0 | 28.10 | 382,156.0 | 382,156.0 | 382,156.0 |
| Block Full Of Ether Transfers (Diff Acc To Diff Acc) | 987,000 | 1,477,259 | 28,843.0 | 28.84 | 375,660.0 | 375,547.3 | 375,436.0 |
| Bytecode Single Opcode (Call) | 1,000,000 | 1,477,259 | 126,038.0 | 126.04 | 282,700.0 | 282,684.1 | 266,572.0 |
| Create (0Bytes, Create2) | 1,000,000 | 1,477,259 | 15,280.0 | 15.28 | 366,924.0 | 366,924.0 | 366,924.0 |
| Create (0Bytes, Create) | 1,000,000 | 1,477,259 | 15,453.0 | 15.45 | 343,404.0 | 343,083.4 | 341,612.0 |
| Create (0.25X Max Code, Zero Data, Create2) | 1,000,000 | 1,477,259 | 15,706.0 | 15.71 | 306,892.0 | 306,892.0 | 306,892.0 |
| Create (0.75X Max Code Size With Non, Zero Data, Create2) | 1,000,000 | 1,477,259 | 15,590.0 | 15.59 | 376,108.0 | 376,108.0 | 376,108.0 |
| Create (Max Code, Create) | 1,000,000 | 1,477,259 | 14,872.0 | 14.87 | 211,244.0 | 211,194.5 | 209,676.0 |
| Creates Collisions (Create2) | 1,000,000 | 1,477,259 | 15,511.0 | 15.51 | 307,116.0 | 307,116.0 | 307,116.0 |
| Creates Collisions (Create) | 1,000,000 | 1,477,259 | 16,300.0 | 16.30 | 282,700.0 | 282,700.0 | 282,700.0 |
| Initcode Jumpdest Analysis (5B) | 1,000,000 | 1,477,259 | 28,091.0 | 28.09 | 307,116.0 | 307,049.8 | 306,892.0 |
| Initcode Jumpdest Analysis (605B5B) | 1,000,000 | 1,477,259 | 23,275.0 | 23.27 | 258,284.0 | 258,284.0 | 258,284.0 |
| Initcode Jumpdest Analysis (615B5B5B) | 1,000,000 | 1,477,259 | 22,027.0 | 22.03 | 293,228.0 | 293,228.0 | 293,228.0 |
| Initcode Jumpdest Analysis (615B5B) | 1,000,000 | 1,477,259 | 20,860.0 | 20.86 | 376,108.0 | 376,108.0 | 376,108.0 |
| Amortized Bn128 Pairings | 1,000,000 | 1,477,259 | 117,593.0 | 117.59 | 184,812.0 | 182,167.0 | 175,404.0 |
| Empty Block | N/A | 1,477,259 | 11,330.0 | 11.33 | 340,044.0 | 339,857.3 | 339,372.0 |
| Binop Simple (Byte) | 1,000,000 | 1,477,259 | 26,708.0 | 26.71 | 260,748.0 | 260,678.2 | 260,300.0 |
| Binop Simple (Div, 1) | 1,000,000 | 1,477,259 | 93,288.0 | 93.29 | 196,460.0 | 194,556.4 | 191,756.0 |
| Binop Simple (Mul) | 1,000,000 | 1,477,259 | 39,586.0 | 39.59 | 284,940.0 | 284,940.0 | 284,940.0 |
| Binop Simple (Or) | 1,000,000 | 1,477,259 | 25,646.0 | 25.65 | 265,228.0 | 265,228.0 | 265,228.0 |
| Binop Simple (Sar) | 1,000,000 | 1,477,259 | 43,123.0 | 43.12 | 284,940.0 | 284,940.0 | 284,940.0 |
| Binop Simple (Shl) | 1,000,000 | 1,477,259 | 38,367.0 | 38.37 | 382,380.0 | 382,380.0 | 382,380.0 |
| Binop Simple (Shr) | 1,000,000 | 1,477,259 | 39,182.0 | 39.18 | 359,084.0 | 359,084.0 | 359,084.0 |
| Binop Simple (Slt) | 1,000,000 | 1,477,259 | 29,254.0 | 29.25 | 284,940.0 | 284,940.0 | 284,940.0 |
| Binop Simple (Smod) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Binop Simple (Sub) | 1,000,000 | 1,477,259 | 28,220.0 | 28.22 | 377,900.0 | 377,900.0 | 377,900.0 |
| Blobhash (Six Blobs, Access Latest) | 1,000,000 | 1,477,259 | 33,311.0 | 33.31 | 376,108.0 | 376,108.0 | 376,108.0 |
| Calldatasize (Calldata Length 1000) | 1,000,000 | 1,477,259 | 24,782.0 | 24.78 | 375,884.0 | 375,884.0 | 375,884.0 |
| Callvalue (From Origin False, Non Zero Value False) | 1,000,000 | 1,477,259 | 24,136.0 | 24.14 | 367,148.0 | 367,148.0 | 367,148.0 |
| Callvalue (From Origin False, Non Zero Value True) | 1,000,000 | 1,477,259 | 23,834.0 | 23.83 | 382,380.0 | 382,380.0 | 382,380.0 |
| Callvalue (From Origin True, Non Zero Value True) | 1,000,000 | 1,477,259 | 24,466.0 | 24.47 | 306,892.0 | 306,892.0 | 306,892.0 |
| Dup (Dup10) | 1,000,000 | 1,477,259 | 25,524.0 | 25.52 | 355,948.0 | 355,605.4 | 355,500.0 |
| Dup (Dup13) | 1,000,000 | 1,477,259 | 24,389.0 | 24.39 | 257,388.0 | 257,388.0 | 257,388.0 |
| Dup (Dup15) | 1,000,000 | 1,477,259 | 25,031.0 | 25.03 | 307,116.0 | 307,116.0 | 307,116.0 |
| Dup (Dup1) | 1,000,000 | 1,477,259 | 25,114.0 | 25.11 | 376,108.0 | 376,108.0 | 376,108.0 |
| Dup (Dup3) | 1,000,000 | 1,477,259 | 23,875.0 | 23.88 | 306,892.0 | 306,892.0 | 306,892.0 |
| Dup (Dup4) | 1,000,000 | 1,477,259 | 25,324.0 | 25.32 | 333,100.0 | 332,327.3 | 330,860.0 |
| Dup (Dup5) | 1,000,000 | 1,477,259 | 25,134.0 | 25.13 | 376,108.0 | 376,108.0 | 376,108.0 |
| Jumpdests | 1,000,000 | 1,477,259 | 25,799.0 | 25.80 | 381,036.0 | 380,876.0 | 380,812.0 |
| Memory Access (Small Mem, Uninit Offset, Off 0, Mload) | 1,000,000 | 1,477,259 | 31,528.0 | 31.53 | 376,556.0 | 376,493.3 | 376,108.0 |
| Memory Access (Small Mem, Uninit Offset, Off 0, Mstore8) | 1,000,000 | 1,477,259 | 28,219.0 | 28.22 | 307,116.0 | 307,116.0 | 307,116.0 |
| Memory Access (Small Mem, Uninit Offset, Off 0, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Small Mem, Uninit Offset, Off 31, Mstore8) | 1,000,000 | 1,477,259 | 27,782.0 | 27.78 | 259,404.0 | 258,906.7 | 258,732.0 |
| Memory Access (Small Mem, Uninit Offset, Off 31, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Small Mem, Init Offset, Off 0, Mstore8) | 1,000,000 | 1,477,259 | 28,322.0 | 28.32 | 307,116.0 | 307,116.0 | 307,116.0 |
| Memory Access (Small Mem, Init Offset, Off 0, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Small Mem, Init Offset, Off 1, Mload) | 1,000,000 | 1,477,259 | 31,205.0 | 31.20 | 306,892.0 | 306,892.0 | 306,892.0 |
| Memory Access (Small Mem, Init Offset, Off 1, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Small Mem, Init Offset, Off 31, Mload) | 1,000,000 | 1,477,259 | 31,476.0 | 31.48 | 382,604.0 | 382,604.0 | 382,604.0 |
| Memory Access (Small Mem, Init Offset, Off 31, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Big Mem, Uninit Offset, Off 0, Mstore8) | 1,000,000 | 1,477,259 | 23,715.0 | 23.71 | 108,428.0 | 107,064.0 | 96,332.0 |
| Memory Access (Big Mem, Uninit Offset, Off 0, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Big Mem, Uninit Offset, Off 31, Mload) | 1,000,000 | 1,477,259 | 31,128.0 | 31.13 | 377,900.0 | 377,900.0 | 377,900.0 |
| Memory Access (Big Mem, Uninit Offset, Off 31, Mstore8) | 1,000,000 | 1,477,259 | 27,333.0 | 27.33 | 285,164.0 | 285,164.0 | 285,164.0 |
| Memory Access (Big Mem, Uninit Offset, Off 31, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Big Mem, Init Offset, Off 0, Mload) | 1,000,000 | 1,477,259 | 30,741.0 | 30.74 | 338,924.0 | 338,912.1 | 338,476.0 |
| Memory Access (Big Mem, Init Offset, Off 0, Mstore8) | 1,000,000 | 1,477,259 | 28,950.0 | 28.95 | 366,700.0 | 366,630.2 | 366,476.0 |
| Memory Access (Big Mem, Init Offset, Off 0, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Memory Access (Big Mem, Init Offset, Off 31, Mload) | 1,000,000 | 1,477,259 | 31,237.0 | 31.24 | 339,372.0 | 339,372.0 | 339,372.0 |
| Memory Access (Big Mem, Init Offset, Off 31, Mstore) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Mod (Op Mod, Mod Bits 191) | 1,000,000 | 1,477,259 | 119,036.0 | 119.04 | 257,388.0 | 257,388.0 | 257,388.0 |
| Mod (Op Mod, Mod Bits 255) | 1,000,000 | 1,477,259 | 91,859.0 | 91.86 | 294,348.0 | 294,070.5 | 293,676.0 |
| Mod (Op Smod, Mod Bits 127) | 1,000,000 | 1,477,259 | 95,914.0 | 95.91 | 366,924.0 | 366,924.0 | 366,924.0 |
| Mod (Op Smod, Mod Bits 191) | 1,000,000 | 1,477,259 | 125,758.0 | 125.76 | 257,388.0 | 257,388.0 | 257,388.0 |
| Modarith (Op Addmod, Mod Bits 127) | 1,000,000 | 1,477,259 | 107,497.0 | 107.50 | 366,476.0 | 366,476.0 | 366,476.0 |
| Modarith (Op Addmod, Mod Bits 191) | 1,000,000 | 1,477,259 | 139,138.0 | 139.14 | 293,228.0 | 292,933.0 | 292,108.0 |
| Modarith (Op Addmod, Mod Bits 255) | 1,000,000 | 1,477,259 | 104,544.0 | 104.54 | 191,308.0 | 189,120.5 | 185,036.0 |
| Modarith (Op Mulmod, Mod Bits 127) | 1,000,000 | 1,477,259 | 129,625.0 | 129.62 | 338,252.0 | 337,353.4 | 335,116.0 |
| Modarith (Op Mulmod, Mod Bits 255) | 1,000,000 | 1,477,259 | 176,896.0 | 176.90 | 367,372.0 | 367,372.0 | 367,372.0 |
| Modarith (Op Mulmod, Mod Bits 63) | 1,000,000 | 1,477,259 | 101,935.0 | 101.94 | 266,572.0 | 266,572.0 | 266,572.0 |
| Modexp (Mod 1045 Gas Base Heavy) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp (Mod 400 Gas Exp Heavy) | 1,000,000 | 1,477,259 | 467,706.0 | 467.71 | 266,348.0 | 265,911.9 | 265,452.0 |
| Modexp (Mod 408 Gas Balanced) | 1,000,000 | 1,477,259 | 333,925.0 | 333.93 | 306,892.0 | 306,892.0 | 306,892.0 |
| Modexp (Mod 600 As Balanced) | 1,000,000 | 1,477,259 | 346,783.0 | 346.78 | 382,156.0 | 381,707.2 | 381,036.0 |
| Modexp (Mod 600 Gas Exp Heavy) | 1,000,000 | 1,477,259 | 516,912.0 | 516.91 | 382,380.0 | 382,380.0 | 382,380.0 |
| Modexp (Mod 800 Gas Base Heavy) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp (Mod Even 16B Exp 320) | 1,000,000 | 1,477,259 | 505,963.0 | 505.96 | 305,100.0 | 302,981.8 | 299,052.0 |
| Modexp (Mod Even 32B Exp 256) | 1,000,000 | 1,477,259 | 371,034.0 | 371.03 | 143,148.0 | 141,204.8 | 137,100.0 |
| Modexp (Mod Even 512B Exp 1024) | 1,000,000 | 1,477,259 | 15,191.0 | 15.19 | 375,884.0 | 375,845.6 | 375,660.0 |
| Modexp (Mod Odd 1024B Exp 1024) | 1,000,000 | 1,477,259 | 15,580.0 | 15.58 | 307,116.0 | 307,116.0 | 307,116.0 |
| Modexp (Mod Odd 32B Exp 96) | 1,000,000 | 1,477,259 | 364,058.0 | 364.06 | 359,756.0 | 359,746.2 | 359,532.0 |
| Modexp (Mod Pawel 3) | 1,000,000 | 1,477,259 | 393,345.0 | 393.35 | 284,940.0 | 283,471.3 | 282,700.0 |
| Modexp (Mod Vul Common 1360N1) | 1,000,000 | 1,477,259 | 333,810.0 | 333.81 | 293,676.0 | 293,478.7 | 293,228.0 |
| Modexp (Mod Vul Common 1360N2) | 1,000,000 | 1,477,259 | 286,047.0 | 286.05 | 137,100.0 | 134,002.2 | 125,900.0 |
| Modexp (Mod Vul Common 200N3) | 1,000,000 | 1,477,259 | 222,973.0 | 222.97 | 355,500.0 | 353,526.9 | 350,572.0 |
| Modexp (Mod Vul Example 1) | 1,000,000 | 1,477,259 | 387,750.0 | 387.75 | 266,572.0 | 266,572.0 | 266,348.0 |
| Modexp (Mod Vul Guido 1 Even) | 1,000,000 | 1,477,259 | 289,152.0 | 289.15 | 207,660.0 | 203,956.2 | 196,460.0 |
| Modexp (Mod Vul Marius 1 Even) | 1,000,000 | 1,477,259 | 417,846.0 | 417.85 | 358,860.0 | 357,557.0 | 355,948.0 |
| Modexp (Mod Vul Nagydani 1 Pow 0X10001) | 1,000,000 | 1,477,259 | 277,129.0 | 277.13 | 376,780.0 | 376,697.4 | 376,556.0 |
| Modexp (Mod Vul Nagydani 1 Qube) | 1,000,000 | 1,477,259 | 211,569.0 | 211.57 | 295,468.0 | 294,960.3 | 294,348.0 |
| Modexp (Mod Vul Nagydani 2 Pow 0X10001) | 1,000,000 | 1,477,259 | 288,465.0 | 288.46 | 366,476.0 | 366,476.0 | 366,476.0 |
| Modexp (Mod Vul Nagydani 2 Square) | 1,000,000 | 1,477,259 | 579,904.0 | 579.90 | 265,228.0 | 264,156.9 | 261,868.0 |
| Modexp (Mod Vul Nagydani 3 Qube) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp (Mod Vul Nagydani 5 Square) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp (Mod Vul Pawel 1 Exp Heavy) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Modexp (Mod Vul Pawel 4 Exp Heavy) | 1,000,000 | 1,477,259 | 340,379.0 | 340.38 | 330,636.0 | 325,147.6 | 317,644.0 |
| Precompile Fixed Cost (Blake2F) | 1,000,000 | 1,477,259 | 296,391.0 | 296.39 | 257,836.0 | 257,785.2 | 257,388.0 |
| Precompile Fixed Cost (Bls12 Fp To G2) | 1,000,000 | 1,477,259 | 426,115.0 | 426.12 | 380,812.0 | 380,121.1 | 379,244.0 |
| Precompile Fixed Cost (Bls12 G2Msm) | 1,000,000 | 1,477,259 | 187,029.0 | 187.03 | 290,316.0 | 289,626.7 | 287,180.0 |
| Precompile Fixed Cost (Bn128 Add 1 2) | 1,000,000 | 1,477,259 | 55,301.0 | 55.30 | 382,604.0 | 382,604.0 | 382,604.0 |
| Precompile Fixed Cost (Bn128 Add Infinities) | 1,000,000 | 1,477,259 | 35,351.0 | 35.35 | 367,148.0 | 367,131.7 | 366,924.0 |
| Precompile Fixed Cost (Bn128 Mul) | 1,000,000 | 1,477,259 | 166,347.0 | 166.35 | 156,364.0 | 154,055.2 | 148,076.0 |
| Precompile Fixed Cost (Bn128 Two Pairings Empty) | 1,000,000 | 1,477,259 | 15,400.0 | 15.40 | 340,268.0 | 340,268.0 | 340,268.0 |
| Precompile Only Data Input (Sha2, 256) | 1,000,000 | 1,477,259 | 71,149.0 | 71.15 | 381,036.0 | 381,036.0 | 381,036.0 |
| Push (Push10) | 1,000,000 | 1,477,259 | 27,942.0 | 27.94 | 376,556.0 | 376,556.0 | 376,556.0 |
| Push (Push11) | 1,000,000 | 1,477,259 | 30,575.0 | 30.57 | 377,900.0 | 377,900.0 | 377,900.0 |
| Push (Push13) | 1,000,000 | 1,477,259 | 30,040.0 | 30.04 | 282,700.0 | 282,700.0 | 282,700.0 |
| Push (Push15) | 1,000,000 | 1,477,259 | 31,158.0 | 31.16 | 223,340.0 | 221,317.6 | 218,636.0 |
| Push (Push16) | 1,000,000 | 1,477,259 | 30,964.0 | 30.96 | 307,116.0 | 307,116.0 | 307,116.0 |
| Push (Push19) | 1,000,000 | 1,477,259 | 33,413.0 | 33.41 | 376,780.0 | 376,780.0 | 376,780.0 |
| Push (Push27) | 1,000,000 | 1,477,259 | 35,363.0 | 35.36 | 339,148.0 | 339,148.0 | 339,148.0 |
| Push (Push30) | 1,000,000 | 1,477,259 | 38,353.0 | 38.35 | 307,116.0 | 307,116.0 | 307,116.0 |
| Push (Push31) | 1,000,000 | 1,477,259 | 37,907.0 | 37.91 | 257,836.0 | 257,836.0 | 257,836.0 |
| Push (Push8) | 1,000,000 | 1,477,259 | 26,824.0 | 26.82 | 265,228.0 | 265,228.0 | 265,228.0 |
| Return Revert (1Kib Of Non, Zero Data, Return) | 1,000,000 | 1,477,259 | 32,700.0 | 32.70 | 317,420.0 | 316,448.3 | 312,716.0 |
| Return Revert (1Kib Of Non, Zero Data, Revert) | 1,000,000 | 1,477,259 | 33,490.0 | 33.49 | 382,156.0 | 382,156.0 | 382,156.0 |
| Return Revert (1Mib Of Non, Zero Data, Return) | 1,000,000 | 1,477,259 | 15,013.0 | 15.01 | 295,692.0 | 295,692.0 | 295,468.0 |
| Return Revert (1Mib Of Non, Zero Data, Revert) | 1,000,000 | 1,477,259 | 16,727.0 | 16.73 | 284,940.0 | 284,940.0 | 284,940.0 |
| Return Revert (1Mib Of Zero Data, Revert) | 1,000,000 | 1,477,259 | 13,297.0 | 13.30 | 158,380.0 | 157,840.1 | 156,588.0 |
| Return Revert (Empty, Return) | 1,000,000 | 1,477,259 | 47,038.0 | 47.04 | 366,700.0 | 366,700.0 | 366,700.0 |
| Return Revert (Empty, Revert) | 1,000,000 | 1,477,259 | 49,341.0 | 49.34 | 257,388.0 | 257,388.0 | 257,388.0 |
| Returndatasize Nonzero (Returned Size 0, Return Data Style Returndatastyle.Revert) | 1,000,000 | 1,477,259 | 23,629.0 | 23.63 | 307,116.0 | 307,116.0 | 307,116.0 |
| Returndatasize Nonzero (Returned Size 1, Return Data Style Returndatastyle.Return) | 1,000,000 | 1,477,259 | 24,086.0 | 24.09 | 308,460.0 | 308,337.9 | 308,236.0 |
| Returndatasize Nonzero (Returned Size 1, Return Data Style Returndatastyle.Revert) | 1,000,000 | 1,477,259 | 23,898.0 | 23.90 | 265,228.0 | 265,228.0 | 265,228.0 |
| Shifts (Shift Right Sar) | 1,000,000 | 1,477,259 | 40,901.0 | 40.90 | 377,676.0 | 377,676.0 | 377,676.0 |
| Shifts (Shift Right Shr) | 1,000,000 | 1,477,259 | 37,472.0 | 37.47 | 306,892.0 | 306,892.0 | 306,892.0 |
| Swap (Swap10) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap11) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap13) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap14) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap15) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap2) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap4) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap5) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Swap (Swap8) | 1,000,000 | N/A | N/A | N/A | N/A | N/A | N/A |
| Tload (Val Mut False, Key Mut True) | 1,000,000 | 1,477,259 | 16,944.0 | 16.94 | 366,924.0 | 366,924.0 | 366,924.0 |
| Tload (Val Mut True, Key Mut False) | 1,000,000 | 1,477,259 | 16,569.0 | 16.57 | 144,492.0 | 144,327.7 | 143,148.0 |
| Tstore (Dense Val Mut True, Key Mut False) | 1,000,000 | 1,477,259 | 34,016.0 | 34.02 | 266,572.0 | 266,572.0 | 266,572.0 |
| Zero Param (Address) | 1,000,000 | 1,477,259 | 38,521.0 | 38.52 | 265,452.0 | 265,296.9 | 265,228.0 |
| Zero Param (Basefee) | 1,000,000 | 1,477,259 | 28,515.0 | 28.52 | 375,884.0 | 375,884.0 | 375,884.0 |
| Zero Param (Caller) | 1,000,000 | 1,477,259 | 38,402.0 | 38.40 | 257,836.0 | 257,836.0 | 257,836.0 |
| Zero Param (Chainid) | 1,000,000 | 1,477,259 | 27,079.0 | 27.08 | 378,124.0 | 378,085.2 | 377,900.0 |
| Zero Param (Gaslimit) | 1,000,000 | 1,477,259 | 25,813.0 | 25.81 | 377,676.0 | 377,573.6 | 376,780.0 |
| Zero Param (Prevrandao) | 1,000,000 | 1,477,259 | 83,469.0 | 83.47 | 382,604.0 | 382,604.0 | 382,604.0 |
| Calldatacopy (Zero Data, Dynamic Src Dst, 0Bytes, Transaction) | 1,000,000 | 1,477,259 | 34,962.0 | 34.96 | 258,508.0 | 258,348.7 | 258,284.0 |
| Calldatacopy (Zero Data, Dynamic Src Dst, 100Bytes, Call) | 1,000,000 | 1,477,259 | 31,441.0 | 31.44 | 307,116.0 | 307,116.0 | 307,116.0 |
| Calldatacopy (Zero Data, Dynamic Src Dst, 1Mib, Call) | 1,000,000 | 1,477,259 | 14,697.0 | 14.70 | 217,740.0 | 216,791.7 | 214,828.0 |
| Calldatacopy (Zero Data, Fixed Src Dst, 0Bytes, Call) | 1,000,000 | 1,477,259 | 25,250.0 | 25.25 | 148,076.0 | 147,153.6 | 144,716.0 |
| Calldatacopy (Non Zero, Dynamic Src Dst, 100Bytes, Call) | 1,000,000 | 1,477,259 | 32,448.0 | 32.45 | 258,732.0 | 258,621.6 | 258,508.0 |
| Calldatacopy (Non Zero, Dynamic Src Dst, 100Bytes, Transaction) | 1,000,000 | 1,477,259 | 31,606.0 | 31.61 | 307,116.0 | 307,116.0 | 307,116.0 |
| Calldatacopy (Non Zero, Dynamic Src Dst, 10Kib, Call) | 1,000,000 | 1,477,259 | 21,111.0 | 21.11 | 382,604.0 | 382,604.0 | 382,604.0 |
| Calldatacopy (Non Zero, Fixed Src Dst, 100Bytes, Transaction) | 1,000,000 | 1,477,259 | 24,160.0 | 24.16 | 382,156.0 | 382,156.0 | 382,156.0 |
| Codecopy (Dynamic Src Dst, 0.50X Max Code Size) | 1,000,000 | 1,477,259 | 21,771.0 | 21.77 | 376,108.0 | 376,108.0 | 376,108.0 |
| Codecopy (Dynamic Src Dst, 0.75X Max Code Size) | 1,000,000 | 1,477,259 | 22,381.0 | 22.38 | 346,316.0 | 345,486.4 | 343,404.0 |
| Codecopy (Fixed Src Dst, 0.50X Max Code Size) | 1,000,000 | 1,477,259 | 20,154.0 | 20.15 | 227,148.0 | 225,606.4 | 223,340.0 |
| Mcopy (Dynamic Src Dst, 100Bytes) | 1,000,000 | 1,477,259 | 33,519.0 | 33.52 | 307,116.0 | 307,116.0 | 307,116.0 |
| Mcopy (Dynamic Src Dst, 1Mib) | 1,000,000 | 1,477,259 | 16,036.0 | 16.04 | 381,036.0 | 381,036.0 | 381,036.0 |
| Mcopy (Fixed Src Dst, 0Bytes) | 1,000,000 | 1,477,259 | 26,320.0 | 26.32 | 382,380.0 | 382,380.0 | 382,156.0 |
| Mcopy (Fixed Src Dst, 10Kib) | 1,000,000 | 1,477,259 | 23,397.0 | 23.40 | 382,604.0 | 382,604.0 | 382,380.0 |
| Returndatacopy (Fixed Dst False, 100Bytes) | 1,000,000 | 1,477,259 | 28,951.0 | 28.95 | 162,860.0 | 161,860.1 | 159,500.0 |
| Returndatacopy (Fixed Dst False, 10Kib) | 1,000,000 | 1,477,259 | 24,720.0 | 24.72 | 382,156.0 | 382,156.0 | 382,156.0 |
| Returndatacopy (Fixed Dst False, 1Mib) | 1,000,000 | 1,477,259 | 15,468.0 | 15.47 | 382,156.0 | 382,156.0 | 382,156.0 |
| Returndatacopy (Fixed Dst True, 0Bytes) | 1,000,000 | 1,477,259 | 29,870.0 | 29.87 | 285,164.0 | 285,164.0 | 284,940.0 |
| Returndatacopy (Fixed Dst True, 100Bytes) | 1,000,000 | 1,477,259 | 24,993.0 | 24.99 | 350,348.0 | 348,607.4 | 347,436.0 |
| Returndatacopy (Fixed Dst True, 10Kib) | 1,000,000 | 1,477,259 | 20,626.0 | 20.63 | 310,252.0 | 310,185.7 | 308,460.0 |
| Returndatacopy (Fixed Dst True, 1Mib) | 1,000,000 | 1,477,259 | 15,573.0 | 15.57 | 299,052.0 | 298,921.9 | 297,484.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 0 Bytes Data, Log0) | 1,000,000 | 1,477,259 | 16,573.0 | 16.57 | 367,372.0 | 367,372.0 | 367,372.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 0 Bytes Data, Log1) | 1,000,000 | 1,477,259 | 15,546.0 | 15.55 | 257,836.0 | 257,836.0 | 257,836.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 0 Bytes Data, Log4) | 1,000,000 | 1,477,259 | 15,680.0 | 15.68 | 231,852.0 | 230,555.2 | 227,596.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Non Zero Data, Log1) | 1,000,000 | 1,477,259 | 14,879.0 | 14.88 | 379,020.0 | 378,738.0 | 378,124.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Non Zero Data, Log2) | 1,000,000 | 1,477,259 | 15,379.0 | 15.38 | 311,820.0 | 311,648.9 | 310,252.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Non Zero Data, Log4) | 1,000,000 | 1,477,259 | 14,818.0 | 14.82 | 166,444.0 | 165,802.5 | 163,532.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Zeros Data, Log0) | 1,000,000 | 1,477,259 | 14,920.0 | 14.92 | 260,300.0 | 260,300.0 | 260,300.0 |
| Log Opcodes (Dynamic, Non Zero Topic, 1 Mib Zeros Data, Log4) | 1,000,000 | 1,477,259 | 14,974.0 | 14.97 | 284,940.0 | 284,940.0 | 284,940.0 |
| Log Opcodes (Dynamic, Zero Topic, 0 Bytes Data, Log1) | 1,000,000 | 1,477,259 | 15,314.0 | 15.31 | 367,372.0 | 367,372.0 | 367,372.0 |
| Log Opcodes (Dynamic, Zero Topic, 0 Bytes Data, Log2) | 1,000,000 | 1,477,259 | 15,597.0 | 15.60 | 366,476.0 | 366,476.0 | 366,476.0 |
| Log Opcodes (Dynamic, Zero Topic, 0 Bytes Data, Log4) | 1,000,000 | 1,477,259 | 14,529.0 | 14.53 | 174,732.0 | 173,134.9 | 170,028.0 |
| Log Opcodes (Dynamic, Zero Topic, 1 Mib Non Zero Data, Log1) | 1,000,000 | 1,477,259 | 15,145.0 | 15.14 | 367,372.0 | 367,372.0 | 367,372.0 |
| Log Opcodes (Dynamic, Zero Topic, 1 Mib Non Zero Data, Log2) | 1,000,000 | 1,477,259 | 15,915.0 | 15.91 | 359,532.0 | 359,532.0 | 359,532.0 |
| Log Opcodes (Dynamic, Zero Topic, 1 Mib Non Zero Data, Log3) | 1,000,000 | 1,477,259 | 12,143.0 | 12.14 | 114,476.0 | 112,968.7 | 109,772.0 |
| Log Opcodes (Dynamic, Zero Topic, 1 Mib Zeros Data, Log4) | 1,000,000 | 1,477,259 | 14,893.0 | 14.89 | 287,180.0 | 287,038.6 | 285,164.0 |
| Log Opcodes (Fixed, Non Zero Topic, 0 Bytes Data, Log2) | 1,000,000 | 1,477,259 | 15,508.0 | 15.51 | 307,116.0 | 307,116.0 | 307,116.0 |
| Log Opcodes (Fixed, Non Zero Topic, 0 Bytes Data, Log3) | 1,000,000 | 1,477,259 | 15,434.0 | 15.43 | 382,156.0 | 382,156.0 | 382,156.0 |
| Log Opcodes (Fixed, Non Zero Topic, 1 Mib Non Zero Data, Log0) | 1,000,000 | 1,477,259 | 15,181.0 | 15.18 | 359,756.0 | 359,756.0 | 359,756.0 |
| Log Opcodes (Fixed, Non Zero Topic, 1 Mib Non Zero Data, Log2) | 1,000,000 | 1,477,259 | 15,161.0 | 15.16 | 285,164.0 | 285,164.0 | 285,164.0 |
| Log Opcodes (Fixed, Non Zero Topic, 1 Mib Non Zero Data, Log3) | 1,000,000 | 1,477,259 | 14,980.0 | 14.98 | 347,212.0 | 347,203.0 | 346,316.0 |
| Log Opcodes (Fixed, Non Zero Topic, 1 Mib Zeros Data, Log1) | 1,000,000 | 1,477,259 | 14,943.0 | 14.94 | 341,612.0 | 341,375.1 | 340,716.0 |
| Log Opcodes (Fixed, Non Zero Topic, 1 Mib Zeros Data, Log2) | 1,000,000 | 1,477,259 | 15,128.0 | 15.13 | 377,900.0 | 377,900.0 | 377,900.0 |
| Log Opcodes (Fixed, Zero Topic, 0 Bytes Data, Log1) | 1,000,000 | 1,477,259 | 15,061.0 | 15.06 | 285,164.0 | 285,164.0 | 285,164.0 |
| Log Opcodes (Fixed, Zero Topic, 1 Mib Non Zero Data, Log2) | 1,000,000 | 1,477,259 | 15,032.0 | 15.03 | 284,940.0 | 284,940.0 | 284,940.0 |
| Log Opcodes (Fixed, Zero Topic, 1 Mib Zeros Data, Log1) | 1,000,000 | 1,477,259 | 15,420.0 | 15.42 | 335,116.0 | 334,366.5 | 333,100.0 |
| Log Opcodes (Fixed, Zero Topic, 1 Mib Zeros Data, Log2) | 1,000,000 | 1,477,259 | 14,430.0 | 14.43 | 307,116.0 | 307,116.0 | 307,116.0 |
| Log Opcodes (Fixed, Zero Topic, 1 Mib Zeros Data, Log4) | 1,000,000 | 1,477,259 | 15,813.0 | 15.81 | 305,100.0 | 305,100.0 | 305,100.0 |
| Address State Cold (Absent Accounts True, Balance) | 1,000,000 | 1,477,259 | 16,468.0 | 16.47 | 359,756.0 | 359,756.0 | 359,756.0 |
| Address State Warm (Present Target, Delegatecall) | 1,000,000 | 1,477,259 | 41,838.0 | 41.84 | 209,676.0 | 209,418.0 | 207,884.0 |
| Address State Warm (Present Target, Extcodehash) | 1,000,000 | 1,477,259 | 21,594.0 | 21.59 | 308,236.0 | 308,236.0 | 308,236.0 |
| Address State Warm (Present Target, Staticcall) | 1,000,000 | 1,477,259 | 50,093.0 | 50.09 | 382,380.0 | 382,380.0 | 382,380.0 |
| Address State Warm (Absent Target, Balance) | 1,000,000 | 1,477,259 | 19,574.0 | 19.57 | 367,372.0 | 367,372.0 | 367,372.0 |
| Address State Warm (Absent Target, Call) | 1,000,000 | 1,477,259 | 44,549.0 | 44.55 | 284,940.0 | 284,940.0 | 284,940.0 |
| Address State Warm (Absent Target, Delegatecall) | 1,000,000 | 1,477,259 | 38,464.0 | 38.46 | 292,108.0 | 291,890.8 | 290,316.0 |
| Address State Warm (Absent Target, Extcodesize) | 1,000,000 | 1,477,259 | 20,564.0 | 20.56 | 306,892.0 | 306,892.0 | 306,892.0 |
| Address State Warm (Absent Target, Staticcall) | 1,000,000 | 1,477,259 | 45,685.0 | 45.69 | 285,164.0 | 285,164.0 | 285,164.0 |
| Blockhash | 1,000,000 | 1,477,259 | 26,900.0 | 26.90 | 214,604.0 | 213,816.4 | 211,916.0 |
| Extcodecopy Warm (512) | 1,000,000 | 1,477,259 | 20,264.0 | 20.26 | 382,380.0 | 382,380.0 | 382,380.0 |
| Extcodecopy Warm (5Kib) | 1,000,000 | 1,477,259 | 20,034.0 | 20.03 | 359,084.0 | 359,084.0 | 359,084.0 |
| Selfbalance | 1,000,000 | 1,477,259 | 79,537.0 | 79.54 | 382,380.0 | 382,380.0 | 382,380.0 |
| Selfdestruct Created (No Value) | 965,720 | 1,477,259 | 13,653.0 | 13.65 | 125,900.0 | 124,223.1 | 120,524.0 |
| Storage Access Cold (Absent Slots False, Ssload) | 999,749 | 1,477,259 | 19,533.0 | 19.53 | 307,116.0 | 307,116.0 | 307,116.0 |
| Storage Access Cold (Absent Slots False, Sstore New Value) | 998,957 | 1,477,259 | 17,839.0 | 17.84 | 340,268.0 | 340,268.0 | 340,268.0 |
| Storage Access Cold (Absent Slots True, Sstore New Value, Out Of Gas) | 1,000,000 | 1,477,259 | 15,477.0 | 15.48 | 297,484.0 | 297,476.3 | 295,916.0 |
| Storage Access Cold (Absent Slots True, Sstore New Value) | 995,207 | 1,477,259 | 15,339.0 | 15.34 | 307,116.0 | 307,116.0 | 307,116.0 |
| Storage Access Cold (Absent Slots True, Sstore Same Value, Out Of Gas) | 1,000,000 | 1,477,259 | 15,184.0 | 15.18 | 307,116.0 | 307,116.0 | 307,116.0 |
| Storage Access Warm (Sstore Same Value) | 1,000,000 | 1,477,259 | 30,082.0 | 30.08 | 376,108.0 | 376,108.0 | 376,108.0 |

