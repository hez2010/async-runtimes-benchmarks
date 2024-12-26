const { Worker, isMainThread, parentPort, workerData } = require('worker_threads');
const v8 = require('v8');

// Add explicit Promise-based task creation
// Implement a proper task function
// Add error handling
// Use Promise.allSettled() for comprehensive tracking

// More improvements:
// Add V8 memory tracking
// Implement more detailed performance metrics
// Use worker_threads for better comparison

function getMemoryUsage() {
    const memoryUsage = process.memoryUsage();
    const v8HeapStats = v8.getHeapStatistics();
    return {
        rss: memoryUsage.rss,
        heapTotal: memoryUsage.heapTotal,
        heapUsed: memoryUsage.heapUsed,
        external: memoryUsage.external,
        v8Total: v8HeapStats.total_heap_size,
        v8Used: v8HeapStats.used_heap_size
    };
}

async function performTask(taskId) {
    const startMemory = getMemoryUsage();
    const startTime = Date.now();

    await new Promise(resolve => setTimeout(resolve, 10000));

    const endTime = Date.now();
    const endMemory = getMemoryUsage();

    return {
        taskId,
        duration: (endTime - startTime) / 1000,
        memoryChange: {
            rss: endMemory.rss - startMemory.rss,
            heapUsed: endMemory.heapUsed - startMemory.heapUsed
        }
    };
}

async function runTasks(numTasks) {
    const tasks = [];

    for (let i = 0; i < numTasks; i++) {
        tasks.push(performTask(i));
    }

    try {
        const results = await Promise.allSettled(tasks);
        
        // Optional: Error handling and logging
        results.forEach((result, index) => {
            if (result.status === 'rejected') {
                console.error(`Task ${index} failed:`, result.reason);
            }
        });
    } catch (error) {
        console.error('Unexpected error in task execution:', error);
    }
}

const numTasks = parseInt(process.argv[2] || 100000);
runTasks(numTasks);