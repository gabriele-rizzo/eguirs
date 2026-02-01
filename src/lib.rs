use winit::{error::EventLoopError, event_loop::EventLoop};

use crate::runner::Runner;

pub mod display;
pub mod runner;

pub fn run(
    app: Box<dyn crate::runner::Runnable>,
    options: &crate::display::DisplayOptions,
) -> Result<(), EventLoopError> {
    let events = EventLoop::new()?;
    let mut runner = Runner::new(&events, options, app)?;

    events.run_app(&mut runner)
}
