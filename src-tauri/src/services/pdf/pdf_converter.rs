use std::sync::Arc;

use pdfium_render::prelude::*;
use tracing::debug;

use crate::deserializers::command_message_ipc::CommandPayloadIPC;
use windows::{core::w, core::HSTRING, Win32::UI::WindowsAndMessaging::*};

pub fn pdf_to_jpg(
    thread_id: usize,
    job_cmd: CommandPayloadIPC,
    pdfium: &Arc<pdfium_render::prelude::Pdfium>,
) {
    debug!("Execute command for {:?}", job_cmd);

    let document: Result<PdfDocument<'_>, PdfiumError> =
        pdfium.load_pdf_from_file(&job_cmd.resource_path, job_cmd.password.as_deref());

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
                            Ok(_) => debug!(
                                "Thread {}: Saved page {} as {}",
                                thread_id,
                                p + 1,
                                output_file
                            ),
                            Err(err) => debug!(
                                "Thread {}: Failed to save page {} as {}: {:?}",
                                thread_id,
                                p + 1,
                                output_file,
                                err
                            ),
                        };
                    }
                    Err(err) => {
                        debug!(
                            "Thread {}: Failed to render page {}: {:?}",
                            thread_id,
                            p + 1,
                            err
                        );
                        let err_text =
                            HSTRING::from(format!("Failed to render page {}: {err:?}", p + 1));
                        unsafe {
                            MessageBoxW(None, &err_text, w!("WinForge"), MB_OK | MB_ICONERROR);
                        }
                    }
                };
            }
            return;
        }
        Err(err) => {
            debug!(
                "Thread {}: Failed to load PDF document: {:?}",
                thread_id, err
            );
            let err_text = match err {
                PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::PasswordError) => {
                    HSTRING::from(format!(
                        "Failed to load PDF document: Incorrect password provided for {path}",
                        path = job_cmd.resource_path
                    ))
                }
                _ => HSTRING::from(format!("Failed to load PDF document: {err:?}")),
            };

            unsafe {
                MessageBoxW(None, &err_text, w!("WinForge"), MB_OK | MB_ICONERROR);
            }
            return;
        }
    };

    //let page = document.unwrap().pages().get(0).expect("Failed to get page 1");
}

pub fn pdf_merge(
    thread_id: usize,
    job_cmd: CommandPayloadIPC,
    pdfium: &Arc<pdfium_render::prelude::Pdfium>,
) {
    debug!("Execute command for {:?}", job_cmd);
}
