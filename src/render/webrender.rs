use glutin::{self, GlContext};
use gleam::gl;
use winit;
use euclid;

use webrender::{self, api::*};
use crate::{Ui, Component, WidgetId, Size, Window};
use super::eventloop::EventLoop;
use super::notifier::Notifier;

pub fn run<State, Msg>(window: Window<State, Msg>)
where
    State: 'static,
    Msg: 'static,
{
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
        //scatter_gpu_cache_updates: false,
        debug_flags: webrender::DebugFlags::ECHO_DRIVER_MESSAGES,
        ..webrender::RendererOptions::default()
    };

    let framebuffer_size = {
        let size = window
            .get_inner_size()
            .unwrap()
            .to_physical(device_pixel_ratio as f64);
        DeviceUintSize::new(size.width as u32, size.height as u32)
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

    let mut ui = Ui::new();
    ui.window_size = Size::new(600.0, 400.0);
    let app_id = ui.new_widget();

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

        let root_widget = ui.root_widget;
        app(app_id, &mut UpdateContext::new(&mut ui, root_widget));

        {
            let mut builder = DisplayListBuilder::new(pipeline_id, layout_size);
            let mut txn = Transaction::new();
            ui.update(|primitive, ui| {
                use visage::primitive::{Rectangle, PrimitiveKind};
                match primitive.kind {
                    PrimitiveKind::Rectangle => {
                        let position = ui.position[*primitive.id];
                        let size = ui.size[*primitive.id];
                        let style: <Rectangle as Widget>::Style = *ui.styles.get(primitive.id).unwrap().downcast_ref().unwrap();
                        let color = style.color;

                        let mut info = LayoutPrimitiveInfo::new(LayoutRect::new(
                            LayoutPoint::new(position.x, position.y),
                            LayoutSize::new(size.w, size.h)
                        ));
                        info.tag = Some((0, 1));
                        builder.push_rect(&info, ColorF::new(color.r, color.g, color.b, color.a));
                    }
                    PrimitiveKind::Text => {

                    }
                }
            });
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
