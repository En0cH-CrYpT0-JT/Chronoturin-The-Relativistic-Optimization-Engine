use std::time::Instant;
use wgpu::util::DeviceExt;
use rand::prelude::*;

// --- CONFIGURATION ---
const NUM_PARTICLES: u32 = 10_000;
const FRAMES: u32 = 600;
const BASELINE_DT: f32 = 0.01;

// UPDATE 1: WIDER SWEEP
// We test a huge range (100.0 down to 0.5) to ensure we capture the sleep transition.
const SENSITIVITY_LEVELS: [f32; 6] = [100.0, 50.0, 25.0, 10.0, 5.0, 1.0]; 

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    dt: f32,
    mode: u32,
    sensitivity: f32,
}

fn main() {
    pollster::block_on(run());
}

async fn run() {
    println!("--- PARETO FRONTIER BENCHMARK (SPARSE UNIVERSE) ---");
    println!("Particles: {}", NUM_PARTICLES);
    println!("Frames: {}", FRAMES);
    println!("Computing Ground Truth (Newtonian)...");

    let instance = wgpu::Instance::default();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.unwrap();

    // UPDATE 2: SPARSE INITIALIZATION
    // We spread particles out (-5.0 to 5.0) so there is "Empty Space" where tension is low.
    // This allows the engine to actually find safe spots to sleep.
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut init_pos = Vec::new();
    let mut init_vel = Vec::new();
    for _ in 0..NUM_PARTICLES {
        init_pos.push([rng.gen_range(-5.0..5.0), rng.gen_range(-5.0..5.0)]);
        init_vel.push([0.0, 0.0]); 
    }

    // 2. RUN BASELINE (Newtonian Mode 0)
    let (baseline_pos, baseline_time) = run_simulation(&device, &queue, &init_pos, &init_vel, 0, 9999.0);
    
    println!("Baseline Time: {:.2} ms", baseline_time);
    println!("-----------------------------------------------------------------------------------");
    println!("| Sensitivity |  Runtime (ms) |  Speedup |  RMSE (Error) | Active % |");
    println!("-----------------------------------------------------------------------------------");

    // 3. RUN PARETO SWEEP
    for sensitivity in SENSITIVITY_LEVELS {
        let (chrono_pos, chrono_time) = run_simulation(&device, &queue, &init_pos, &init_vel, 1, sensitivity);

        let speedup = baseline_time / chrono_time;
        let rmse = calculate_rmse(&baseline_pos, &chrono_pos);
        let active_pct_est = (1.0 / speedup) * 100.0;

        println!("| {:11.1} | {:13.2} | {:7.1}x | {:13.6} | {:7.1}% |", 
            sensitivity, chrono_time, speedup, rmse, active_pct_est);
    }
    println!("-----------------------------------------------------------------------------------");
}

fn calculate_rmse(baseline: &Vec<[f32; 2]>, test: &Vec<[f32; 2]>) -> f32 {
    let mut error_sum = 0.0;
    for i in 0..baseline.len() {
        let dx = baseline[i][0] - test[i][0];
        let dy = baseline[i][1] - test[i][1];
        error_sum += dx*dx + dy*dy;
    }
    (error_sum / baseline.len() as f32).sqrt()
}

fn run_simulation(
    device: &wgpu::Device, 
    queue: &wgpu::Queue, 
    init_pos: &Vec<[f32; 2]>, 
    init_vel: &Vec<[f32; 2]>,
    mode: u32,
    sensitivity: f32
) -> (Vec<[f32; 2]>, f32) {
    
    let pos_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None, contents: bytemuck::cast_slice(init_pos), usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });
    let vel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None, contents: bytemuck::cast_slice(init_vel), usage: wgpu::BufferUsages::STORAGE,
    });
    let active_flags = vec![0u32; NUM_PARTICLES as usize];
    let flag_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None, contents: bytemuck::cast_slice(&active_flags), usage: wgpu::BufferUsages::STORAGE,
    });

    // LOAD SHADER
    let shader = device.create_shader_module(wgpu::include_wgsl!("../shader.wgsl"));
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None, layout: None, module: &shader, entry_point: "main",
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None, layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                     label: None, contents: bytemuck::cast_slice(&[Uniforms { dt: BASELINE_DT, mode, sensitivity }]), usage: wgpu::BufferUsages::UNIFORM
                }), offset: 0, size: None
            })},
            wgpu::BindGroupEntry { binding: 1, resource: pos_buffer.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: vel_buffer.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: flag_buffer.as_entire_binding() },
        ],
    });

    let start = Instant::now();
    
    // Command Encoding Loop
    for _ in 0..FRAMES {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None, timestamp_writes: None });
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.dispatch_workgroups((NUM_PARTICLES + 63) / 64, 1, 1);
        }
        queue.submit(Some(encoder.finish()));
    }
    
    device.poll(wgpu::Maintain::Wait);
    let runtime = start.elapsed().as_secs_f32() * 1000.0;

    // Download Results
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None, size: pos_buffer.size(), usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(&pos_buffer, 0, &staging_buffer, 0, pos_buffer.size());
    queue.submit(Some(encoder.finish()));
    
    let slice = staging_buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    device.poll(wgpu::Maintain::Wait);
    rx.recv().unwrap().unwrap();
    
    let data = slice.get_mapped_range();
    let result: Vec<[f32; 2]> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging_buffer.unmap();

    (result, runtime)
}
