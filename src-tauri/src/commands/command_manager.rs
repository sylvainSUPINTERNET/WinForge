use std::sync::Arc;

use tracing::{debug, info};
use crate::{
    deserializers::command_message_ipc::CommandPayloadIPC,
    };
use crate::services::pdf::pdf_converter::pdf_to_jpg;

const CONVERT_PDF_TO_JPG: &str = "pdfToJpg";

pub fn execute(thread_id: usize, job_cmd: CommandPayloadIPC, pdfium: &Arc<pdfium_render::prelude::Pdfium>) {

    match job_cmd.cmd_name.as_str() {
        CONVERT_PDF_TO_JPG => {
            pdf_to_jpg(thread_id, job_cmd, pdfium);
        }
        _ => {
            debug!("Thread {} received unknown command: {:?}", thread_id, job_cmd.cmd_name);
        }
    }
}