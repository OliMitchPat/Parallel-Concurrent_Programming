use nalgebra::Vector3; // Needed for vector math

#[derive(Clone)]
pub struct Bird {
    pub position: Vector3<f32>,
    pub velocity: Vector3<f32>,
    pub acceleration: Vector3<f32>,
}


pub fn find_neighbors<'a>(bird: &Bird, flock: &'a [Bird], radius: f32) -> Vec<&'a Bird> {
    flock.iter()
        .filter(|other| {
            let dist_sq = (bird.position.x - other.position.x).powi(2) +
                          (bird.position.y - other.position.y).powi(2) +
                          (bird.position.z - other.position.z).powi(2);
            dist_sq > 0.0 && dist_sq < radius.powi(2)
        })
        .collect()
}

pub fn compute_separation(bird: &Bird, neighbors: &[&Bird]) -> Vector3<f32> {
    let mut force = Vector3::zeros();
    for neighbor in neighbors {
        force += bird.position - neighbor.position;
    }
    if !neighbors.is_empty() {
        force /= neighbors.len() as f32;
    }
    force
}

pub fn compute_alignment(bird: &Bird, neighbors: &[&Bird]) -> Vector3<f32> {
    let mut avg_velocity = Vector3::zeros();
    for neighbor in neighbors {
        avg_velocity += neighbor.velocity;
    }
    if !neighbors.is_empty() {
        avg_velocity /= neighbors.len() as f32;
    }
    avg_velocity
}

pub fn compute_cohesion(bird: &Bird, neighbors: &[&Bird]) -> Vector3<f32> {
    let mut center = Vector3::zeros();
    for neighbor in neighbors {
        center += neighbor.position;
    }
    if !neighbors.is_empty() {
        center /= neighbors.len() as f32;
    }

    center - bird.position
}

// Adds a force to keep birds within a given boundary (e.g., box or circle)
pub fn compute_boundary_force(bird: &Bird, boundary_radius: f32) -> Vector3<f32> {
    let mut force = Vector3::zeros();

    let distance_from_center = bird.position.magnitude();

    if distance_from_center > boundary_radius {
        force = -bird.position.normalize() * (distance_from_center - boundary_radius) * 2.0; // Strength multiplier
    } else if distance_from_center > boundary_radius * 0.75 {
        force = -bird.position.normalize() * (distance_from_center - boundary_radius * 0.75) * 0.5;
    }
    force
}



pub fn compute_flocking_forces(bird: &Bird, flock: &[Bird], ws: f32, wa: f32, wc: f32, boundary_radius: f32) -> Vector3<f32> {
    let neighbors: Vec<&Bird> = find_neighbors(bird, flock, 1.0);

    let separation = compute_separation(bird, &neighbors);
    let alignment = compute_alignment(bird, &neighbors);
    let cohesion = compute_cohesion(bird, &neighbors);

    let boundary = compute_boundary_force(bird, boundary_radius);

    ws * separation + wa * alignment + wc * cohesion + boundary
}

pub fn update_bird(bird: &mut Bird, dt: f32, max_speed: f32, flock: &[Bird], boundary_radius: f32) {
    let forces = compute_flocking_forces(bird, flock, 2.0, 1.0, 0.4, boundary_radius);
    bird.acceleration = forces;

    bird.velocity += bird.acceleration * dt;

    if bird.velocity.magnitude() > max_speed {
        bird.velocity = bird.velocity.normalize() * max_speed;
    }

    bird.position += bird.velocity * dt;
}

impl Bird {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Bird {
            position: Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)),
            velocity: Vector3::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1)),
            acceleration: Vector3::zeros(),
        }
    }

    pub fn update(&mut self, flock: &[Bird], dt: f32) {
        let max_speed = 1.0;
        let boundary_radius = 3.0; 
        update_bird(self, dt, max_speed, flock, boundary_radius);
    }
}