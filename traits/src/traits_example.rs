use std::thread::sleep;
use std::time::Duration;

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

pub fn expensive_calculation(_n:&i32) {
    sleep(Duration::from_secs(1));
}


pub struct Progress<Iter> {
    iter: Iter,
    i:usize
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress{ iter, i:0}
    }
}

impl<Iter> Iterator for Progress<Iter> where Iter:Iterator {
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}


pub trait ProgressIteratorExt : Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}