mod code;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use libgm::{parse_data_file, read_data_file, GMData};
use biologischer_log::{init_logger, CustomLogger};
use libgm::gm::{GMRoom, GMValue};
use log::info;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use code::run::Stack;
use crate::code::run::Variables;

struct App {
    logger: Arc<CustomLogger>,

    window: Option<Window>,
    pixels: Option<Pixels<'static>>,

    data: GMData,
    window_title: String,
    window_width: u32,
    window_height: u32,
    current_room: GMRoom,
    stack: Stack,
    variables: Variables,
}


impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Application resumed");
        let window_attributes = WindowAttributes::default()
            .with_title(&self.window_title)
            .with_inner_size(PhysicalSize::new(self.window_width, self.window_height));

        let window: Window = event_loop.create_window(window_attributes).expect("Could not create window");
        let size: PhysicalSize<u32> = window.inner_size();

        // SAFETY HACK: Convert the Window ref into a 'static one.
        // This is safe because we are storing the Window alongside the Pixels object
        let static_window: &'static Window = unsafe { std::mem::transmute(&window) };

        let surface_texture: SurfaceTexture<&Window> = SurfaceTexture::new(size.width, size.height, static_window);
        let pixels: Pixels = Pixels::new(640, 480, surface_texture).expect("Failed to create Pixels");

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                self.logger.shutdown();
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    let frame: &mut [u8] = pixels.frame_mut();
                    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                        let x: u8 = (i % self.window_width as usize) as u8;
                        let y: u8 = (i / self.window_width as usize) as u8;
                        pixel.copy_from_slice(&[x, y, 128, 255]);
                    }
                    pixels.render().unwrap();
                }
            }
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                match event.physical_key {
                    PhysicalKey::Unidentified(_) => {}
                    PhysicalKey::Code(keycode) => {
                        if keycode == KeyCode::Escape {
                            info!("User pressed escape");
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), String> {
    let logger: Arc<CustomLogger> = init_logger(env!("CARGO_PKG_NAME"));
    info!("=======================================");
    info!("|    Acorn GameMaker Runner v0.1.0     ");
    info!("=======================================");

    let data_path: PathBuf = Path::new("./data.win").canonicalize()
        .map_err(|e| format!("Could not find data.win in current directory: {e}"))?;
    info!("Loading data file {data_path:?}");

    let raw_data: Vec<u8> = read_data_file(&data_path)?;
    let data: GMData = parse_data_file(raw_data)?;

    info!("============= General Info ============");
    info!("| Game Name: {}", data.general_info.display_name.display(&data.strings));
    info!("| GameMaker Version: {}", data.general_info.version);
    info!("| Bytecode Version: {}", data.general_info.bytecode_version);
    info!("=======================================");

    let first_room_id: usize = data.general_info.room_order[0] as usize;
    let first_room: GMRoom = data.rooms.rooms_by_index[first_room_id].clone();
    let window_title: String = data.general_info.display_name.resolve(&data.strings.strings_by_index)?.to_owned();

    let event_loop: EventLoop<()> = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App {
        logger,
        window: None,
        pixels: None,
        window_title,
        window_width: data.general_info.default_window_width,
        window_height: data.general_info.default_window_height,
        current_room: first_room,
        data,
        stack: Stack::new(),
        variables: Variables {
            globals: HashMap::new(),
            instances: HashMap::new(),
            locals: HashMap::new(),
        },
    };
    event_loop.run_app(&mut app)
        .map_err(|e| format!("An error has occurred in the event loop: {e}"))
}

