use std::sync::Arc;

use tracing::debug;
use pdfium_render::prelude::*;


use crate::deserializers::command_message_ipc::CommandPayloadIPC;

pub fn pdf_to_jpg(thread_id: usize, job_cmd:CommandPayloadIPC, pdfium: &Arc<pdfium_render::prelude::Pdfium>) {
    debug!("Execute command for {:?}", job_cmd);

    let document: Result<PdfDocument<'_>, PdfiumError> = pdfium.load_pdf_from_file(
        &job_cmd.resource_path, 
        job_cmd.password.as_deref());

    let without_ext = job_cmd
        .resource_path
        .rsplit_once('.')
        .map(|(name, _ext)| name)
        .unwrap_or(&job_cmd.resource_path);

    match document {
        Ok(doc) => {
            let pages = doc.pages();

            for p in 0..pages.len() {
                let page = pages.get(p).expect("Failed to get page 1");
                
                let config = PdfRenderConfig::new()
                    .set_target_width(page.width().value as i32)
                    .set_target_height(page.height().value as i32);

                match page.render_with_config(&config) {
                    Ok(bitmap) => {
                        let output_file = format!("{}_page_{}.jpg", without_ext, p + 1);

                        match bitmap.as_image().unwrap().save(&output_file) {
                            Ok(_) => debug!("Thread {}: Saved page {} as {}", thread_id, p + 1, output_file),
                            Err(err) => debug!("Thread {}: Failed to save page {} as {}: {:?}", thread_id, p + 1, output_file, err),
                        };
                    },
                    Err(err) => {
                        debug!("Thread {}: Failed to render page {}: {:?}", thread_id, p + 1, err);
                    }
                };
            }
            return;
        },
        Err(err) => {
            debug!("Thread {}: Failed to load PDF document: {:?}", thread_id, err);
            return;
        }
    };

    //let page = document.unwrap().pages().get(0).expect("Failed to get page 1");
}
