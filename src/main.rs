mod constants;
mod game;
mod resource;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

struct App {
    game_world: Option<resource::GameWorld>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    width: u32,
    height: u32,
    last_frame_time: Instant,
    time_accumulator: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            width: 800,
            height: 600,
            game_world: None,
            last_frame_time: Instant::now(),
            time_accumulator: 0.0,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("Ray Tracer")
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        let game_world = game::init(self.width, self.height);

        // SurfaceTexture needs to own a reference to the window.
        // Arc::clone(&window) creates a new Arc that points to the same window.
        let surface_texture = SurfaceTexture::new(self.width, self.height, Arc::clone(&window));

        // Pixels::new will return a Pixels<'static> because surface_texture owns the Arc.
        let pixels = Pixels::new(self.width, self.height, surface_texture).unwrap();

        self.window = Some(window);
        self.game_world = Some(game_world);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(new_size) => {
                self.width = new_size.width;
                self.height = new_size.height;

                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(new_size.width, new_size.height);
                    let _ = pixels.resize_buffer(new_size.width, new_size.height);

                    if let Some(game_world) = &mut self.game_world {
                        game_world
                            .grid
                            .change_window(new_size.width, new_size.height);
                    }
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    winit::event::KeyEvent {
                        physical_key: PhysicalKey::Code(keycode),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                if let Some(game_world) = &mut self.game_world {
                    match keycode {
                        KeyCode::KeyW => {
                            game_world.change_direction(resource::Direction::Up);
                        }
                        KeyCode::KeyA => {
                            game_world.change_direction(resource::Direction::Left);
                        }
                        KeyCode::KeyS => {
                            game_world.change_direction(resource::Direction::Down);
                        }
                        KeyCode::KeyD => {
                            game_world.change_direction(resource::Direction::Right);
                        }
                        _ => (),
                    }
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    // Clear the frame to black
                    frame.fill(0);

                    let now = Instant::now();
                    let delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
                    self.last_frame_time = now;
                    self.time_accumulator += delta_time;

                    if let Some(game_world) = &mut self.game_world {
                        let time_per_update = 1.0 / game_world.snake_fps;
                        while self.time_accumulator >= time_per_update {
                            game_world.check_for_apple();
                            game_world.move_snake();
                            self.time_accumulator -= time_per_update;
                        }
                        // draw score
                        game::draw_number(game_world.score, 1, 1, pixels, self.width, game_world);
                        // draw apple
                        let apple = game_world.add_circle(game_world.apple[0], game_world.apple[1]);
                        game::draw_object(&apple.points, pixels, self.width);
                        // draw snake
                        for point in game_world.snake_body.clone() {
                            let square = game_world.add_square(point[0], point[1]);
                            game::draw_object(&square.points, pixels, self.width);
                        }
                    }

                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        event_loop.exit();
                    }
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
