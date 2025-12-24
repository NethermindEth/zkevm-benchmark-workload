# Running SP1-Cluster Benchmarks

This guide explains how to run zkEVM benchmarks using the SP1-Cluster prover.

## Prerequisites

1. **SP1 Cluster Infrastructure** - Clone and set up the sp1-cluster-infra repository:
   ```bash
   git clone https://github.com/succinctlabs/sp1-cluster.git /root/sp1-cluster-infra
   ```

2. **Docker** with GPU support (NVIDIA runtime configured)

3. **Benchmark fixtures** in `./zkevm-fixtures-worstcase-prague` or similar directory

## Step 1: Start the SP1 Cluster

Before running benchmarks, you must start all cluster services in the correct order.

```bash
cd /root/sp1-cluster-infra/infra

# Clean start (removes old data - use if you have issues)
docker compose down -v

# Start all required services
docker compose up -d postgresql redis api coordinator cpu-node gpu0 gpu1 gpu2 gpu3
```

### Verify Services Are Running

Wait 10-15 seconds, then verify:

```bash
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
```

You should see ALL of these services running:
- `infra-postgresql-1` - Database
- `infra-redis-1` - Redis cache (port 6379)
- `infra-api-1` - **API service (port 50051)** ← Critical!
- `infra-coordinator-1` - Job coordinator
- `infra-cpu-node-1` - CPU worker
- `infra-gpu0-1` through `infra-gpu3-1` - GPU workers

**Important:** The `infra-api-1` container MUST be running and expose port 50051:
```bash
netstat -tlnp | grep 50051
# Should show: tcp  0  0  127.0.0.1:50051  0.0.0.0:*  LISTEN
```

### Troubleshooting Cluster Startup

If the API container keeps crashing, check logs:
```bash
docker logs infra-api-1 --tail 30
```

Common issues:
- **"failed to lookup address information"** - Network/DNS issue. Run `docker compose down -v` and restart.
- **"PoolTimedOut"** - PostgreSQL not ready. Wait and restart API: `docker compose restart api`
- **PostgreSQL "directory exists but is not empty"** - Corrupted data. Run `docker compose down -v` to clean volumes.

## Step 2: Run the Benchmark

Once the cluster is running, execute the benchmark:

```bash
cd /root/sp1-cluster/zkevm-benchmark-workload

./run_benchmark_and_gen_report.sh \
    --fixtures ./zkevm-fixtures-worstcase-prague \
    --action prove \
    --prover sp1-cluster \
    --cluster-endpoint "http://127.0.0.1:50051" \
    --cluster-redis-url "redis://:redispassword@127.0.0.1:6379/0" \
    --cluster-num-gpus 4 \
    --samples 1 \
    --output results-sp1-cluster
```

### Command Options

| Option | Description | Default |
|--------|-------------|---------|
| `--fixtures <dir>` | Path to fixture directory | Required |
| `--action <execute\|prove>` | Benchmark action | `execute` |
| `--prover <sp1\|risc0\|sp1-cluster>` | Prover to use | `sp1` |
| `--cluster-endpoint <url>` | SP1 cluster gRPC endpoint | Required for sp1-cluster |
| `--cluster-redis-url <url>` | Redis URL for artifacts | Required for sp1-cluster |
| `--cluster-num-gpus <n>` | Number of GPUs | `4` |
| `--samples <n>` | Number of benchmark runs | `3` |
| `--output <dir>` | Output directory | `results-<timestamp>` |
| `--force-rerun` | Re-run all benchmarks | Skip existing |

### Using Environment Variables

Instead of command-line options, you can set environment variables:

```bash
export SP1_CLUSTER_ENDPOINT="http://127.0.0.1:50051"
export SP1_CLUSTER_REDIS_URL="redis://:redispassword@127.0.0.1:6379/0"

./run_benchmark_and_gen_report.sh \
    --fixtures ./zkevm-fixtures-worstcase-prague \
    --action prove \
    --prover sp1-cluster \
    --samples 1
```

## Step 3: Monitor Progress

Check benchmark progress:

```bash
# Count completed fixtures
ls results-sp1-cluster/run_1/reth/sp1-cluster-v5.2.3/ | wc -l

# View a result file
cat "results-sp1-cluster/run_1/reth/sp1-cluster-v5.2.3/<fixture-name>.json"
```

Successful result example:
```json
{
  "name": "test_worstcase.py::test_worstcase_bytecode[...]",
  "timestamp_completed": "2025-12-17T08:32:45.018757290Z",
  "metadata": {
    "block_used_gas": 21550
  },
  "proving": {
    "success": {
      "proof_size": 1477226,
      "proving_time_ms": 6054
    }
  }
}
```

Failed result (connection error):
```json
{
  "proving": {
    "crashed": {
      "reason": "zkVM method error: Failed to connect to gRPC service: transport error"
    }
  }
}
```

## Step 4: Stop the Cluster

When done:

```bash
cd /root/sp1-cluster-infra/infra
docker compose down
```

To also remove data volumes:
```bash
docker compose down -v
```

## Quick Reference

### Full Working Example

```bash
# 1. Start cluster
cd /root/sp1-cluster-infra/infra
docker compose down -v  # Clean start
docker compose up -d postgresql redis api coordinator cpu-node gpu0 gpu1 gpu2 gpu3
sleep 15  # Wait for services

# 2. Verify API is running
docker ps | grep api
netstat -tlnp | grep 50051

# 3. Run benchmark
cd /root/sp1-cluster/zkevm-benchmark-workload
./run_benchmark_and_gen_report.sh \
    --fixtures ./zkevm-fixtures-worstcase-prague \
    --action prove \
    --prover sp1-cluster \
    --cluster-endpoint "http://127.0.0.1:50051" \
    --cluster-redis-url "redis://:redispassword@127.0.0.1:6379/0" \
    --cluster-num-gpus 4 \
    --samples 1

# 4. Stop cluster when done
cd /root/sp1-cluster-infra/infra
docker compose down
```

### Cluster Service Dependencies

```
postgresql
    └── api (port 50051) ← External endpoint
            └── coordinator
                    ├── cpu-node
                    ├── gpu0
                    ├── gpu1
                    ├── gpu2
                    └── gpu3
redis (port 6379) ← Artifact storage
```

All services must be on the same Docker network (`infra_default`) for DNS resolution to work.
