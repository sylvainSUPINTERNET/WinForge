use core::option::Option::Some;
use tracing::{debug, error, info, warn};

use super::reg_values;
use crate::services::image_converter;

pub fn menu_select_action(args: Vec<String>) {
    debug!("args: {:?}", args);

    match args.get(1).map(String::as_str) {
        Some(reg_values::IMAGE_PNG_TO_JPEG) => image_converter::convert_png_to_jpeg(args),
        _ => warn!("Unknown reg value"),
    }
}
