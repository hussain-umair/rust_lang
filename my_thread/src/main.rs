use rand::Rng;
use std::error::Error;
use std::{
    collections::VecDeque,
    fmt::{Debug, Display, Formatter},
};

pub type Dtq = u64;

// Thread
pub trait Thread: Debug + Display {
    fn do_work(&mut self, dur: Dtq) -> Result<Dtq, SchedError>;
}
struct MyThread {
    id: u8,
    name: String,
    runtime: Dtq,
}
impl Debug for MyThread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\nruntime: {}",
            self.id, self.name, self.runtime
        )
    }
}
impl Display for MyThread {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\nname: {}\nruntime: {}",
            self.id, self.name, self.runtime
        )
    }
}
impl Thread for MyThread {
    fn do_work(&mut self, dur: Dtq) -> Result<Dtq, SchedError> {
        if dur == 0 {
            return Err(SchedError::InvalidInterval);
        }

        let used = if self.runtime > dur {
            self.runtime -= dur;
            dur
        } else {
            let used = self.runtime;
            self.runtime = 0;
            used
        };
        Ok(used)
    }
}
impl MyThread {
    fn new(id: u8, name: &str, runtime: Dtq) -> MyThread {
        MyThread {
            id,
            name: String::from(name),
            runtime,
        }
    }
}

// Scheduler
#[derive(thiserror::Error, Debug)]
pub enum SchedError {
    #[error("invalid scheduling interval")]
    InvalidInterval,

    #[error("unexpectedly empty threads queue")]
    EmptyQueue,
}
pub trait Scheduler<T: Thread> {
    fn new() -> Self;
    fn add_threads(&mut self, thread: T);
    fn run(&mut self) -> Result<(), SchedError>;
    fn log_scheduling(&self, thread: &T) {
        print!("{:>4}", self.get_time());
        println!(
            " {scheduler} scheduled {thread}",
            scheduler = Self::get_name()
        );
    }
    fn get_time(&self) -> Dtq;
    fn get_name() -> &'static str;
}
// Test Scheduler
pub struct TestScheduler {
    run_queue: VecDeque<MyThread>,
    current_time: Dtq,
}
const RUN_INTERVAL: Dtq = 10;
impl Scheduler<MyThread> for TestScheduler {
    fn new() -> Self {
        Self {
            run_queue: VecDeque::new(),
            current_time: 0,
        }
    }

    fn add_threads(&mut self, thread: MyThread) {
        self.run_queue.push_back(thread);
    }

    fn run(&mut self) -> Result<(), SchedError> {
        loop {
            let mut t = self.run_queue.pop_front().ok_or(SchedError::EmptyQueue)?;

            self.log_scheduling(&t);

            assert!(RUN_INTERVAL < f64::MAX as Dtq);
            assert!(RUN_INTERVAL < i64::MAX as Dtq);

            let variance: f64 = rand::thread_rng()
                .gen_range((-0.5 * RUN_INTERVAL as f64)..=(0.5 * RUN_INTERVAL as f64));

            let interval = (RUN_INTERVAL as i64 + variance as i64) as u64;

            let runtime = t.do_work(interval)?;

            self.current_time += runtime;

            if runtime < interval {
                if self.run_queue.is_empty() {
                    return Ok(());
                }
            } else {
                self.run_queue.push_back(t);
            }
        }
    }

    fn get_time(&self) -> Dtq {
        self.current_time
    }
    fn get_name() -> &'static str {
        "TestShed"
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    const NUM_THREADS: u8 = 3;
    const MIN_RUNTIME: Dtq = 18;
    const MAX_RUNTIME: Dtq = 32;

    let mut test_threads = Vec::new();

    for i in 1..NUM_THREADS + 1 {
        let t = MyThread::new(
            i,
            format!("THREAD_{i}").as_str(),
            rand::thread_rng().gen_range(MIN_RUNTIME..=MAX_RUNTIME),
        );

        test_threads.push(t);
    }

    let mut scheduler = TestScheduler::new();

    for t in test_threads {
        scheduler.add_threads(t);
    }

    scheduler.run()?;
    Ok(())
}
