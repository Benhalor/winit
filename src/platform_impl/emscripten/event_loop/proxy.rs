use crate::event::Event;
use crate::event_loop::EventLoopClosed;

pub struct Proxy<T: 'static> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: 'static> Proxy<T> {
    pub fn new() -> Self {
    }

    pub fn send_event(&self, event: T) -> Result<(), EventLoopClosed<T>> {
        
        Ok(())
    }
}

impl<T: 'static> Clone for Proxy<T> {
    fn clone(&self) -> Self {
        Proxy {
        }
    }
}
