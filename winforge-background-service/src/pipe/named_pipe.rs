use std::fs::OpenOptions;
use std::io::Write;

use crate::pipe::PIPE_NAME;

pub fn send_ui_notification() -> Result<(), Box<dyn std::error::Error>> {
    let mut pipe = OpenOptions::new()
        .read(false)
        .write(true)
        .open(PIPE_NAME)?;

    pipe.write_all(b"EVENT_WATCHERS_FOLDER_LIST_REFRESH")?;

    Ok(())
}


// use tokio::net::windows::named_pipe::ClientOptions;
// use tokio::io::AsyncWriteExt;

// use crate::pipe::PIPE_NAME;


// pub async fn send_ui_notification() -> Result<(), Box<dyn std::error::Error>> {
//     let mut pipe = ClientOptions::new()
//         .open(PIPE_NAME)?;
//     pipe.write_all(b"EVENT_WATCHERS_FOLDER_LIST_REFRESH").await?;

//     Ok(())
// }