use crate::deserializers::command_message_ipc::CommandPayloadIPC;
use crate::events;
use crate::services::pdf::pdf_converter::{pdf_merge, pdf_to_jpg};
use std::sync::Arc;
use tauri::Emitter;
use tracing::debug;

const CONVERT_PDF_TO_JPG: &str = "pdfToJpg";
const PDF_MERGE: &str = "pdfMerge";

pub fn execute(
    thread_id: usize,
    event: String,
    pdfium: &Arc<pdfium_render::prelude::Pdfium>,
    app: &tauri::AppHandle,
) {
    match event.trim() {
        events::EVENT_WATCHERS_FOLDER_LIST_REFRESH => {
            let _ = app.emit(events::EVENT_WATCHERS_FOLDER_LIST_REFRESH, ());
        }
        command_payload => {
            let job_cmd = match serde_json::from_str::<CommandPayloadIPC>(command_payload) {
                Ok(job_cmd) => job_cmd,
                Err(error) => {
                    debug!(
                        "Thread {} failed to deserialize command payload: {}, error: {}",
                        thread_id, command_payload, error
                    );
                    return;
                }
            };

            debug!("Thread {} received job_cmd: {:?}", thread_id, job_cmd);

            match job_cmd.cmd_name.as_str() {
                CONVERT_PDF_TO_JPG => {
                    pdf_to_jpg(thread_id, job_cmd, pdfium, app);
                }
                PDF_MERGE => {
                    pdf_merge(thread_id, job_cmd, pdfium, app);
                }
                _ => {
                    debug!(
                        "Thread {} received unknown command: {:?}",
                        thread_id, job_cmd.cmd_name
                    );
                }
            }
        }
    }
}
