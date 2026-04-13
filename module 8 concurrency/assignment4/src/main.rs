use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items each producer will produce
    const ITEM_COUNT: usize = 10;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    // Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();

    // Share receiver among consumers
    let shared_rx = Arc::new(Mutex::new(rx));

    let mut producer_handles = Vec::new();
    let mut consumer_handles = Vec::new();

    // Create 2 producer threads
    for id in 1..=NUM_PRODUCERS {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT);
        });
        producer_handles.push(handle);
    }

    // Create 3 consumer threads
    for id in 1..=NUM_CONSUMERS {
        let rx_clone = Arc::clone(&shared_rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        consumer_handles.push(handle);
    }

    // Wait for all producers to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Send termination signal once for each consumer
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for all consumers to finish
    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let number = rng.gen_range(1..=100);
        println!("Producer {} produced {}", id, number);
        tx.send(number).unwrap();
        thread::sleep(Duration::from_millis(200));
    }

    println!("Producer {} finished", id);
}

// Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = {
            let receiver = rx.lock().unwrap();
            receiver.recv().unwrap()
        };

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        }

        println!("Consumer {} consumed {}", id, value);
        thread::sleep(Duration::from_millis(300));
    }

    println!("Consumer {} exiting", id);
}