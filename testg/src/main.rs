use pdfium_render::prelude::*;

fn main () {
    let pdfium = Pdfium::new(
        // https://github.com/bblanchon/pdfium-binaries
        Pdfium::bind_to_library("./pdfium-win-x64.dll").expect("Failed to bind to pdfium library")
    );
    let document: Result<PdfDocument<'_>, PdfiumError> = pdfium.load_pdf_from_file("test.pdf", None);
    let page = document.unwrap().pages().get(0).expect("Failed to get page 1");
    // let page = document.pages().get(4)?; // page 5

    let bitmap = page.render_with_config(
        &PdfRenderConfig::new()
            .set_target_width(page.width().value as i32)
            .set_target_height(page.height().value as i32)
    ).unwrap();
    
    // let pdfBitmap = page.render().unwrap();

    // bitmap.as_image().unwrap().save("page0.jpg").unwrap();

    // bitmap.as_image().unwrap().save("page0.jpg").unwrap();
}


// use std::{
//     collections::VecDeque,
//     sync::{Arc, Condvar, Mutex},
//     thread,
// };

// struct Shared {
//     queue: Mutex<VecDeque<u32>>,
//     condvar: Condvar,
// }

// #[allow(unused_variables)]
// fn main() {

//     let shared = Arc::new(
//         Shared {
//             queue: Mutex::new(VecDeque::new()),
//             condvar: Condvar::new(),
//         });


//     let mut threads = Vec::new();

//     // consumer
//     for i in 0..5 {
//         let s = Arc::clone(&shared);
//         let th = thread::spawn( move || {

//         loop {
//             let mut queue_guard = s.queue.lock().unwrap();
//             while queue_guard.is_empty() {
//                 queue_guard = s.condvar.wait(queue_guard).unwrap();
//             }
            

//             let val = queue_guard.pop_front();
//             drop(queue_guard);

//             println!("Thread {:?} processing value: {:?}", i, val);
//         }
            
//         });
        
//         threads.push(th);
//     }


//     // producer
//     let mut queue_guard = shared.queue.lock().unwrap();
//     queue_guard.push_front(1);
//     queue_guard.push_front(2);
//     queue_guard.push_front(3);
//     queue_guard.push_front(22);
//     drop(queue_guard);
//     shared.condvar.notify_one();    

//     let mut queue_guard2 = shared.queue.lock().unwrap();
//     queue_guard2.push_front(15);
//     queue_guard2.push_front(285);
//     drop(queue_guard2);
//     shared.condvar.notify_one();   


//     for th in threads {
//         let res = th.join();
//         println!("Result : {:?} ", res);
//     }



// }


