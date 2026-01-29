# snake2: A Grid-Based Game Implementation in Rust

This document details the architecture and implementation plan for **snake2**. I have designed this project as a graphical application built entirely in Rust, utilizing minimal, low-level crates for performance and control.

## Technologies I Have Used

I have deliberately kept the external dependencies lean to ensure high performance:

*   **Language & Build System:** I am using **Rust** (specifically Rust 2024 edition) managed by **Cargo**.
*   **Windowing & Event Handling:** I rely on the **`winit`** crate (`0.30.12`) to handle cross-platform window creation, input events (keyboard), and managing the main event loop.
*   **Rendering:** I am using the **`pixels`** crate (`0.15.0`) for raw pixel buffer access, which allows me to draw directly to the framebuffer without relying on higher-level graphics APIs, ensuring maximal drawing speed.

## Project Structure and Architecture

I have organized the source code into distinct modules within the `src/` directory:

*   `src/main.rs`: This is the entry point. It sets up the `winit::EventLoop` and defines the `App` struct which implements `winit::application::ApplicationHandler`. This module is responsible for initializing the window, handling user input (WASD keys for direction control), managing the application lifecycle, and driving the main rendering loop (`WindowEvent::RedrawRequested`). I've hardcoded an initial window size of `800x600`, though it resizes gracefully.
*   `src/game.rs`: This module contains the drawing utilities. I implemented functions like `draw_object` and `draw_on_point` which handle the actual pixel manipulation onto the `pixels` buffer, including basic alpha blending for visual effects. It also contains the logic for drawing numbers (the score) using a pre-defined, 7x5 bitmap font stored in `constants::POINT_MATRIX`.
*   `src/resource.rs`: This is where I maintain the entire state of the game in the `GameWorld` struct. This struct manages the snake's body (`Vec<[u32; 2]>`), the apple's position, the current score, the game speed (`snake_fps`), and the direction of movement. The game logic, including toroidal (wrapping) movement around the screen boundaries and increasing speed upon eating an apple, is implemented here.
*   `src/constants.rs`: This module defines hardcoded constants, most notably `POINT_MATRIX`, which is a 3D array defining the 7x5 pixel representation for digits 0 through 9, used exclusively by the score drawing function.

## Core Game Mechanics Explained

1.  **Grid System:** I abstract the screen into a logical grid using the `Grid` struct within `resource.rs`. The size of each cell in this grid is fixed at **15x15 pixels** (derived from `SIZE` in `game.rs`). All game objects (snake segments, apple) are positioned on this grid.
2.  **Drawing:** All graphical elements are rendered by converting their grid coordinates to raw pixel coordinates before drawing them to the buffer provided by `pixels`. Snake segments are rendered as **Squares** (a solid 15x15 block), while the apple is rendered as a **Circle** within its 15x15 cell, using circle equation checks for rendering.
3.  **Game Loop & Timing:** I control the game's update rate (tick rate) separately from the frame rate (FPS). In `main.rs`, I accumulate `delta_time` and only call the game logic update (`move_snake`, `check_for_apple`) when the accumulated time exceeds the inverse of the target speed (`1.0 / game_world.snake_fps`). The snake speed increases every time I eat an apple.
4.  **Input Handling:** I respond to WASD keys pressed in the `main.rs` event handler, which calls `game_world.change_direction`. The movement logic in `get_next` ensures that if the snake moves off one edge, it reappears on the opposite edge (toroidal world).

## How to Build and Run

I have configured Cargo to handle all dependencies. To run this project, I must execute the following command from the project root:

```bash
cargo run
```

This command compiles all the Rust source files, downloads and compiles the `pixels` and `winit` dependencies, and then executes the binary, opening a new window titled "Ray Tracer" (which I should probably change later, but I'll leave it as-is for now as per my current inspection).

To build the release executable:

```bash
cargo build --release
```
