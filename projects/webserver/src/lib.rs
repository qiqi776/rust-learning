use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>, // 持有 N 个 Worker
    sender: Option<mpsc::Sender<Job>>,
}

// 任务类型
type Job = Box<dyn FnOnce() + Send + 'static>;

// 创建一个worker结构体
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>, // 真正的持有thread句柄
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

        Worker {id, thread: Some(thread),}
    }
}



// 这是线程池的实现
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // 建立任务队列，包含一个接收端一个发送端
        let (sender, receiver) = mpsc::channel();

        // 利用原子引用Arc，共享接收端
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        // 创建worker
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    // 派发任务
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // 发送一个空任务，通知worker线程退出
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}