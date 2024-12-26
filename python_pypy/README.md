# PyPy Async Tasks Benchmark

## Performance Characteristics

### Why PyPy?

- Just-In-Time (JIT) compilation
- Improved memory management
- Potential performance gains over CPython

### Benchmark Methodology

- Spawn 100,000 concurrent tasks
- Each task sleeps for 10 seconds
- Track memory and execution metrics

## Performance Recommendations

1. Use latest PyPy version
2. Run with `pypy3` interpreter
3. Consider `-OO` optimization flag

### Execution

```bash
pypy3 main.py [num_tasks]
