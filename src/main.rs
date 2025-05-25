mod code;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use biologischer_log::{init_logger, CustomLogger};
use libgm::{parse_data_file, read_data_file, GMData};
use libgm::gm::GMRoom;
use log::info;
use pixels::Pixels;
use winit::window::Window;
use code::run::Stack;
use crate::code::run::Variables;

#[derive(Debug)]
pub struct App {
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

    app.run_code(0, 0)?;

    app.logger.shutdown();
    Ok(())
}