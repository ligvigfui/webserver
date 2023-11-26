pub use std::{
    collections::HashMap,
    net::TcpStream,
    sync::{ Arc, Mutex},
};

use std::{
    thread,
    time,
    sync::mpsc,
};

pub use crate::{
    extensions::IsHex,
    server_functions::{*,
        routing::*,
        handling::*,
        status_codes::*,
        method::*,
        request::*,
    },
    neptunCRF::{User, hash::handle_neptun_login}
};

pub mod server_functions;
#[allow(non_snake_case)]
pub mod neptunCRF;
pub mod wedding;
pub mod dev;
pub mod vue;
pub mod extensions;

pub static VERSION: &str = "0.2.0";
pub static DEBUG: DebugLevel = DebugLevel::HIGH;
pub static DEBUG_LEN: usize = 200;

#[derive(PartialEq, PartialOrd)]
pub enum DebugLevel {
    LOW,
    MEDIUM,
    HIGH,
}

/// Returns the current time in seconds since the Unix epoch.
pub fn now() -> u64 {
    time::SystemTime::now().duration_since(time::UNIX_EPOCH).expect("Time went backwards").as_secs()
}


/// Returns a string with the current time in the format "YYYY-MM-DD HH:MM:SS"
/// with a two hour offset.
pub fn readable_time() -> String {
    let current_date_time = chrono::NaiveDateTime::from_timestamp_opt(now() as i64, 0)
        .expect("Invalid timestamp");
    //get the current timezone offset
    let offset = chrono::Local::now().offset().local_minus_utc() as i64;
    let local_date_time = current_date_time + chrono::Duration::seconds(offset);
    local_date_time.format("%Y-%m-%d %H:%M:%S").to_string()
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
            if DEBUG >= DebugLevel::LOW {
                println!("Shutting down worker {}", worker.id)};

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
                    if DEBUG >= DebugLevel::HIGH {
                        println!("Worker {id} got a job; executing.");}

                    job();
                }
                Err(_) => {
                    if DEBUG >= DebugLevel::MEDIUM {
                        println!("Worker {id} disconnected; shutting down.");}
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_request_creation(){
        let test = format!("GET / HTTP/1.1\r\n{}{}{}{}{}",
            "Host: localhost:7878\r\n",
            "Connection: keep-alive\r\n",
            "Content-Length: 40\r\n",
            "Accept-Language: en-US,en;q=0.9\r\n\r\n",
            "hjafshfas\r\n\r\ndkgsgoaw sdhf\r\nasdkgfvs ewu");
        let test_request = Request::from(test.as_bytes()).unwrap();
        assert_eq!(test_request.method, Method::GET);
        assert_eq!(test_request.path, "/");
        assert_eq!(test_request.protocol, "HTTP/1.1");
        assert_eq!(test_request.headers.get("Host"), Some(&"localhost:7878"));
        assert_eq!(test_request.headers.get("Connection"), Some(&"keep-alive"));
        assert_eq!(test_request.headers.get("Content-Length"), Some(&"40"));
        assert_eq!(test_request.headers.get("Accept-Language"), Some(&"en-US,en;q=0.9"));
        assert_eq!(test_request.body, "hjafshfas\r\n\r\ndkgsgoaw sdhf\r\nasdkgfvs ewu");
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn time_test() {
        let time = now();
        assert!(time > 0);
    }
}
