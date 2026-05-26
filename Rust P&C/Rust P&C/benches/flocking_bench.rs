use criterion::{criterion_group, criterion_main, Criterion};
use opengl_rust_glium::{update_flock_in_threads, update_flock_per_bird, update_flock_lock_free_chunks, Bird};

fn benchmark_chunk_thread(c: &mut Criterion) {
    let bird_counts = vec![500, 1000, 5000, 10000];
    for &count in &bird_counts {
        c.bench_function(&format!("chunk_thread_{}", count), |b| {
            let flock: Vec<Bird> = (0..count).map(|_| Bird::new()).collect();
            b.iter(|| {
                let mut flock_clone = flock.clone();
                update_flock_in_threads(&mut flock_clone, 0.016, 4);
            });
        });
    }
}

fn benchmark_thread_per_bird(c: &mut Criterion) {
    let bird_counts = vec![500, 1000, 5000, 10000];
    for &count in &bird_counts {
        c.bench_function(&format!("thread_per_bird_{}", count), |b| {
            let flock: Vec<Bird> = (0..count).map(|_| Bird::new()).collect();
            b.iter(|| {
                let flock_clone = flock.clone();
                update_flock_per_bird(&flock_clone, 0.016);
            });
        });
    }
}

fn benchmark_lock_free(c: &mut Criterion) {
    let bird_counts = vec![500, 1000, 5000, 10000];
    for &count in &bird_counts {
        c.bench_function(&format!("lock_free_{}", count), |b| {
            let flock: Vec<Bird> = (0..count).map(|_| Bird::new()).collect();
            b.iter(|| {
                let flock_clone = flock.clone();
                update_flock_lock_free_chunks(&flock_clone, 0.016, 4);
            });
        });
    }
}

criterion_group!(benches, benchmark_chunk_thread, benchmark_thread_per_bird, benchmark_lock_free);
criterion_main!(benches);