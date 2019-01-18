use crate::{Font, FontSize, Position, Size};
use crate::backend::winit::DEFAULT_FONT_NAME;
use fnv::FnvHashMap as HashMap;
use webrender::api::{
    AddFont, AddFontInstance, FontInstanceKey, FontKey, RenderApi, ResourceUpdate, GlyphInstance, LayoutPoint
};

pub(crate) struct LoadedFont {
    rusttype: rusttype::Font<'static>,
    webrender: FontKey,
    instances: HashMap<FontSize, FontInstanceKey>,
}

pub struct FontManager {
    fonts: HashMap<Font, LoadedFont>,
    glyph_cash: Vec<GlyphInstance>,
    default_font: Font,
}

impl Default for FontManager {
    fn default() -> Self {
        FontManager {
            fonts: HashMap::default(),
            glyph_cash: Vec::default(),
            default_font: Font::from_family(DEFAULT_FONT_NAME),
        }
    }
}

impl FontManager {
    pub(crate) fn add_font(&mut self, font: Font, data: Vec<u8>, api: &RenderApi) {
        let key = api.generate_font_key();
        api.update_resources(vec![ResourceUpdate::AddFont(AddFont::Raw(key, data.clone(), 0))]);
        let rt_font = rusttype::Font::from_bytes(data).unwrap();
        self.fonts.insert(
            font,
            LoadedFont {
                rusttype: rt_font,
                webrender: key,
                instances: HashMap::default(),
            },
        );
    }

    pub(crate) fn remove_font(&mut self, font: Font, api: &RenderApi) {
        let font = self.fonts.remove(&font).unwrap();
        let mut updates = Vec::default();
        for instance in font.instances.values() {
            updates.push(ResourceUpdate::DeleteFontInstance(*instance));
        }
        updates.push(ResourceUpdate::DeleteFont(font.webrender));
        api.update_resources(updates);
    }

    pub fn default_font(&self) -> &Font {
        &self.default_font
    }

    pub fn rusttype(&self, font: &Font) -> &rusttype::Font<'static> {
        &self.fonts[font].rusttype
    }

    pub fn instance(&mut self, font: &Font, size: FontSize, api: &RenderApi) -> Option<FontInstanceKey> {
        if let Some(loaded_font) = self.fonts.get_mut(&font) {
            let key = loaded_font
                .instances
                .get(&size)
                .cloned()
                .unwrap_or_else(|| {
                    let key = api.generate_font_instance_key();
                    api.update_resources(vec![ResourceUpdate::AddFontInstance(AddFontInstance {
                        key,
                        font_key: loaded_font.webrender,
                        glyph_size: app_units::Au::from_px(size),
                        options: None,
                        platform_options: None,
                        variations: Vec::new(),
                    })]);
                    loaded_font.instances.insert(size, key);
                    key
                });
            Some(key)
        } else {
            log::error!("Tried to receive instance of unknown {:?}", font);
            None
        }
    }

    pub fn dimensions(&self, text: &str, font: &Font, size: FontSize) -> Size {
        let mut dimensions = Size::zero();
        let size = size as f32;
        dimensions.height = size;
        let scale = rusttype::Scale {
            // TODO: Fix glyph overlapping without additional x-scaling
            // The current value roughly fits OpenSans
            x: size * 1.2,
            y: size,
        };
        let point = rusttype::Point { x: 0.0, y: 0.0 };
        let font = &self.fonts[font].rusttype;
        font.layout(text, scale, point)
            .last()
            .map(|glyph| {
                let pos = glyph.position();
                let hmet = glyph.unpositioned().h_metrics();
                dimensions.width = pos.x + hmet.advance_width;
            });
        return dimensions;
    }

    pub fn layout<'a>(
        &mut self,
        text: &str,
        font: &Font,
        size: FontSize,
        position: Position,
    ) -> &[webrender::api::GlyphInstance] {
        let size = size as f32;
        let scale = rusttype::Scale {
            // TODO: Fix glyph overlapping without additional x-scaling
            // The current value roughly fits OpenSans
            x: size * 1.2,
            y: size,
        };
        let point = rusttype::Point {
            x: position.x,
            y: position.y,
        };
        let font = &self.fonts[font].rusttype;
        let glyphs = font.layout(text, scale, point)
            .map(|glyph| {
                let index = glyph.id().0;
                let pos = glyph.position();
                let point = LayoutPoint::new(pos.x, pos.y + size);
                GlyphInstance { index, point }
            });
        self.glyph_cash.clear();
        self.glyph_cash.extend(glyphs);
        return self.glyph_cash.as_slice();
    }
}
