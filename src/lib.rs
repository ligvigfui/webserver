use std::{
    thread, 
    time::{self, SystemTime},
    sync::{ mpsc, Arc, Mutex},
};

pub use crate::{
    routing::{
        routing::*,
        handling::*,
    },
    neptunCRF::*,
};

pub mod routing;
pub mod neptunCRF;

pub static VERSION: &str = "0.1.1-dev.2";
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

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug)]
pub struct Request<'a> {
    pub method: Method,
    pub path: &'a str,
    pub protocol: &'a str,
    pub headers: Vec<(&'a str, &'a str)>,
    pub body: &'a str,
}

impl Method {
    pub fn from(method: &str) -> Result<Method, &'static str> {
        match method {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err("Invalid method"),
        }
    }
}

impl<'a> Request<'a> {
    pub fn from(buffer: &'a [u8]) -> Request<'a> {
        let str_buff = std::str::from_utf8(&buffer).unwrap();
        let (start_line, headers_and_body) = str_buff.split_once("\r\n").unwrap();
        let mut start_line_cut = start_line.split(" ");
        let (method, path, protocol) = (start_line_cut.next().unwrap(), start_line_cut.next().unwrap(), start_line_cut.next().unwrap());
        let (headers, body) = headers_and_body.split_once("\r\n\r\n").unwrap();
        let headers_iter = headers.split("\r\n");
        let mut headers_vec = Vec::new();
        for header in headers_iter {
            let mut header_cut = header.split(": ");
            let (header_name, header_value) = (header_cut.next().unwrap(), header_cut.next().unwrap());
            headers_vec.push((header_name, header_value));
        }
        Request {
            method: Method::from(method).unwrap(),
            path,
            protocol,
            headers: headers_vec,
            body,
        }
    }

    pub fn get_header(&self, header_name: &str) -> Option<&str> {
        for header in &self.headers {
            if header.0 == header_name {
                return Some(header.1);
            }
        }
        None
    }
}


impl IsHex for String {
    /// Checks if a string is hex
    /// # Returns 
    /// true if the string is hex
    /// 
    /// false if the string is not hex
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
        let test_request = Request::from(test.as_bytes());
        assert_eq!(test_request.method, Method::GET);
        assert_eq!(test_request.path, "/");
        assert_eq!(test_request.protocol, "HTTP/1.1");
        assert_eq!(test_request.headers[0], ("Host", "localhost:7878"));
        assert_eq!(test_request.headers[1], ("Connection", "keep-alive"));
        assert_eq!(test_request.headers[2], ("Content-Length", "40"));
        assert_eq!(test_request.headers[3], ("Accept-Language", "en-US,en;q=0.9"));
        assert_eq!(test_request.body, "hjafshfas\r\n\r\ndkgsgoaw sdhf\r\nasdkgfvs ewu");
    }
}
