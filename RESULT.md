# How Much Memory Do You Need in 2024 to Run 1 Million Concurrent Tasks? - Take 2

Did you still remember [the memory consumption comparison](https://pkolaczk.github.io/memory-consumption-of-async/) between asynchronous programming across popular languages in 2023?

Now at the end of 2024, I wonder how things changed in the span of one year, with the latest version of languages.

So I did the benchmark again in [How Much Memory Do You Need in 2024 to Run 1 Million Concurrent Tasks?](https://hez2010.github.io/async-runtimes-benchmarks-2024)

Then some folks pointed out that the code for some languages were non-optimal, so after taking changes from the community, I ran the benchmark again.

Now let's see the result.

## Benchmark

The program to benchmark is the same with the one in the last year:

> Let's launch N concurrent tasks, where each task waits for 10 seconds and then the program exists after all tasks finish. The number of tasks is controlled by the command line argument.

This time, let's focus on coroutine instead of multiple threads.

All benchmark code can be accessed at [async-runtimes-benchmarks-2024](https://github.com/hez2010/async-runtimes-benchmarks-2024).

What is a coroutine?

> Coroutines are computer program components that allow execution to be suspended and resumed, generalizing subroutines for cooperative multitasking. Coroutines are well-suited for implementing familiar program components such as cooperative tasks, exceptions, event loops, iterators, infinite lists and pipes.

### Rust

I created 3 programs in Rust. One uses `tokio`:

```rust
use std::env;
use tokio::{spawn, time::{sleep, Duration}};

#[tokio::main]
async fn main() {
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();

    let mut tasks = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        tasks.push(spawn(sleep(Duration::from_secs(10))));
    }
    for task in tasks {
        task.await.unwrap();
    }
}
```

One uses `async_std`:

```rust
use std::env;
use async_std::task;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();

    let mut tasks = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        tasks.push(task::spawn(task::sleep(Duration::from_secs(10))));
    }

    for task in tasks {
        task.await;
    }
}
```

And one uses `tokio` but uses `futures::future::join_all` to track all tasks instead of `spawn` each task separately:

```rust
use std::env;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let num_tasks = env::args().skip(1).next().unwrap().parse().unwrap();

    futures::future::join_all((0..num_tasks).map(|_| tokio::time::sleep(Duration::from_secs(10))))
        .await;
}
```

Both `tokio` and `async_std` are popular async runtime commonly used in Rust.

### C#

C#, similar to Rust, has first-class support for async/await:

```csharp
int numTasks = int.Parse(args[0]);

List<Task> tasks = new List<Task>(numTasks);

for (int i = 0; i < numTasks; i++)
{
    tasks.Add(Task.Delay(TimeSpan.FromSeconds(10)));
}

await Task.WhenAll(tasks);
```

.NET also offers NativeAOT compilation since .NET 7, which compiles the code to the final binary directly so that it no longer needs a VM to run managed code. So we added the benchmark for NativeAOT as well.

### NodeJS

So does NodeJS:

```javascript
const util = require('util');
const delay = util.promisify(setTimeout);

async function runTasks(numTasks) {
  const tasks = [];

  for (let i = 0; i < numTasks; i++) {
    tasks.push(delay(10000));
  }

  await Promise.all(tasks);
}

const numTasks = parseInt(process.argv[2]);
runTasks(numTasks);
```

### Python

And Python, too:

```python
import asyncio
import sys

async def main(num_tasks):
    tasks = []

    for task_id in range(num_tasks):
        tasks.append(asyncio.sleep(10))

    await asyncio.gather(*tasks)

if __name__ == "__main__":
    num_tasks = int(sys.argv[1])
    asyncio.run(main(num_tasks))
```


### Go

In Go, goroutines are the building block for concurrency. We don’t await them separately, but we use a `WaitGroup` instead:

```go
package main

import (
	"fmt"
	"os"
	"strconv"
	"sync"
	"time"
)

func main() {
	numRoutines := 100000
	if len(os.Args) > 1 {
		n, err := strconv.Atoi(os.Args[1])
		if err == nil {
			numRoutines = n
		}
	}

	var wg sync.WaitGroup
	for i := 0; i < numRoutines; i++ {
		wg.Add(1)
		go func() {
			time.Sleep(10 * time.Second)
			wg.Done()
		}()
	}
	wg.Wait()
}
```

### Java

Java offers virtual threads since JDK 21, which are a similar concept to goroutines:

```java
import java.time.Duration;
import java.util.ArrayList;
import java.util.List;

public class VirtualThreads {

    public static void main(String[] args) throws InterruptedException {
	    int numTasks = Integer.parseInt(args[0]);
        List<Thread> threads = new ArrayList<>(numTasks);

        for (int i = 0; i < numTasks; i++) {
            Thread thread = Thread.startVirtualThread(() -> {
                try {
                    Thread.sleep(Duration.ofSeconds(10));
                } catch (InterruptedException e) {
                    // Handle exception
                }
            });
            threads.add(thread);
        }

        for (Thread thread : threads) {
            thread.join();
        }
    }
}
```

While there's a new variant of JVM called GraalVM. GraalVM also offers native image, which is a similar concept to NativeAOT in .NET. So we added the benchmark for GraalVM as well.

## Test Environment

- Hardware: 13th Gen Intel(R) Core(TM) i7-13700K
- OS: Debian GNU/Linux 12 (bookworm)
- Rust: 1.82.0
- .NET: 9.0.100
- Go: 1.23.3
- Java: openjdk 23.0.1 build 23.0.1+11-39
- Java (GraalVM): java 23.0.1 build 23.0.1+11-jvmci-b01
- NodeJS: v23.2.0
- Python: 3.13.0

All programs were launched using the release mode if available, and support for internationalization and globalization was disabled as we did't have libicu in our test environment.

## Results

<script src="https://cdn.jsdelivr.net/npm/chart.js">

</script>

### Minimum Footprint

Let's start from something small, because some runtimes require some memory for themselves, let's first launch only one task.

<div style="height:40vh; width:80vw">
  <canvas id="cvs-0">
  </canvas>
</div>
<script>
    const ctx0 = document.getElementById('cvs-0');
    new Chart(ctx0, {
        type: 'bar',
        data: {
            labels: ['Rust (tokio)', 'Rust (async_std)', 'Rust (futures)', 'C#', 'C# (NativeAOT)', 'Go', 'Java (OpenJDK)', 'Java (GraalVM)', 'Java (GraalVM native-image)', 'NodeJS', 'Python'],
            datasets: [
                { label: 'Memory (MB)', data: [4.71875, 5.1484375, 3.03515625, 24.359375, 3.58984375, 3.5546875, 46.91015625, 112.421875, 8.265625, 41.9296875, 19.515625] },
                { label: 'CPU (%)', data: [0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0] },
                { label: 'Time (Sec)', data: [10, 10, 10, 10.01, 10, 10, 10.03, 10.11, 10, 10.05, 10.09] },
            ]
        },
        options: { indexAxis: 'y' }
    });
</script>

**Note: You can click the legend label on the top to hide a specific legend.**

We can see that Rust, C# (NativeAOT), and Go achieved similar results, as they were compiled statically to native binaries and needed very little memory. Java (GraalVM native-image) also did a great job but cost a bit more than the other statically compiled ones. The other programs running on managed platforms or through interpreters consume more memory.

Rust (futures) seems to have the smallest footprint in this case. While Go and C# (NativeAOT) seem to have the similar minimal footprint.

Python, which is running on an interpreter, also shows great result.

Java with GraalVM is a bit surprising, as it cost far more memory than Java with OpenJDK, but I guess this can be tuned with some settings.

### 10K Tasks

<div style="height:40vh; width:80vw">
  <canvas id="cvs-1">
  </canvas>
</div>
<script>
    const ctx1 = document.getElementById('cvs-1');
    new Chart(ctx1, {
        type: 'bar',
        data: {
            labels: ['Rust (tokio)', 'Rust (async_std)', 'Rust (futures)', 'C#', 'C# (NativeAOT)', 'Go', 'Java (OpenJDK)', 'Java (GraalVM)', 'Java (GraalVM native-image)', 'NodeJS', 'Python'],
            datasets: [
                { label: 'Memory (MB)', data: [9.03515625, 8.1015625, 4.78125, 28.1171875, 7.55859375, 34.29296875, 108.37109375, 181.8359375, 22.828125, 64.7890625, 33.34765625] },
                { label: 'CPU (%)', data: [0, 0, 0, 0, 0, 0, 11, 6, 1, 0, 1] },
                { label: 'Time (Sec)', data: [10, 10, 10, 10.02, 10, 10.01, 10.1, 10.08, 10.03, 10.02, 10.1] },
            ]
        },
        options: { indexAxis: 'y' }
    });
</script>

A few surprises here! The three Rust benchmarks, C# (NativeAOT) achieved very promising results: they both used very little memory (less than 10MB), which didn't grow too much compared to minimal footprint results, even though there were 10K tasks running behind the scenes! C# (NativeAOT) followed closely behind, using only ~10MB of memory. We need more tasks to put more pressure on them!

The memory consumption grew dramatically in Go. Goroutines are supposed to be very lightweight, but they actually consumed far more RAM than Rust required. In this case, virtual threads in Java (GraalVM native image) seem to be more lightweight than Goroutines in Go. To my surprise, both Go and Java (GraalVM native image), which were compiled to native binaries statically, cost similar RAM with the C# one running on a VM!

### 100K Tasks

<div style="height:40vh; width:80vw">
  <canvas id="cvs-2">
  </canvas>
</div>
<script>
    const ctx2 = document.getElementById('cvs-2');
    new Chart(ctx2, {
        type: 'bar',
        data: {
            labels: ['Rust (tokio)', 'Rust (async_std)', 'Rust (futures)', 'C#', 'C# (NativeAOT)', 'Go', 'Java (OpenJDK)', 'Java (GraalVM)', 'Java (GraalVM native-image)', 'NodeJS', 'Python'],
            datasets: [
                { label: 'Memory (MB)', data: [46.234375, 53.24609375, 22.7578125, 50.05078125, 30.02734375, 262.67578125, 196.96875, 419.01171875, 100.83984375, 125.06640625, 146.46875] },
                { label: 'CPU (%)', data: [1, 1, 0, 2, 2, 5, 70, 54, 14, 1, 9] },
                { label: 'Time (Sec)', data: [10.04, 10.03, 10.02, 10.07, 10, 10.09, 10.61, 10.25, 10.32, 10.06, 10.71] },
            ]
        },
        options: { indexAxis: 'y' }
    });
</script>

After we increased the number of tasks to 100K, the memory consumption of all the languages started to grow significantly.

Both Rust and C# did a really good job in this case. Rust continues to lead the benchmark, and C# follows closely. Really impressive!

At this point, the Go program has been beaten not only by Rust but also by Java (except the one running on GraalVM), C#, and NodeJS. But worth to note that Java costs significantly more CPU to complete the benchmark.

### 1 Million Tasks

Let's go extreme now.

<div style="height:40vh; width:80vw">
  <canvas id="cvs-3">
  </canvas>
</div>
<script>
    const ctx3 = document.getElementById('cvs-3');
    new Chart(ctx3, {
        type: 'bar',
        data: {
            labels: ['Rust (tokio)', 'Rust (async_std)', 'Rust (futures)', 'C#', 'C# (NativeAOT)', 'Go', 'Java (OpenJDK)', 'Java (GraalVM)', 'Java (GraalVM native-image)', 'NodeJS', 'Python'],
            datasets: [
                { label: 'Memory (MB)', data: [439.08984375, 502.25, 207.37890625, 218.38671875, 196.59375, 2585.5859375, 1095.1875, 1546.76171875, 1057.32421875, 563.88671875, 1275.71875] },
                { label: 'CPU (%)', data: [17, 19, 7, 21, 27, 28, 452, 475, 117, 13, 67] },
                { label: 'Time (Sec)', data: [10.41, 10.35, 10.3, 10.5, 10.58, 10.83, 15.98, 15.6, 13.41, 10.51, 19.36] },
            ]
        },
        options: { indexAxis: 'y' }
    });
</script>

Finally, Rust (futures) and C# show very promising result; either is very competitive and has really become a monster.

And it's worth to note that Rust consistently cost the least CPU for running all the tasks.

NodeJS, which runs on a VM, also shows great result in this case: although it costs more RAM than C#, it requires less CPU to complete the benchmark, which is even less than some of the Rust benchmarks.

While both Java and Python start to be not able to complete the benchmark in 10 seconds, and Java costs significant more CPU than other languages.

## Final Word

As we have observed, a high number of concurrent tasks can consume a significant amount of memory, even if they do not perform complex operations. Different language runtimes have varying trade-offs, with some being lightweight and efficient for a small number of tasks but scaling poorly with hundreds of thousands of tasks.

Many things have changed since last year. With the benchmark results on the latest compilers and runtimes, we see a huge improvement in .NET, and .NET with NativeAOT is really competitive.

Rust continues to be memory saving as expected, and achieved similar result with C# (NativeAOT).

NodeJS shows impressive result in term of CPU usage.

Python faced performance issue and was not able to complete the benchmark in time in the 1M case.

Both Java Virtual Thread and Goroutine take similar approach on concurrency, while others are using async/await, so let's exclude other languages and only focus on these two: the native image of Java built with GraalVM did a great job in terms of memory efficiency, but it failed to finished the benchmark in 10 seconds in the 1M case; while Goroutine is able to complete all the tasks in time, but it costs much more RAM than Java in the 1M case. 
