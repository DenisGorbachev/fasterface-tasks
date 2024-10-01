use std::fmt::Debug;

pub trait Widget: Debug {
    /// Dummy function; required for this trait to be a base trait of a trait object
    fn draw(&self);
}
