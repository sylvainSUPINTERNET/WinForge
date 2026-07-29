use std::fs::OpenOptions;
use std::io::Write;

use crate::{events::EVENT_WATCHERS_FOLDER_LIST_REFRESH, pipe::UI_PIPE_NAME};

pub fn send_ui_notification() -> Result<(), Box<dyn std::error::Error>> {
    let mut pipe = OpenOptions::new()
        .read(false)
        .write(true)
        .open(UI_PIPE_NAME)?;

    pipe.write_all(EVENT_WATCHERS_FOLDER_LIST_REFRESH.as_bytes())?;

    Ok(())
}


// use tokio::net::windows::named_pipe::ClientOptions;
// use tokio::io::AsyncWriteExt;

// use crate::pipe::UI_PIPE_NAME;


// pub async fn send_ui_notification() -> Result<(), Box<dyn std::error::Error>> {
//     let mut pipe = ClientOptions::new()
//         .open(UI_PIPE_NAME)?;
//     pipe.write_all(EVENT_WATCHERS_FOLDER_LIST_REFRESH.as_bytes()).await?;

//     Ok(())
// }
