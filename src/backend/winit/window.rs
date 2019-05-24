use crate::backend::webrender::Webrenderer;
use crate::backend::winit::{EventHandler, EventLoop};
use crate::{Component, Font, Input, Size, UiData, UiInput, UiLayout, UiRender, UiUpdate, UiView};
use gleam::gl;
use glutin::GlContext;

pub enum AppEvent {
    SetTitle(String),
    Quit,
}

pub struct Window {
    title: String,
    size: Size,
}

impl Window {
    pub fn new() -> Self {
        Window {
            title: String::new(),
            size: Size::new(600.0, 400.0),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn run<Comp>(mut self)
    where
        Comp: Component<Event = AppEvent>,
    {
        let mut eventloop = EventLoop::new();

        let context_builder =
            glutin::ContextBuilder::new().with_gl(glutin::GlRequest::GlThenGles {
                opengl_version: (3, 2),
                opengles_version: (3, 0),
            });

        let window_builder = winit::WindowBuilder::new()
            .with_title(self.title)
            .with_multitouch()
            .with_dimensions(winit::dpi::LogicalSize::new(
                self.size.width as f64,
                self.size.height as f64,
            ));

        let window =
            glutin::GlWindow::new(window_builder, context_builder, eventloop.events_loop())
                .unwrap();

        unsafe {
            window.make_current().ok();
        }

        let gl = match window.get_api() {
            glutin::Api::OpenGl => unsafe {
                gl::GlFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
            },
            glutin::Api::OpenGlEs => unsafe {
                gl::GlesFns::load_with(|symbol| window.get_proc_address(symbol) as *const _)
            },
            glutin::Api::WebGl => unimplemented!(),
        };

        let mut dpr = window.get_hidpi_factor() as f32;

        let mut fresh = true;
        let mut input = Input::new();
        let mut ehandler = EventHandler::new();
        let mut data = UiData::default();
        let app_id = data.fresh_id();

        let mut renderer = Webrenderer::new(eventloop.create_proxy(), gl.clone(), dpr);
        // TODO: pass `Size` directly
        renderer.resize(self.size, dpr);
        renderer.add_font(
            Font::from_family(super::DEFAULT_FONT_NAME),
            super::DEFAULT_FONT,
        );

        'main: loop {
            let events = eventloop.next();

            for event in events {
                use winit::{Event, WindowEvent};
                match event {
                    Event::WindowEvent { ref event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            break 'main;
                        }
                        WindowEvent::Resized(lsize) => {
                            self.size = Size::new(lsize.width as f32, lsize.height as f32);
                            renderer.resize(self.size, dpr);
                            fresh = true;
                        }
                        WindowEvent::HiDpiFactorChanged(new_dpr) => {
                            dpr = (*new_dpr) as f32;
                            renderer.resize(self.size, dpr);
                        }
                        _ => (),
                    },
                    _ => (),
                }
                if let Some(event) = ehandler.convert_winit_event(event) {
                    input.push_event(event);
                }
            }

            {
                UiInput::<Comp>::run(&mut data, &mut input, app_id);
                input.clear_events();

                if fresh | UiUpdate::run(&mut data, &mut renderer, app_id) {
                    fresh = false;

                    UiView::<Comp>::run(&mut data, &mut renderer, app_id, Comp::Props::default());

                    UiLayout::run(&mut data, app_id, self.size);

                    UiRender::run(&data, &mut renderer, app_id);
                    renderer.render();
                }
            }

            renderer.flush();
            window.swap_buffers().ok();
        }

        renderer.deinit();
    }
}
