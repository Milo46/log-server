# Performance Benchmarks

This document tracks the performance benchmarks of the log-server over time.

## Benchmark Methodology

- **Tool**: Python concurrent benchmark script (`testing/benchmark.py`)
- **Test**: 1,000 concurrent requests with 100 parallel workers
- **Environment**: Docker Compose (PostgreSQL + log-server)
- **Network**: localhost (client and server on same machine)
- **Build Mode**: Release (`cargo build --release`)
- **Hardware**: [Add your hardware specs here]

## Benchmark History

### v1.0.0 - 2025-11-26

**Commit**: `dev@1535ca26` (dirty)  
**Date**: 2025-11-26 21:15:56

**Results**:
```
Total requests:       1,000
Concurrent workers:   100
Total time:          0.42 seconds
Success rate:        100% (1,000/1,000)

Throughput:          2,361.35 req/s

Response times (ms):
  Average:           33.65
  Min:               3.08
  Max:               94.42
  P50 (median):      31.42
  P95:               60.18
  P99:               77.43
```

**SRD Requirement**: ✅ PASS (1,000 req/s required, 2,361 req/s achieved)  
**Performance Margin**: +136.1% above requirement

**Notes**:
- First release benchmark
- Local testing (client and server on localhost)
- Docker Compose deployment (release build)
- PostgreSQL 16-alpine
- JSONB storage with GIN indexes
- JSON Schema Draft 7 validation per request
- Request ID middleware enabled

---

## How to Run Benchmarks

```bash
# 1. Start the server in release mode
docker compose up --build -d

# 2. Create test schema (if needed)
curl -X POST http://localhost:8080/schemas \
  -H "Content-Type: application/json" \
  -d @testing/fixtures/benchmark_schema.json

# 3. Update benchmark.py with the schema ID

# 4. Run benchmark
cd testing
python3 benchmark.py
```

## Performance Targets

- ✅ **Minimum**: 1,000 req/s (SRD requirement)
- ✅ **Target**: 2,000+ req/s (healthy margin)
- 🎯 **Stretch**: 5,000+ req/s (with optimizations)

## Known Optimizations

Potential improvements for future versions:
- [ ] Schema caching in memory
- [ ] Prepared statement optimization
- [ ] Connection pool tuning
- [ ] Batch log insertion
- [ ] Redis caching layer
