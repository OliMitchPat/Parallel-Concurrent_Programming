use crate::flocking_one::Bird;
use std::sync::Arc;
use std::thread;

pub fn update_flock_per_bird(original_birds: &Vec<Bird>, dt: f32) -> Vec<Bird> {
    let shared_flock = Arc::new(original_birds.clone());
    let mut handles = Vec::with_capacity(original_birds.len());

    for bird in original_birds.iter().cloned() {
        let shared_flock = Arc::clone(&shared_flock);
        let handle = thread::spawn(move || {
            let mut updated_bird = bird;
            updated_bird.update(&shared_flock, dt);
            updated_bird
        });

        handles.push(handle);
    }

    handles
        .into_iter()
        .map(|h| h.join().expect("Thread panicked"))
        .collect()
}
