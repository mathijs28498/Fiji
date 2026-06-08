<p align="center">
<img src="assets/images/demo_fiji_3d.gif" alt="Fiji 3D Example" width="100%"/>
</p>

# Fiji: Declarative Vulkan Renderer

## Overview
Fiji is a Vulkan-backed, high-performance 2D/3D rendering library written in Rust. It was built to provide a simple, declarative, and safe API (heavily inspired by Processing and p5.js) while leveraging the extreme performance of Vulkan under the hood. It abstracts away the massive boilerplate of Vulkan (swapchains, render passes, pipelines, and memory allocators) into a clean, immediate-mode-style builder API.

## Technical Highlights & Architecture

* **API-Agnostic Abstraction:** The user-facing API is completely decoupled from Vulkan. Users define scenes using closures and the Builder pattern (e.g., `Block::new_default().with_color()`), allowing the backend to efficiently batch and dispatch draw calls.
* **Safe Vulkan Interop:** Leverages `vulkano` and Rust's ownership model to safely manage GPU memory, command buffers, and pipeline states without undefined behavior.
* **CPU Text Rasterization:** Implements fast, high-quality text rendering using `fontdue` for CPU rasterization, backed by automatic per-glyph GPU texture caching.
* **Unified UI & World Spaces:** Supports rendering objects in both 3D world space and 2D screen space, automatically handling projection matrices.
* **Dynamic Geometry Generation:** The 2D pipeline supports dynamic, user-drawn polygons at runtime, actively pushing new vertex data to the GPU buffers without stalling the render loop.
<p align="center">
<img src="assets/images/demo_fiji_2d.gif" alt="Fiji 2D Example" width="100%"/>
</p>

## Code Example
Fiji makes it trivial to spin up a high-performance window with a game loop and input handling:

```rust
use fiji::{Context, Rect, KeyCode};
use nalgebra_glm::{Vec2, Vec4};

fn main() {
    let mut context = Context::new(1280, 720, "Simple Example");
    
    context.run(move |input, event_handler, ctx| {
        if input.key_pressed(&KeyCode::Escape) {
            event_handler.exit();
        }

        ctx.rect(
            Rect::new_default()
                .with_size(Vec2::new(100., 100.))
                .with_position(Vec2::new(400., 400.))
                .with_color(Vec4::new(0.7, 0.3, 0.7, 1.))
        );

        ctx.render(event_handler);
    });
}
```

## Build Instructions

Fiji is built using standard Rust tooling (`cargo`). 

### Prerequisites
* [Rust & Cargo](https://rustup.rs/)
* Vulkan SDK (ensure your drivers support Vulkan 1.2+)

### Running the Examples
1. Clone the repository:
   ```bash
   git clone https://github.com/mathijs28498/Fiji.git
   ```
2. Navigate to the project directory.
3. Run one of the provided examples. The `cargo run --example` command will automatically resolve dependencies, compile the project, and launch the window.
   ```bash
   cargo run --release --example 3d_example
   cargo run --release --example 2d_example
   cargo run --release --example ui_example
   ```

## License
This project is licensed under the [MIT License](LICENSE).