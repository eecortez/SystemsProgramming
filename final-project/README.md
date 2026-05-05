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
- ChatGPT
- Rust docs

Accepted advice:
- Using Arc<Mutex> for shared queue

Rejected/fixed:
- Initially placed scheduling logic outside threads, causing errors

---

## Author
Eli Cortez