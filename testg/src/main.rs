use std::collections::VecDeque;


trait Job {
    fn execute(&self);
}

struct PngToJpeg {
    name: String
}
struct PngToPdf {
    name: String
}

impl Job for PngToJpeg {
    fn execute(&self) {
        println!("Job : PngToJpeg.")
    }
}
impl Job for PngToPdf {
    fn execute(&self) {
        println!("Job : PngToPDF");
    }
}

pub struct JobQueue {
    jobs: VecDeque<Box<dyn Job>>
}

#[allow(unused_variables)]
fn main() {

    let mut job_queue = JobQueue {
        jobs: VecDeque::new()
    };

    let job1 = Box::new(PngToJpeg {
        name: String::from("hello png to jpeg")
    });

    let job2 = Box::new(PngToPdf {
        name: String::from("Hello png to pdf")
    });

    job_queue.jobs.push_front(job1);
    job_queue.jobs.push_front(job2);
    
    while let Some(job) = job_queue.jobs.pop_front() {
        job.execute();
    }

    /*
    let job1 = PngToJpeg {
        name: String::from("job Uno"),
    };

    let job2 = Job {
        name: String::from("job Secondo"),
    };
    */

    //job_queue.jobs.push_back(job1);
    //job_queue.jobs.push_back(job2);

    //while let Some(job) = job_queue.jobs.pop_front() {
    //    job.execute();
    //}



}
