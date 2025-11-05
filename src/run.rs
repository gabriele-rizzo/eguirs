use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::{
    display::DisplayOptions,
    runner::{Runnable, Runner},
};

pub fn run<T: Runnable + 'static>(
    app: Box<T>,
    options: DisplayOptions,
) -> Result<(), EventLoopError> {
    let events = EventLoop::new()?;
    let mut runner = Runner::new(&events, &options, app)?;

    events.run_app(&mut runner)
}
