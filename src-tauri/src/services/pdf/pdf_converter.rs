use tracing::{debug, info};

use crate::deserializers::command_message_ipc::CommandPayloadIPC;

pub fn pdf_to_jpg(thread_id: usize, job_cmd:CommandPayloadIPC) {
    debug!(" ........... PDF converter service started");    
}