use rand::Rng;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
enum TaskKind {
    CPU,
    IO,
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    arrival_time: u64,
    kind: TaskKind,
    duration: u64,
    created_at: Instant,
}

#[derive(Debug)]
struct Metrics {
    total_completed: usize,
    cpu_completed: usize,
    io_completed: usize,
    total_wait_time: u128,
    total_turnaround_time: u128,
}

fn generate_tasks(num_tasks: usize) -> Vec<Task> {
    let mut rng = rand::thread_rng();
    let mut tasks = Vec::new();

    for i in 0..num_tasks {
        let kind = if rng.gen_bool(0.5) {
            TaskKind::CPU
        } else {
            TaskKind::IO
        };

        tasks.push(Task {
            id: i,
            arrival_time: rng.gen_range(0..100),
            kind,
            duration: rng.gen_range(10..50),
            created_at: Instant::now(),
        });
    }

    tasks
}

fn main() {
    let start_time = Instant::now();
    let tasks = generate_tasks(500);


    let args: Vec<String> = std::env::args().collect();
    let use_optimized = args.contains(&"--optimized".to_string());

    println!("Generated {} tasks", tasks.len());

    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let metrics = Arc::new(Mutex::new(Metrics {
        total_completed: 0,
        cpu_completed: 0,
        io_completed: 0,
        total_wait_time: 0,
        total_turnaround_time: 0,
    }));

    {
        let mut q = queue.lock().unwrap();
        for task in tasks {
            q.push_back(task);
        }
    }

    let mut handles = vec![];

    for worker_id in 0..4 {
        let queue_clone = Arc::clone(&queue);
        let metrics_clone = Arc::clone(&metrics);

        let handle = thread::spawn(move || loop {
            let task_opt = {
                let mut q = queue_clone.lock().unwrap();

                if use_optimized {
                    if q.is_empty() {
                        None
                    } else {
                        let mut min_index = 0;
                        for i in 1..q.len() {
                            if q[i].duration < q[min_index].duration {
                                min_index = i;
                            }
                        }
                        Some(q.remove(min_index).unwrap())
                    }
                } else {
                    q.pop_front()
                }
            };

            match task_opt {
                Some(task) => {
                    let start_processing = Instant::now();
                    let wait_time = start_processing.duration_since(task.created_at).as_millis();

                    println!("Worker {} processing task {}", worker_id, task.id);

                    thread::sleep(Duration::from_millis(task.duration));

                    let finish_time = Instant::now();
                    let turnaround_time = finish_time.duration_since(task.created_at).as_millis();

                    let mut m = metrics_clone.lock().unwrap();
                    m.total_completed += 1;
                    m.total_wait_time += wait_time;
                    m.total_turnaround_time += turnaround_time;

                    match task.kind {
                        TaskKind::CPU => m.cpu_completed += 1,
                        TaskKind::IO => m.io_completed += 1,
                    }
                }
                None => {
                    println!("Worker {} done", worker_id);
                    break;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total_runtime = start_time.elapsed().as_millis();
    let m = metrics.lock().unwrap();

    println!("\n========== METRICS ==========");
    println!(
        "Policy: {}",
        if use_optimized {
            "Optimized Shortest Job First"
        } else {
            "FIFO"
        }
    );
    println!("Total tasks completed: {}", m.total_completed);
    println!("CPU tasks completed: {}", m.cpu_completed);
    println!("IO tasks completed: {}", m.io_completed);
    println!("Makespan / total runtime: {} ms", total_runtime);

    if m.total_completed > 0 {
        println!(
            "Average wait time: {:.2} ms",
            m.total_wait_time as f64 / m.total_completed as f64
        );

        println!(
            "Average turnaround time: {:.2} ms",
            m.total_turnaround_time as f64 / m.total_completed as f64
        );
    }

    println!("=============================");
    println!("All tasks completed!");
}