use crate::{Font, Size};
use gleam::gl;
use std::rc::Rc;
use webrender::api::*;

mod notifier;
use self::notifier::Notifier;

mod font_manager;
pub use self::font_manager::FontManager;

pub struct Webrenderer {
    renderer: webrender::Renderer,
    layout_size: LayoutSize,
    device_size: DeviceIntSize,
    document_id: DocumentId,
    pipeline_id: PipelineId,
    pub api: RenderApi,
    pub font_manager: FontManager,
    pub builder: DisplayListBuilder,
}

impl Webrenderer {
    pub(crate) fn new(proxy: winit::EventsLoopProxy, gl: Rc<gl::Gl>, dpr: f32) -> Self {
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
        let font_manager = FontManager::default();
        let builder = DisplayListBuilder::new(pipeline_id, layout_size);

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
            font_manager,
            builder,
        }
    }

    pub(crate) fn add_font(&mut self, font: Font, data: impl Into<Vec<u8>>) {
        self.font_manager.add_font(font, data.into(), &self.api);
    }

    pub(crate) fn remove_font(&mut self, font: &Font) {
        self.font_manager.remove_font(font, &self.api)
    }

    pub(crate) fn resize(&mut self, size: Size, dpr: f32) {
        let Size { width, height, .. } = size;
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
        self.builder = DisplayListBuilder::new(self.pipeline_id, self.layout_size);
    }

    pub(crate) fn render(&mut self) {
        let mut txn = Transaction::new();
        let mut builder = DisplayListBuilder::new(self.pipeline_id, self.layout_size);
        std::mem::swap(&mut builder, &mut self.builder);
        txn.set_display_list(Epoch(0), None, self.layout_size, builder.finalize(), true);
        txn.generate_frame();
        self.api.send_transaction(self.document_id, txn);
    }

    pub(crate) fn flush(&mut self) {
        self.renderer.update();
        self.renderer.render(self.device_size).unwrap(); // TODO: Handle possible errors
        self.renderer.flush_pipeline_info();
    }

    pub(crate) fn deinit(self) {
        self.renderer.deinit();
    }
}
