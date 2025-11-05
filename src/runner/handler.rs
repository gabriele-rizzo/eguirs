use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::runner::Runner;

impl ApplicationHandler for Runner {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(&mut self, events: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => events.exit(),
            WindowEvent::Resized(new_size) => {
                self.display.display.resize((*new_size).into());
            }
            WindowEvent::RedrawRequested => self.redraw(events),
            _ => {}
        }

        let response = self.eg.on_event(&self.display.window, &event);

        if response.repaint {
            self.display.window.request_redraw();
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, cause: StartCause) {
        if let StartCause::ResumeTimeReached { .. } = cause {
            self.display.window.request_redraw();
        }
    }
}
