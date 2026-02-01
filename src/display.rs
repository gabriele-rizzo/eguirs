use glium::{backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface};
use winit::{event_loop::EventLoop, window::Window};

pub struct DisplayOptions {
    pub title: String,
    pub resizable: bool,
    pub size: winit::dpi::LogicalSize<u32>,
}

impl DisplayOptions {
    pub fn new(title: &str, size: winit::dpi::LogicalSize<u32>, resizable: bool) -> Self {
        Self {
            title: title.to_string(),
            size,
            resizable,
        }
    }
}

pub(crate) struct Display {
    pub(crate) window: Window,
    pub(crate) display: glium::Display<WindowSurface>,
}

impl Display {
    pub(crate) fn new(events: &EventLoop<()>, options: &DisplayOptions) -> Self {
        let attributes = Window::default_attributes().with_resizable(options.resizable);
        let (window, display) = SimpleWindowBuilder::new()
            .set_window_builder(attributes)
            .with_inner_size(options.size.width, options.size.height)
            .with_title(&options.title)
            .build(events);

        Self { window, display }
    }
}
