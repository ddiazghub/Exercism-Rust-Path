use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::thread::{JoinHandle, Thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 {
        return HashMap::new();
    }

    let workload_size = (input.len() as f64 / worker_count as f64).ceil() as usize;
    let mut workers: Vec<JoinHandle<HashMap<char, usize>>> = Vec::new();
    let chunks = input.chunks(workload_size);

    for chunk in chunks {
        let chunk: Vec<String> = chunk
            .into_iter()
            .map(|word| String::from(*word))
            .collect();

        let thread_i = thread::spawn(move || {
            let mut freq: HashMap<char, usize> = HashMap::new();

            for word in chunk {
                for ch in word.to_lowercase().chars() {
                    if ch.is_alphabetic() {
                        match freq.get_mut(&ch) {
                            Some(f) => *f += 1,
                            None => drop(freq.insert(ch, 1))
                        };
                    }
                }
            }

            freq
        });

        workers.push(thread_i);
    }

    let mut freq = HashMap::new();

    for worker in workers {
        let mut map = worker.join().unwrap();

        for (&k, &v) in map.iter() {
            match freq.get_mut(&k) {
                Some(f) => *f += v,
                None => drop(freq.insert(k, v))
            };
        }
    }

    freq
}
