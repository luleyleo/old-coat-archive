use crate::{Font, FontSize};
use fnv::FnvHashMap as HashMap;
use webrender::api::{AddFont, AddFontInstance, FontInstanceKey, FontKey, RenderApi, ResourceUpdate};

pub(crate) struct LoadedFont {
    rusttype: rusttype::Font<'static>,
    webrender: FontKey,
    instances: HashMap<FontSize, FontInstanceKey>,
}

#[derive(Default)]
pub struct FontManager {
    fonts: HashMap<Font, LoadedFont>
}

impl FontManager {
    pub(crate) fn add_font(&mut self, font: Font, data: Vec<u8>, api: &RenderApi) {
        let key = api.generate_font_key();
        api.update_resources(vec![ResourceUpdate::AddFont(AddFont::Raw(key, data.clone(), 0))]);
        let rt_font = rusttype::Font::from_bytes(data).unwrap();
        self.fonts.insert(font, LoadedFont {
            rusttype: rt_font,
            webrender: key,
            instances: HashMap::default(),
        });
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

    pub fn load_rusttype(&self, font: &Font) -> &rusttype::Font<'static> {
        &self.fonts[font].rusttype
    }

    pub fn load_instance(&mut self, font: &Font, size: FontSize, api: &RenderApi) -> Option<FontInstanceKey> {
        if let Some(loaded_font) = self.fonts.get_mut(&font) {
            let key = loaded_font.instances.get(&size).cloned().unwrap_or_else(|| {
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
            log::error!("Tried to receive instance of no-existend {:?}", font);
            None
        }
    }
}
