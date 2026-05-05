# Concurrent Task Dispatcher (Rust) Final Project

## Overview
This project simulates a concurrent task dispatcher system similar to an operating system scheduler.

Tasks are generated, placed into a shared queue, and processed by worker threads using different scheduling policies.

---

## How to Run

### FIFO (default)
cargo run

### Optimized (Shortest Job First)
cargo run -- --optimized

---

## Features
- 500 tasks generated randomly
- CPU and IO task types
- 4 worker threads
- Shared queue using Arc<Mutex>
- FIFO and optimized scheduling
- Metrics collection

---

## Metrics
- Total tasks completed
- CPU tasks completed
- IO tasks completed
- Makespan (total runtime)
- Average wait time
- Average turnaround time

---

## Experiments
Results saved in:
- fifo.txt
- optimized.txt

FIFO is fair but slower.  
Optimized is faster but less fair.

---

## Tool Use Disclosure

Tools used:
- ChatGPT (for guidance, debugging, and structuring the project)
- Rust documentation

Advice accepted:
- Using Arc<Mutex<VecDeque>> for safely sharing the queue across threads

Advice rejected or fixed:
- Initially placed scheduling logic outside the worker threads, which caused errors and incorrect behavior. This was corrected by moving the logic inside the worker loop.

---

## Author
Eli Cortez