use egui::ViewportId;
use egui_glium::EguiGlium;
use glium::Frame;
use winit::{
    error::EventLoopError,
    event_loop::{ActiveEventLoop, EventLoop},
};

use crate::display::{Display, DisplayOptions};

pub(crate) mod handler;

pub trait Runnable {
    fn ui(&mut self, quit: &mut bool, context: &egui::Context);
    fn draw(&mut self, display: &glium::Display<glium::glutin::surface::WindowSurface>) -> Frame;
}

pub(crate) struct Runner {
    eg: EguiGlium,
    display: Display,
    app: Box<dyn Runnable>,
    quit: bool,
}

impl Runner {
    pub fn new(
        events: &EventLoop<()>,
        options: &DisplayOptions,
        app: Box<dyn Runnable>,
    ) -> Result<Self, EventLoopError> {
        let display = Display::new(&events, &options);
        let eg = EguiGlium::new(ViewportId::ROOT, &display.display, &display.window, &events);

        Ok(Self {
            eg,
            display,
            app,
            quit: false,
        })
    }

    fn redraw(&mut self, events: &ActiveEventLoop) {
        self.eg.run(&self.display.window, |context| {
            self.app.ui(&mut self.quit, context)
        });

        if self.quit {
            events.exit()
        }

        let mut target = self.app.draw(&self.display.display);

        self.eg.paint(&self.display.display, &mut target);
        target.finish().unwrap();
    }
}
