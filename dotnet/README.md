# .NET Async Tasks Benchmark

## Performance Characteristics

### Why .NET?

- High-performance async/await model
- Cross-platform runtime
- Advanced memory management
- Support for multiple concurrency models

### Benchmark Methodology

- Spawn 100,000 concurrent tasks
- Each task sleeps for 10 seconds
- Track memory and execution metrics

## Performance Recommendations

1. Use .NET 8.0+ runtime
2. Enable tiered compilation
3. Use Release configuration
4. Consider native AOT compilation

### Execution

```bash
dotnet run -c Release [num_tasks]
```