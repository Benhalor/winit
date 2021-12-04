use super::{super::monitor, device, proxy::Proxy, window};
use crate::dpi::{PhysicalSize, Size};
use crate::event::{
    DeviceEvent, DeviceId, ElementState, Event, KeyboardInput, TouchPhase, WindowEvent,
};
use crate::event_loop::ControlFlow;
use crate::monitor::MonitorHandle as RootMH;
use crate::window::{Theme, WindowId};
use std::cell::RefCell;
use std::clone::Clone;
use std::collections::{vec_deque::IntoIter as VecDequeIter, VecDeque};
use std::rc::Rc;




pub struct WindowTarget<T: 'static> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Clone for WindowTarget<T> {
    fn clone(&self) -> Self {
        WindowTarget {
        }
    }
}

impl<T: 'static> WindowTarget<T> {
    pub fn available_monitors(&self) -> VecDequeIter<monitor::Handle> {
        VecDeque::new().into_iter()
    }

    pub fn primary_monitor(&self) -> Option<RootMH> {
        Some(RootMH {
            inner: monitor::Handle,
        })
    }
}
