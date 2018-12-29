use gleam::gl;
use std::rc::Rc;
use webrender::api::*;

mod notifier;
use self::notifier::Notifier;

mod primitives;
pub use self::primitives::PrimitiveRenderer;

pub type Renderer = DisplayListBuilder;

pub struct Webrenderer {
    renderer: webrender::Renderer,
    api: RenderApi,
    layout_size: LayoutSize,
    device_size: DeviceIntSize,
    document_id: DocumentId,
    pipeline_id: PipelineId,
}

impl Webrenderer {
    pub fn new(proxy: winit::EventsLoopProxy, gl: Rc<gl::Gl>, dpr: f32) -> Self {
        let opts = webrender::RendererOptions {
            device_pixel_ratio: dpr,
            clear_color: Some(ColorF::new(0.1, 0.1, 0.1, 1.0)),
            ..webrender::RendererOptions::default()
        };

        let notifier = Box::new(Notifier::new(proxy));
        let (renderer, sender) =
            webrender::Renderer::new(gl.clone(), notifier, opts, None).unwrap();
        let api = sender.create_api();
        let pipeline_id = PipelineId(0, 0);
        let layout_size = LayoutSize::new(0.0, 0.0);
        let device_size = DeviceIntSize::new(0, 0);
        let document_id = api.add_document(device_size, 0);

        {
            let mut txn = Transaction::new();
            txn.set_root_pipeline(pipeline_id);
            api.send_transaction(document_id, txn);
        }

        Webrenderer {
            renderer,
            api,
            layout_size,
            device_size,
            document_id,
            pipeline_id,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32, dpr: f32) {
        self.layout_size = LayoutSize::new(width, height);
        let (width, height) = (width * dpr, height * dpr);
        let (width, height) = (width as i32, height as i32);
        self.device_size = DeviceIntSize::new(width, height);
        self.api.set_window_parameters(
            self.document_id,
            self.device_size,
            self.device_size.into(),
            dpr,
        );
    }

    pub fn new_builder(&mut self) -> Renderer {
        DisplayListBuilder::new(self.pipeline_id, self.layout_size)
    }

    pub fn render(&mut self, builder: Renderer) {
        let mut txn = Transaction::new();
        txn.set_display_list(Epoch(0), None, self.layout_size, builder.finalize(), true);
        txn.generate_frame();
        self.api.send_transaction(self.document_id, txn);
    }

    pub fn flush(&mut self) {
        self.renderer.update();
        self.renderer.render(self.device_size).unwrap(); // TODO: Handle possible errors
        self.renderer.flush_pipeline_info();
    }

    pub fn deinit(self) {
        self.renderer.deinit();
    }
}
