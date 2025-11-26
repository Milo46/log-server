#!/usr/bin/env python3
"""
Concurrent benchmark for log-server
Tests the "1000 requests per second" non-functional requirement
"""
import urllib.request
import json
import time
import subprocess
from datetime import datetime, timezone
from concurrent.futures import ThreadPoolExecutor, as_completed
from threading import Lock

lock = Lock()
response_times = []


def get_git_info():
    """Get current git commit hash and branch"""
    try:
        commit_hash = subprocess.check_output(
            ['git', 'rev-parse', 'HEAD'], 
            stderr=subprocess.DEVNULL
        ).decode('ascii').strip()[:8]
        
        branch = subprocess.check_output(
            ['git', 'rev-parse', '--abbrev-ref', 'HEAD'],
            stderr=subprocess.DEVNULL
        ).decode('ascii').strip()
        
        # Check for uncommitted changes
        status = subprocess.check_output(
            ['git', 'status', '--porcelain'],
            stderr=subprocess.DEVNULL
        ).decode('ascii').strip()
        
        dirty = " (dirty)" if status else ""
        
        return f"{branch}@{commit_hash}{dirty}"
    except:
        return "unknown"


def create_log_request_data(message: str, request_num: int) -> dict:
    return {
        "schema_id": "963e3380-569d-4c39-9681-5c2f7ab9b186",
        "log_data": {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "level": "INFO",
            "message": message,
            "request_id": f"bench-{request_num}"
        }
    }


def send_request(request_id: int) -> tuple[int, float, bool]:
    """Send a single request and return (request_id, response_time_ms, success)"""
    try:
        data = create_log_request_data(f"Benchmark request {request_id}", request_id)
        json_data = json.dumps(data).encode("utf-8")
        
        request = urllib.request.Request(
            "http://localhost:8080/logs",
            data=json_data,
            headers={"Content-Type": "application/json"}
        )
        
        start_time = time.perf_counter()
        response = urllib.request.urlopen(request, timeout=10)
        end_time = time.perf_counter()
        
        response_time_ms = (end_time - start_time) * 1000
        success = response.status == 201
        
        return (request_id, response_time_ms, success)
    except Exception as e:
        print(f"Request {request_id} failed: {e}")
        return (request_id, 0, False)


def run_benchmark(total_requests: int, concurrent_workers: int):
    """Run benchmark with specified concurrency"""
    git_info = get_git_info()
    
    print(f"\n{'='*60}")
    print(f"Starting benchmark:")
    print(f"  Git: {git_info}")
    print(f"  Date: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"  Total requests: {total_requests}")
    print(f"  Concurrent workers: {concurrent_workers}")
    print(f"{'='*60}\n")
    
    response_times.clear()
    successful_requests = 0
    failed_requests = 0
    
    start_time = time.perf_counter()
    
    with ThreadPoolExecutor(max_workers=concurrent_workers) as executor:
        futures = [executor.submit(send_request, i) for i in range(total_requests)]
        
        for future in as_completed(futures):
            request_id, response_time_ms, success = future.result()
            
            with lock:
                if success:
                    response_times.append(response_time_ms)
                    successful_requests += 1
                else:
                    failed_requests += 1
            
            total_completed = successful_requests + failed_requests
            if total_completed % 100 == 0:
                print(f"Progress: {total_completed}/{total_requests} requests completed...")
    
    end_time = time.perf_counter()
    total_time_seconds = end_time - start_time
    
    print(f"\n{'='*60}")
    print(f"BENCHMARK RESULTS:")
    print(f"{'='*60}")
    print(f"Total time: {total_time_seconds:.2f} seconds")
    print(f"Successful requests: {successful_requests}")
    print(f"Failed requests: {failed_requests}")
    print(f"\nThroughput: {successful_requests / total_time_seconds:.2f} requests/second")
    
    if response_times:
        avg_response_time = sum(response_times) / len(response_times)
        sorted_times = sorted(response_times)
        p50 = sorted_times[len(sorted_times) // 2]
        p95 = sorted_times[int(len(sorted_times) * 0.95)]
        p99 = sorted_times[int(len(sorted_times) * 0.99)]
        
        print(f"\nResponse times (ms):")
        print(f"  Average: {avg_response_time:.2f}")
        print(f"  Min: {min(response_times):.2f}")
        print(f"  Max: {max(response_times):.2f}")
        print(f"  P50 (median): {p50:.2f}")
        print(f"  P95: {p95:.2f}")
        print(f"  P99: {p99:.2f}")
    
    print(f"{'='*60}")
    
    requests_per_second = successful_requests / total_time_seconds
    meets_requirement = requests_per_second >= 1000
    
    print(f"\nSRD Requirement (1000 req/s): {'✓ PASS' if meets_requirement else '✗ FAIL'}")
    if not meets_requirement:
        print(f"  Current: {requests_per_second:.2f} req/s")
        print(f"  Target: 1000 req/s")
        print(f"  Gap: {1000 - requests_per_second:.2f} req/s")
    else:
        margin = ((requests_per_second - 1000) / 1000) * 100
        print(f"  Achieved: {requests_per_second:.2f} req/s")
        print(f"  Margin: +{margin:.1f}% above requirement")
    
    print(f"{'='*60}")
    print(f"\nGit: {git_info}")
    print(f"{'='*60}\n")


def main():
    scenarios = [
        (100, 10, "Light load - 100 requests, 10 concurrent"),
        (1000, 50, "Medium load - 1000 requests, 50 concurrent"),
        (1000, 100, "Heavy load - 1000 requests, 100 concurrent"),
        (2000, 200, "Stress test - 2000 requests, 200 concurrent"),
    ]
    
    print("\nLog Server Concurrent Benchmark")
    print("================================\n")
    print("This benchmark tests concurrent request handling")
    print("to validate the 1000 req/s SRD requirement.\n")
    
    for total_requests, concurrent_workers, description in scenarios:
        print(f"\nScenario: {description}")
        response = input("Run this scenario? (y/n/q to quit): ").strip().lower()
        
        if response == 'q':
            print("Benchmark cancelled.")
            break
        elif response == 'y':
            run_benchmark(total_requests, concurrent_workers)
            time.sleep(2)  # Brief pause between scenarios
        else:
            print("Skipped.\n")
    
    print("\nBenchmark complete!")


if __name__ == "__main__":
    main()
