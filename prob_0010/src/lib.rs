use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn schedule<F, T> (task: F, ms: u64) -> JoinHandle<T> where
    F: 'static + Send + FnOnce() -> T,
    T: 'static + Send {

    thread::spawn(move || -> T {
        let seconds = ms / 1000;
        let nano = ((ms * 1000 * 1000) % 1000*1000*1000) as u32;
        thread::sleep(Duration::new(seconds, nano));
        task()
    })
}