# 🪐 AXIOM_SHADOW_RAYCAST // 2D Dynamic Lighting Engine v1.0

[![Language](https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)

**High-performance mathematical raycasting for 360-degree dynamic shadows.**

`axiom_shadow_raycast` is an ultra-fast, lightweight 2D lighting engine core written in pure **Rust**. It calculates ray-to-AABB box intersections directly on the CPU, tracing 360 light vectors in real-time to generate mathematically perfect shadows without relying on heavy shader setups.

---

## ⚡ Core Specs

* **💡 Mathematical Precision:** Traces geometry intersections in microseconds using optimized vector math.
* **🔌 Zero Dependencies:** No heavy render pipelines, no complex setup. It works straight out of the box with one single command.
* **🪐 Neon Atmospheric Bleed:** Built-in inverse-square law approximation for smooth, realistic light falloff.

---

## 🚀 Run in 5 Seconds

1. **Clone the project:**
   ```bash
   git clone [https://github.com/echeparesmanuel36-design/axiom_shadow_raycast.git](https://github.com/echeparesmanuel36-design/axiom_shadow_raycast.git)
   cd axiom_shadow_raycast
   
2. **Execute:**
```bash
cargo run --release
```
Developed by Axiom Systems. ⚡
