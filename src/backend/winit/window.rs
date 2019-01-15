use crate::backend::webrender::Webrenderer;
use crate::backend::winit::EventLoop;
use crate::{Component, Input, Size, UiData, UiInput, UiLayout, UiRender, UiUpdate, UiView};
use gleam::gl;
use glutin::GlContext;

static FONT: &[u8] = include_bytes!("../../../assets/fonts/OpenSans-Regular.ttf");

#[derive(Default)]
pub struct AppProps;

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
        Comp::Props: Default,
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
                self.size.w as f64,
                self.size.h as f64,
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
        let mut data = UiData::default();
        let app_id = data.fresh_id();

        let mut renderer = Webrenderer::new(eventloop.create_proxy(), gl.clone(), dpr);
        renderer.resize(self.size.w, self.size.h, dpr);
        let default_font = data.font_queue.add(FONT);
        renderer.handle_fontqueue(&mut data.font_queue);

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
                            renderer.resize(self.size.w, self.size.h, dpr);
                            fresh = true;
                        }
                        WindowEvent::HiDpiFactorChanged(new_dpr) => {
                            dpr = (*new_dpr) as f32;
                            renderer.resize(self.size.w, self.size.h, dpr);
                        }
                        _ => (),
                    },
                    _ => (),
                }
                input.push_event(event);
            }

            {
                UiInput::<Comp>::run(&mut data, &mut input, app_id);

                if fresh | UiUpdate::run(&mut data, app_id) {
                    fresh = false;
                    renderer.handle_fontqueue(&mut data.font_queue);

                    UiView::<Comp>::run(&mut data, app_id, Comp::Props::default());

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
