use std::{
    thread, 
    time::{self, SystemTime},
    sync::{ mpsc, Arc, Mutex},
};

pub mod routing;
pub mod neptunCRF;

pub static VERSION: &str = "0.1.1-dev.1";
pub static DEBUG: bool = false;

//what does this do?
//wait for requests
//handle connection
// return OK if data is right
// return Wc if not
//send replay

//
//get request from google server with email
//send email to email with password


pub fn extract_anything(messege: &str, keyword: &str) -> Option<String> {
    //separate keyword
    let string_start = match messege.find(keyword) {
        Some(index) => index,
        None => {
            // the substring was not found
            println!("{} not found in the string" , keyword);
            return None;
        }
    };



    // extract the part of the string after "Id:"
    let string0 = &messege[(keyword.len() + string_start)..];
    // find the end of the line
    let line_end = match string0.find('\r') {
        Some(index) => index,
        None => string0.len(),
    };
    // extract the credentials string
    Some(string0[..line_end].to_string())
}
 



/// Checks if a string is hex
/// Returns true if the string is hex
/// Returns false if the string is not hex
impl IsHex for String {
    fn is_hex(&self) -> bool {
        for c in self.chars() {
            if !c.is_digit(16) {
                return false;
            }
        }
        true
    }
}

pub trait IsHex {
    fn is_hex(&self) -> bool;
    fn is_not_hex(&self) -> bool {
        !self.is_hex()
    }
}

/// Returns the current time in seconds since the Unix epoch.
pub fn now() -> u64 {
    time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs()
}


/// Returns a string with the current time in the format "YYYY-MM-DD HH:MM:SS"
/// with a two hour offset.
pub fn readable_time() -> String {
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let current_date_time = chrono::NaiveDateTime::from_timestamp_opt(current_time as i64, 0)
        .expect("Invalid timestamp");
    let current_date_time = chrono::DateTime::<chrono::Utc>::from_utc(current_date_time, chrono::Utc);
    let two_hours = chrono::Duration::hours(2);
    let future_date_time = current_date_time + two_hours;
    future_date_time.format("%Y-%m-%d %H:%M:%S").to_string()
}


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
