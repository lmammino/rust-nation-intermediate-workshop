use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

type Task = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    tasks: Arc<Mutex<Vec<Task>>>,
    threads: Vec<thread::JoinHandle<()>>,
    should_stop: Arc<Mutex<bool>>,
}

impl ThreadPool {
    fn new(number_of_threads: u8) -> Self {
        let tasks: Arc<Mutex<Vec<Task>>> = Arc::new(Mutex::new(Vec::new()));
        let mut threads = Vec::new();
        let should_stop: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

        for _ in 0..number_of_threads {
            let t = tasks.clone();
            let ss = should_stop.clone();
            let thread = thread::spawn(move || loop {
                let mut tasks = t.lock().unwrap();
                if let Some(task) = tasks.pop() {
                    drop(tasks);
                    println!("{:?}: running a task", thread::current().id());
                    task();
                } else {
                    drop(tasks);
                    let should_stop = ss.lock().unwrap();
                    if *should_stop {
                        drop(should_stop);
                        println!("{:?}: STOPPING", thread::current().id());
                        break;
                    } else {
                        drop(should_stop);
                        println!("{:?}: BORING: nothing to do", thread::current().id());
                        thread::sleep(Duration::from_secs(1));
                    }
                }
            });
            threads.push(thread);
        }

        Self {
            tasks,
            threads,
            should_stop,
        }
    }

    fn execute<F: FnOnce() + Send + 'static>(&self, task: F) {
        let tasks = self.tasks.clone();
        let mut tasks = tasks.lock().unwrap();
        tasks.push(Box::new(task));
    }

    fn stop(&mut self) {
        println!("STOPPING EVERYTHING");
        let should_stop = self.should_stop.clone();
        let mut should_stop = should_stop.lock().unwrap();
        *should_stop = true;
        drop(should_stop);
        for thread in self.threads.drain(..) {
            thread.join().unwrap();
        }
    }
}

fn main() {
    let mut pool = ThreadPool::new(10);

    pool.execute(|| {
        thread::sleep(Duration::from_secs(2));
        println!("SLOW Hello from thread");
    });
    for i in 0..15 {
        pool.execute(move || {
            println!("FAST Hello from thread for task: {i}");
        });
    }

    // First we're making sure enough time is given to threads to execute the tasks
    // Then, replace this line with the `stop` method.
    thread::sleep(Duration::from_secs(3));
    pool.stop();
}
