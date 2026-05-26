#[macro_use]
extern crate glium;
extern crate winit;
use nalgebra::{Matrix4, Perspective3, Point3, Vector3};
mod flocking_one;
use flocking_one::{Bird};
use rand::Rng;
mod chunk_thread; 
use chunk_thread::update_flock_in_threads; 
mod thread_per_bird; 
use thread_per_bird::update_flock_per_bird; 
mod chunk_thread_lock_free;
use chunk_thread_lock_free::update_flock_lock_free_chunks;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Flocking")
        .build(&event_loop);

    // Define vertex structure for rendering triangles (birds)
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],  
    }
    implement_vertex!(Vertex, position);

    let mut rng = rand::thread_rng();
    let mut flock: Vec<Bird> = (0..10)
        .map(|_| Bird::new())
        .collect();
    
    // Define triangle vertices for the birds
    let vertex1 = Vertex { position: [-0.05, -0.0288, 0.0] };
    let vertex2 = Vertex { position: [ 0.00,  0.0577, 0.0] };
    let vertex3 = Vertex { position: [ 0.05, -0.0288, 0.0] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main() {
            gl_Position = projection * view * model * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut delta_t: f32 = -0.016;

    #[allow(deprecated)] 
    use std::time::Instant;

let mut previous_time = Instant::now(); // Initialize the time tracking

let _ = event_loop.run(move |event, window_target| {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => window_target.exit(),

            winit::event::WindowEvent::Resized(window_size) => {
                display.resize(window_size.into());
            },

            winit::event::WindowEvent::RedrawRequested => {
                let current_time = Instant::now(); // Get current time
                let delta_t = current_time.duration_since(previous_time).as_secs_f32(); // Calculate delta time
                
                previous_time = current_time; // Update previous time for the next frame

                let next_frame_time = current_time + std::time::Duration::from_nanos(16_666_667);
                winit::event_loop::ControlFlow::WaitUntil(next_frame_time);

                // Update the birds' positions based on flocking behavior
                //update_flock_in_threads(&mut flock, delta_t, 4);
                //flock = update_flock_per_bird(&flock, delta_t);
                 flock = update_flock_lock_free_chunks(&flock, delta_t, 4);

                let mut target = display.draw();

                // Clear the screen to black
                target.clear_color(0.0, 0.0, 0.0, 1.0);

                // Perspective projection matrix
                let perspective = Perspective3::new(1.0, std::f32::consts::FRAC_PI_3, 0.1, 100.0);
                let projection_matrix: [[f32; 4]; 4] = *perspective.as_matrix().as_ref();

                // View matrix (camera positioned at (0, 0, 5) looking at the origin)
                let eye = Point3::new(0.0, 0.0, 5.0);
                let look = Point3::new(0.0, 0.0, 0.0);
                let up = Vector3::new(0.0, 1.0, 0.0);
                let view_matrix: [[f32; 4]; 4] = *Matrix4::look_at_rh(&eye, &look, &up).as_ref();

                // Iterate over the flock and draw each bird
                for bird in &flock {
                    let pos_x = bird.position[0];
                    let pos_y = bird.position[1];
                    let pos_z = bird.position[2];
                    
                    // Model matrix (position of the bird)
                    let model_matrix = [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [pos_x, pos_y, pos_z, 1.0],
                    ];

                    // Combine uniforms
                    let uniforms = uniform! {
                        model: model_matrix,
                        view: view_matrix,
                        projection: projection_matrix,
                    };

                    // Draw the bird (triangle)
                    target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                }

                // Display the completed drawing
                target.finish().unwrap();
            },
            _ => (),
        },
        winit::event::Event::AboutToWait => {
            window.request_redraw();
        },
        _ => (),
    };
});
}
