use std::thread::{spawn, JoinHandle};

pub struct ThreadPool {
    joinhandles: Vec<JoinHandle<()>>
}
impl ThreadPool {
    pub fn new() -> Self {
        ThreadPool { joinhandles: Vec::new() }
    }
    pub fn add(&mut self, stop_if_num_gt: usize, func: impl FnOnce() -> () + std::marker::Send + 'static) {
        if self.joinhandles.len() == stop_if_num_gt + 1 {
            self.joinhandles.remove(0).join().unwrap();
        }
        self.joinhandles.push(spawn(func));
    }
}