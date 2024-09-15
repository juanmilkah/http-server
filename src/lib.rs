use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct Threadpool{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let thread = thread::spawn(move|| loop {
            let message = receiver.lock().unwrap().recv();
            
            match message{
                Ok(job) => {
                    println!("Worker {id} received job and executing!");
                    job();
                },
                Err(_) => {
                    println!("Worker {id} disconnected; Shutting Down!");
                    break;
                }
            }
        });
        Worker{id,thread: Some(thread)}
    }
}

impl Threadpool{
    pub fn new(size: usize)-> Threadpool{
        /*panic if less than zero*/
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Threadpool{workers, sender: Some(sender)}
    }
    pub fn execute<F>(&self, f:F) where F:FnOnce() + Send + 'static, {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for Threadpool{
    fn drop(&mut self){
        for worker in &mut self.workers{
            /*drop the senders b4 the receiving end*/
            drop(self.sender.take());
            println!("Droping Worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
