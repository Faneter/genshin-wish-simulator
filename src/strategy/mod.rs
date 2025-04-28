pub mod strategies;

use crate::simulator::UpSimulator;

pub trait Strategy {
    fn run<T: UpSimulator>(&mut self, up: &mut T);
}
