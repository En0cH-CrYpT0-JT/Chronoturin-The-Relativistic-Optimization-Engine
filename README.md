# Chronoturin-The-Relativistic-Optimization-Engine
"Order costs Energy. Stability should be free."

![Status](https://img.shields.io/badge/Status-Experimental-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![Language](https://img.shields.io/badge/Rust-1.75+-red)

---

## 🌌 The Problem: The Newtonian Waste
In standard N-Body physics simulations (and force-directed data layouts), the engine treats every particle equally. It burns the same amount of computational energy calculating the position of a chaotic, high-speed particle as it does for a stable, settled one.

This is **Newtonian Waste**. It is the equivalent of scrubbing a clean floor with the same intensity as a dirty one.

## ⚡ The Solution: Chronoturin
Chronoturin is a **Relativistic Compute Engine** built in Rust and WebGPU. It introduces a novel optimization metric called **Local Time Dilation**.

Instead of a global time step ($\Delta t$), every particle possesses its own **Time Debt**.
1.  **Sense:** The particle calculates its local "Social Tension" (Entropy).
2.  **Dilate:**
    * If **High Tension** (Chaos/Conflict) → Time runs at **100% speed** (The Golden Core).
    * If **Low Tension** (Order/Stability) → Time **dilates** (slows down) to **2% speed**.
3.  **Result:** The system mechanically focuses 98% of your GPU's power solely on the areas that *need* to be solved.

## 👁️ Visual Proof: The "Fusion Core"
We ran a comparative benchmark separating 100,000 particles into two clusters.

### 1. Newtonian Mode (The Old Way)
* **Visual:** A blinding white sun.
* **Meaning:** The engine is calculating interactions for *every single particle*, even the ones on the outer edge that are already sorted.
* **Efficiency:** 0% (Maximum Waste).
  

<img width="522" height="817" alt="image" src="https://github.com/user-attachments/assets/b105d0f1-532f-4ff9-ac8a-0b0cc340ec8d" />


### 2. Chronoturin Mode (The New Way)
* **Visual:** A glowing golden core surrounded by a dark purple shell.
* **Meaning:** The engine has put the outer shell to "Sleep" (Purple). It is **only** spending energy on the high-conflict center (Gold).
* **Efficiency:** ~300% Speedup in pure compute scenarios.


<img width="495" height="817" alt="image" src="https://github.com/user-attachments/assets/0b618116-2149-46a2-aaa6-5d29624fc0f1" />


---

## 🔬 What We Are Proposing
* **A New Standard for Data Gravity:** Using physics to sort Big Data is powerful, but usually too expensive. Chronoturin makes it viable by ignoring "settled" data.
* **Recursive Scale Invariance:** A proof of concept for the theory that as a system gains order, its energy requirement to maintain that order should drop to near zero.

## 🚫 What We Are NOT Proposing
* **We are not "Cheating":** We do not delete particles or approximate positions.
* **We are not "Biasing":** The logic is strictly conditional. Every particle is *checked* every frame. If a "sleeping" particle suddenly feels tension, it **wakes up instantly**.

---

## 🛠️ Usage
See `INSTALL.md` for full setup instructions.

### Running the Visualizer
```bash
cargo run --release
```

### This will generate two sets of frames in your project folder:

newton_XXX.png (Baseline)

chrono_XXX.png (Optimized)

---

### 📽️ Creating the Video
If you have FFmpeg installed, stitch the frames into a comparison video:
```
ffmpeg -framerate 30 -i newton_%03d.png -c:v libx264 -pix_fmt yuv420p newtonian_baseline.mp4
```

```
ffmpeg -framerate 30 -i chrono_%03d.png -c:v libx264 -pix_fmt yuv420p chronoturin_optimized.mp4
```

## 📉 The Pareto Benchmark (Accuracy vs. Speed)

We benchmarked the engine on a sparse N-Body simulation (10,000 particles) to measure the trade-off between **Time Dilation Sensitivity** (how aggressively it sleeps) and **Runtime**.

| Sensitivity | Runtime (ms) | Speedup | RMSE (Error) | Active % |
|-------------|--------------|---------|--------------|----------|
| 100.0 | 153.22 | 190.7× | 5.673273 | 0.5% |
| 50.0  | 153.77 | 190.1× | 5.673273 | 0.5% |
| 25.0  | 16691.31 | 1.8× | 5.528395 | 57.1% |
| 10.0  | 26669.35 | 1.1× | 5.862503 | 91.3% |
| 5.0   | 29073.69 | 1.0× | 6.029157 | 99.5% |
| 1.0   | 30002.61 | 1.0× | 6.433895 | 102.7% |



**Conclusion:**
The engine allows developers to tune the "Physics Fidelity." A moderate setting (Sensitivity 25.0) nearly doubles performance by sleeping ~43% of the simulation, while an aggressive setting (Sensitivity 50.0) provides a **190x speedup** for background elements, with measurable but stable error.

**Run the Benchmark:**

Before running make sure to copy shader to replace in ```bin``` and then replace ```shader.wgsl``` in ```src```


```bash
cargo run --release --bin pareto
```

📄 Documentation
Whitepaper (PDF): Full theoretical breakdown of the Relativistic Time Dilation metric. Available in The Chronoturin Framework (Enhanced Edition)
A Computational Informational Model of Time, Recursion, and Physical Reality.
