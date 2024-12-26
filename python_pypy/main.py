#!/usr/bin/env python3
"""
PyPy Async Tasks Benchmark

This script measures concurrent task performance using PyPy's enhanced runtime.
Focuses on demonstrating PyPy's JIT compilation and concurrency capabilities.

Key Improvements:
- Leverages PyPy's JIT compiler for potential performance gains
- Detailed memory and performance tracking
- Comprehensive error handling
"""

import asyncio
import sys
import time
import logging
import tracemalloc  # More advanced memory tracking for PyPy

# Configure advanced logging
logging.basicConfig(
    level=logging.INFO, 
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(),  # Console output
        logging.FileHandler('pypy_benchmark.log')  # Log to file
    ]
)
logger = logging.getLogger(__name__)

class TaskMetrics:
    """
    Comprehensive task performance metrics tracking
    
    Attributes:
        task_id (int): Unique task identifier
        start_time (float): Task start timestamp
        end_time (float): Task end timestamp
        memory_start (int): Memory usage at task start
        memory_end (int): Memory usage at task end
    """
    def __init__(self, task_id):
        self.task_id = task_id
        self.start_time = time.time()
        self.end_time = None
        self.memory_start = tracemalloc.get_traced_memory()[0]
        self.memory_end = None

    def complete(self):
        """Finalize task metrics"""
        self.end_time = time.time()
        self.memory_end = tracemalloc.get_traced_memory()[0]

    def report(self):
        """Generate detailed task performance report"""
        return {
            'task_id': self.task_id,
            'duration': self.end_time - self.start_time,
            'memory_delta': self.memory_end - self.memory_start
        }

async def perform_task(task_id):
    """
    Simulate a concurrent task with comprehensive tracking
    
    Args:
        task_id (int): Unique task identifier
    
    Returns:
        dict: Detailed task performance metrics
    """
    # Initialize task metrics
    metrics = TaskMetrics(task_id)
    
    try:
        # Simulate 10-second task with asyncio
        await asyncio.sleep(10)
        
        # Complete task metrics
        metrics.complete()
        
        # Log task completion
        logger.info(f"Task {task_id} completed successfully")
        
        return metrics.report()
    
    except Exception as e:
        # Comprehensive error handling
        logger.error(f"Task {task_id} failed: {e}")
        logger.error(traceback.format_exc())
        return None

async def main(num_tasks):
    """
    Create and manage concurrent tasks
    
    Args:
        num_tasks (int): Number of concurrent tasks to create
    """
    # Enable memory tracing
    tracemalloc.start()
    
    try:
        # Create tasks with asyncio
        tasks = [perform_task(i) for i in range(num_tasks)]
        
        # Wait for all tasks to complete
        results = await asyncio.gather(*tasks, return_exceptions=True)
        
        # Process and log results
        successful_tasks = [r for r in results if r is not None]
        logger.info(f"Completed {len(successful_tasks)}/{num_tasks} tasks")
    
    finally:
        # Stop memory tracing
        tracemalloc.stop()

def cli():
    """Command-line interface for benchmark"""
    # Parse task count from command line or use default
    num_tasks = int(sys.argv[1]) if len(sys.argv) > 1 else 100_000
    
    # Run benchmark
    asyncio.run(main(num_tasks))

if __name__ == "__main__":
    cli()