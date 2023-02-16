

use std::{
    sync::{mpsc, Arc, Mutex},
    thread, fmt::Display, time,
};

//what does this do?
//wait for requests
//handle connection
// return OK if data is right
// return Wc if not
//send replay

//
//get request from google server with email
//send email to email with password



#[derive(Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    sessions: Vec<Session>,
}
#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub time: i32,
}
impl User {
    pub fn new(email: String, password: String) -> User {
        User {
            email,
            password,
            sessions: vec![],
        }
    }
    fn add_session(&mut self, session_id: String) {
        for session in &mut self.sessions {
            if session.id == session_id {
                session.time = now();
                return;
            }
        }
        self.sessions.push(Session { id: session_id, time: now()});
        if self.sessions.len() > 5 {
            self.sessions.remove(0);
        }
    }
    fn can_login(&self, with_session_id: &String) -> bool {
        // if user is in sessions and loged in in the last 5 secs
        // or user is 
        let mut logged_in_in_5 = 0;
        for session in &self.sessions {
            if session.id == *with_session_id && session.time+5 > now() && logged_in_in_5 == 0 {
                return true;
            } else if session.time+5 > now() {
                logged_in_in_5 += 1;
            }
        }
        if logged_in_in_5 == 0 {
            return true;
        }
        return false;
    }
    pub fn login(&mut self, session_id: String) -> CustomResult {
        if self.can_login(&session_id) {
            self.add_session(session_id);
            return CustomResult::Ok;
        }
        return CustomResult::Mu;
    }
}

pub fn now() -> i32 {
    time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i32
}
#[derive(Debug, PartialEq)]
pub enum CustomResult {
    Ok,
    Wc,
    Mu,
}
impl Display for CustomResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomResult::Ok => write!(f, "Ok"),
            CustomResult::Wc => write!(f, "Wrong credentials"),
            CustomResult::Mu => write!(f, "Multiple users"),
        }
    }
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
