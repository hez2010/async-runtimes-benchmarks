const util = require('util');
const delay = util.promisify(setTimeout);

// Add explicit Promise-based task creation
// Implement a proper task function
// Add error handling
// Use Promise.allSettled() for comprehensive tracking

async function performTask(taskId) {
    const startTime = Date.now();
    await delay(10000);
    const endTime = Date.now();
    return {
        taskId,
        duration: (endTime - startTime) / 1000
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