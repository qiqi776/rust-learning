use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>, // 持有 N 个 Worker
    sender: mpsc::Sender<Job>,
}

// 任务类型
type Job = Box<dyn FnOnce() + Send + 'static>;

// 创建一个worker结构体
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>, // 真正的持有thread句柄
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
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
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }

    
}

