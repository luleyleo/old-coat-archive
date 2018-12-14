use glutin::{self, GlContext};
use gleam::gl;
use winit;
use euclid;

use webrender::{self, api::*};
use crate::{UiData, UiView, UiLayout, Component, Size, Window, AppEvent, AppProps, BoxConstraints};
use crate::ui::UiRender;
use super::eventloop::EventLoop;
use super::notifier::Notifier;

pub fn run<Comp: Component<Props=AppProps, Event=AppEvent> + 'static>(window: Window<Comp::State, Comp::Msg, Comp>) {
    let mut eventloop = EventLoop::new();
    let context_builder = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::GlThenGles {
            opengl_version: (3, 2),
            opengles_version: (3, 0),
        });
    let window_builder = winit::WindowBuilder::new()
        .with_title(window.title)
        .with_multitouch()
        .with_dimensions(winit::dpi::LogicalSize::new(600.0, 400.0));
    let window = glutin::GlWindow::new(window_builder, context_builder, eventloop.events_loop())
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

    let device_pixel_ratio = window.get_hidpi_factor() as f32;

    let opts = webrender::RendererOptions {
        device_pixel_ratio,
        clear_color: Some(ColorF::new(0.1, 0.1, 0.1, 1.0)),
        debug_flags: webrender::DebugFlags::ECHO_DRIVER_MESSAGES,
        ..webrender::RendererOptions::default()
    };

    let framebuffer_size = {
        let size = window
            .get_inner_size()
            .unwrap()
            .to_physical(device_pixel_ratio as f64);
        DeviceIntSize::new(size.width as i32, size.height as i32)
    };
    let notifier = Box::new(Notifier::new(eventloop.create_proxy()));
    let (mut renderer, sender) = webrender::Renderer::new(gl.clone(), notifier, opts, None).unwrap();
    let api = sender.create_api();
    let document_id = api.add_document(framebuffer_size, 0);

    let epoch = Epoch(0);
    let pipeline_id = PipelineId(0, 0);
    let layout_size = framebuffer_size.to_f32() / euclid::TypedScale::new(device_pixel_ratio);

    let mut txn = Transaction::new();
    txn.set_root_pipeline(pipeline_id);
    api.send_transaction(document_id, txn);

    let mut data = UiData::default();
    let app_id = data.fresh_id();

    'main: loop {
        let events = eventloop.next();

        for event in events {
            match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => break 'main,
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::KeyboardInput {
                        input: winit::KeyboardInput {
                            state: winit::ElementState::Pressed,
                            virtual_keycode: Some(key),
                            ..
                        },
                        ..
                    },
                    ..
                } => match key {
                    winit::VirtualKeyCode::Escape => break 'main,
                    _ => ()
                }
                _ => ()
            };
        }

        {
            let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);
            let mut txn = Transaction::new();

            UiView::new(&mut data, app_id)
                .start::<Comp>(AppProps::default());

            UiLayout::new(&mut data)
                .size(app_id, BoxConstraints::tight(Size::new(600.0, 400.0)));

            UiRender::new(&data, app_id)
                .render(&mut builder);

            txn.set_display_list(
                epoch,
                None,
                layout_size,
                builder.finalize(),
                true,
            );
            txn.generate_frame();
            api.send_transaction(document_id, txn);
        }

        renderer.update();
        renderer.render(framebuffer_size).unwrap();
        let _ = renderer.flush_pipeline_info();
        window.swap_buffers().ok();
    }

    renderer.deinit();
}
