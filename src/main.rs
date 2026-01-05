use std::time::Instant;
use wgpu::util::DeviceExt;
use rand::prelude::*;
use image::RgbImage;
use std::io::Write;

// --- CONFIGURATION ---
const NUM_STARS: u32 = 100_000; 
const WORKGROUP_SIZE: u32 = 256;
const FRAMES_PER_MODE: usize = 150; // 150 frames for each mode
const SIM_STEPS_PER_FRAME: usize = 5; 

// DATA TYPES
const TYPE_A: f32 = 0.0; 
const TYPE_B: f32 = 1.0; 

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Star {
    x: f32, y: f32, z: f32,
    vx: f32, vy: f32, vz: f32,
    mass: f32, 
    data_type: f32, 
    time_debt: f32, 
    active_flag: f32, // Replaces 'padding' to visualize work
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct GalaxyState {
    time_seed: f32, 
    dilation_mode: f32, // 0.0 = Newton, 1.0 = Chronoturin
    padding2: f32, padding3: f32,
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    println!("--- CHRONOTURIN: COMPARATIVE VISUALIZER ---");
    
    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.expect("No GPU");
    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor::default(),
        None, 
    ).await.expect("No Device");

    let mut rng = rand::thread_rng();
    
    // --- RUN TWO PASSES (Newtonian vs Chronoturin) ---
    for pass in 0..2 {
        let is_chronoturin = pass == 1;
        let mode_name = if is_chronoturin { "CHRONOTURIN" } else { "NEWTONIAN" };
        let file_prefix = if is_chronoturin { "chrono" } else { "newton" };
        let dilation_val = if is_chronoturin { 1.0 } else { 0.0 };

        println!("\n>> STARTING PASS {}: {} MODE", pass + 1, mode_name);

        // 1. RESET DATA (Identical Start for Fairness)
        let mut initial_data = Vec::with_capacity(NUM_STARS as usize);
        // We use a deterministic seed-like generation by re-creating RNG if we wanted exactness,
        // but random chaos is fine as long as density is similar.
        for _ in 0..NUM_STARS {
            let r = 300.0 * rng.gen::<f32>().sqrt();
            let theta = rng.gen_range(0.0..std::f32::consts::TAU);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);
            let x = r * phi.sin() * theta.cos();
            let y = r * phi.sin() * theta.sin();
            let z = r * phi.cos();
            let data_type = if rng.gen_bool(0.5) { TYPE_A } else { TYPE_B };

            initial_data.push(Star { 
                x, y, z, vx: 0.0, vy: 0.0, vz: 0.0, mass: 1.0, 
                data_type, time_debt: 0.0, active_flag: 0.0 
            });
        }

        let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Star Storage"),
            contents: bytemuck::cast_slice(&initial_data),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        });

        let galaxy_state = GalaxyState { time_seed: 0.0, dilation_mode: dilation_val, padding2: 0.0, padding3: 0.0 };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Galaxy State"),
            contents: bytemuck::cast_slice(&[galaxy_state]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback Buffer"),
            size: (initial_data.len() * std::mem::size_of::<Star>()) as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None, layout: None, module: &shader, entry_point: "main",
        });
        
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None, layout: &bind_group_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: storage_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: uniform_buffer.as_entire_binding() },
            ],
        });

        // RENDER LOOP
        let fov = 800.0;
        let camera_z = -1000.0;

        for frame in 0..FRAMES_PER_MODE {
            let start_time = Instant::now();

            for _ in 0..SIM_STEPS_PER_FRAME {
                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
                    cpass.set_pipeline(&compute_pipeline);
                    cpass.set_bind_group(0, &bind_group, &[]);
                    cpass.dispatch_workgroups((NUM_STARS + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE, 1, 1);
                }
                queue.submit(Some(encoder.finish()));
            }

            // Readback
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            encoder.copy_buffer_to_buffer(&storage_buffer, 0, &readback_buffer, 0, (initial_data.len() * std::mem::size_of::<Star>()) as u64);
            queue.submit(Some(encoder.finish()));

            let buffer_slice = readback_buffer.slice(..);
            let (tx, rx) = std::sync::mpsc::channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
            
            device.poll(wgpu::Maintain::Wait);
            
            if let Ok(Ok(())) = rx.recv() {
                let data = buffer_slice.get_mapped_range();
                let stars: &[Star] = bytemuck::cast_slice(&data);
                let mut img = RgbImage::new(1024, 1024);

                for star in stars {
                    let rel_z = star.z - camera_z;
                    if rel_z > 10.0 {
                        let factor = fov / rel_z;
                        let screen_x = star.x * factor + 512.0;
                        let screen_y = star.y * factor + 512.0;

                        if screen_x >= 0.0 && screen_x < 1024.0 && screen_y >= 0.0 && screen_y < 1024.0 {
                            let pixel = img.get_pixel_mut(screen_x as u32, screen_y as u32);
                            
                            // BASE COLORS
                            if star.data_type < 0.5 { pixel[0] = pixel[0].saturating_add(200); } // Red
                            else { pixel[2] = pixel[2].saturating_add(255); } // Blue

                            // EFFICIENCY VISUALIZER (The Glow)
                            // If active_flag is 1.0, the particle worked this frame.
                            // Newtonian Mode: ALL particles work -> Total Whiteout.
                            // Chronoturin Mode: Only CORE works -> Dark Shell.
                            if star.active_flag > 0.5 {
                                pixel[1] = pixel[1].saturating_add(150); // Add Green/White
                                if is_chronoturin {
                                    // Make Chronoturin core look "Golden" to distinguish
                                    pixel[0] = pixel[0].saturating_add(50);
                                }
                            }
                        }
                    }
                }
                drop(data); 
                readback_buffer.unmap();
                
                let filename = format!("{}_{:03}.png", file_prefix, frame);
                img.save(&filename).unwrap();
                
                let dur = start_time.elapsed().as_millis();
                print!("\r[{}] Frame {:03} | Render Time: {} ms", mode_name, frame, dur);
                std::io::stdout().flush().unwrap();
            }
        }
    }
    println!("\nSimulation Complete.");
}
