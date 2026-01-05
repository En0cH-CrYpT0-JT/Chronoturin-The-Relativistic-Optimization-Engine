struct Star {
    x: f32, y: f32, z: f32, 
    vx: f32, vy: f32, vz: f32,
    mass: f32, 
    data_type: f32, 
    time_debt: f32, 
    // We use padding as an "Active Flag" for visualization
    // 1.0 = UPDATED (Hot), 0.0 = SLEPT (Cold)
    active_flag: f32, 
};

struct GalaxyState {
    time_seed: f32, 
    dilation_mode: f32, // 0.0 = NEWTONIAN, 1.0 = CHRONOTURIN
    padding2: f32, padding3: f32,
};

@group(0) @binding(0) var<storage, read_write> stars: array<Star>;
@group(0) @binding(1) var<uniform> state: GalaxyState;

const G: f32 = 0.5;
const DT: f32 = 0.05;
const SAMPLES: u32 = 32; 

fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&stars)) { return; }

    var star = stars[index];

    // 1. CALCULATE TENSION (Distance Weighted)
    var tension = 0.0;
    var force_x = 0.0;
    var force_y = 0.0;
    var force_z = 0.0;

    for (var i = 0u; i < SAMPLES; i++) {
        let r_seed = vec2<f32>(f32(index) * 0.1, state.time_seed + f32(i));
        let target_idx = u32(rand(r_seed) * f32(arrayLength(&stars)));
        
        if (target_idx != index) {
            let other = stars[target_idx];
            let dx = other.x - star.x;
            let dy = other.y - star.y;
            let dz = other.z - star.z;
            let dist_sq = dx*dx + dy*dy + dz*dz + 10.0;
            let dist = sqrt(dist_sq);
            
            var interaction = -1.0; 
            if (abs(star.data_type - other.data_type) < 0.1) {
                interaction = 1.0; 
            } else {
                // Distance Weighting (The "Mandelbrot Logic")
                tension += (1.0 / dist_sq) * 10000.0; 
            }
            
            let f = (G * 500.0 * interaction) / dist_sq;
            force_x += f * (dx / dist);
            force_y += f * (dy / dist);
            force_z += f * (dz / dist);
        }
    }

    // 2. APPLY MODE LOGIC
    var dilation = 1.0; 

    if (state.dilation_mode > 0.5) {
        // --- CHRONOTURIN MODE (Smart) ---
        tension = tension / f32(SAMPLES);
        if (tension < 0.5) { 
            dilation = 0.02; // Sleep (98% Savings)
        }
    } else {
        // --- NEWTONIAN MODE (Dumb) ---
        // Always 1.0. Never sleep.
        dilation = 1.0;
    }

    // 3. PHYSICS UPDATE
    star.vx *= 0.90; star.vy *= 0.90; star.vz *= 0.90;
    star.time_debt += dilation;

    // Default to "Sleeping" state for visualization
    star.active_flag = 0.0; 

    if (star.time_debt >= 1.0) {
        star.time_debt -= 1.0;
        
        // Mark as ACTIVE (Hot!)
        star.active_flag = 1.0; 

        star.vx += force_x * DT;
        star.vy += force_y * DT;
        star.vz += force_z * DT;
        star.x += star.vx * DT;
        star.y += star.vy * DT;
        star.z += star.vz * DT;
    }

    stars[index] = star;
}
