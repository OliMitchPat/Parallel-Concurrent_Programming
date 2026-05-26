use crate::flocking_one::Bird;
use std::sync::Arc;
use std::thread;

pub fn update_flock_lock_free_chunks(original_birds: &Vec<Bird>, dt: f32, chunk_size: usize) -> Vec<Bird> {
    let shared_flock = Arc::new(original_birds.clone());
    let total_birds = shared_flock.len();
    let num_chunks = (total_birds + chunk_size - 1) / chunk_size;

    let mut handles = Vec::new();

    for chunk_index in 0..num_chunks {
        let shared_flock = Arc::clone(&shared_flock);
        let start = chunk_index * chunk_size;
        let end = ((chunk_index + 1) * chunk_size).min(total_birds);

        let chunk = shared_flock[start..end].to_vec();

        let handle = thread::spawn(move || {
            chunk
                .into_iter()
                .map(|mut bird| {
                    bird.update(&shared_flock, dt);
                    bird
                })
                .collect::<Vec<Bird>>()
        });

        handles.push(handle);
    }

    let mut updated_birds = Vec::with_capacity(total_birds);
    for handle in handles {
        let chunk = handle.join().unwrap();
        updated_birds.extend(chunk);
    }

    updated_birds
}
