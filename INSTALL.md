# Installation & Setup Guide

Chronoturin is a high-performance simulation engine. Because it interacts directly with your GPU via **WebGPU**, it requires a specific environment setup.

## 1. Prerequisites

### Install Rust
Chronoturin is written in Rust. You need the standard Rust toolchain.
* **Windows:** Download `rustup-init.exe` from [rust-lang.org](https://www.rust-lang.org).
* **Mac/Linux:**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```
* *Verify:* Open a terminal and type `rustc --version`.

### FFmpeg (Optional)
To convert the image frames into a video, install FFmpeg.
* **Windows:** `winget install ffmpeg`
* **Mac:** `brew install ffmpeg`
* **Linux:** `sudo apt install ffmpeg`

---

## 2. Project Setup

1.  **Clone/Create the Repo:**
    ```bash
    git clone https://github.com/En0cH-CrYpT0-JT/Chronoturin-The-Relativistic-Optimization-Engine.git chronoturin
    cd chronoturin
    ```
    *(Or if starting from scratch)*:
    ```bash
    cargo new chronoturin
    cd chronoturin
    ```


2.  **Configure Dependencies:**
    Open the `Cargo.toml` file in the root directory. Replace its contents with the following. **This specific version combination is required** for the simulation to compile correctly.

    ```toml
    [package]
    name = "chronoturin"
    version = "1.0.0"
    edition = "2021"

    [dependencies]
    # Graphics & Compute
    wgpu = "0.19"
    pollster = "0.3"
    bytemuck = { version = "1.14", features = ["derive"] }

    # Math & Logic
    rand = "0.8"
    
    # Visualization
    image = "0.24"
    ```

3.  **Add the Source Code:**
    * Ensure `src/main.rs` contains the Host Code.
    * Ensure `src/shader.wgsl` contains the Compute Shader Code.
    * *(If you downloaded this repo, these files are already in place).*

---

## 3. Running the Simulation

**⚠️ IMPORTANT:** You must run this in **Release Mode**.
Debug mode (default) adds safety checks that make physics simulations run 10x slower.

1.  **Run the Command:**
    ```bash
    cargo run --release
    ```

2.  **What Happens Next:**
    * The engine will initialize your GPU.
    * It will run **Pass 1: Newtonian Mode** (Generating `newton_*.png`).
    * It will run **Pass 2: Chronoturin Mode** (Generating `chrono_*.png`).
    * *Note: This process is disk-heavy. It saves a high-res image every frame.*

3.  **Check Results:**
    Open your project folder. You will see 300+ PNG files.
    * **Newtonian Images:** Should look bright white/active everywhere.
    * **Chronoturin Images:** Should look like a glowing golden core with a dark shell.

---

## Troubleshooting

**"Error: could not find Maintain"**
* This means your `Cargo.toml` is using an old version of `wgpu`. Ensure it is set to `"0.19"`.

**"Performance is slow"**
* Did you forget the `--release` flag?
* Note that the "Render Time" printed in the console includes the time it takes to save the PNG to your hard drive (~130ms). The physics itself is running in <2ms.
