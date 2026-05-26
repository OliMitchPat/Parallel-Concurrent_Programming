use crate::flocking_one::Bird;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn update_flock_in_threads(flock: &mut Vec<Bird>, dt: f32, chunk_size: usize) {
    let shared_flock = Arc::new(flock.clone());
    let birds_arc = Arc::new(Mutex::new(flock.clone()));

    let total_birds = shared_flock.len();
    let num_chunks = (total_birds + chunk_size - 1) / chunk_size;

    let mut handles = vec![];

    for chunk_index in 0..num_chunks {
        let birds_arc = Arc::clone(&birds_arc);
        let shared_flock = Arc::clone(&shared_flock);

        let handle = thread::spawn(move || {
            let mut birds_lock = birds_arc.lock().unwrap();
            let start = chunk_index * chunk_size;
            let end = ((chunk_index + 1) * chunk_size).min(birds_lock.len());

            for bird in &mut birds_lock[start..end] {
                bird.update(&shared_flock, dt);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *flock = birds_arc.lock().unwrap().clone();
}


