use crate::{FontId, FontSize};
use fnv::FnvHashMap as HashMap;
use webrender::api::{AddFont, AddFontInstance, FontInstanceKey, FontKey, RenderApi, ResourceUpdate, Transaction};

struct QueuedFont {
    id: FontId,
    data: Vec<u8>,
}

pub(crate) struct Font {
    rusttype: rusttype::Font<'static>,
    webrender: FontKey,
    instances: HashMap<FontSize, FontInstanceKey>,
}

#[derive(Default)]
pub struct FontManager {
    fonts: HashMap<FontId, Font>
}

impl FontManager {
    pub(crate) fn add_font(&mut self, fid: FontId, data: Vec<u8>, api: &RenderApi) {
        let key = api.generate_font_key();
        api.update_resources(vec![ResourceUpdate::AddFont(AddFont::Raw(key, data.clone(), 0))]);
        let font = rusttype::Font::from_bytes(data).unwrap();
        self.fonts.insert(fid, Font {
            rusttype: font,
            webrender: key,
            instances: HashMap::default(),
        });
    }

    pub(crate) fn remove_font(&mut self, fid: FontId, api: &RenderApi) {
        let font = self.fonts.remove(&fid).unwrap();
        let mut updates = Vec::default();
        for instance in font.instances.values() {
            updates.push(ResourceUpdate::DeleteFontInstance(*instance));
        }
        updates.push(ResourceUpdate::DeleteFont(font.webrender));
        api.update_resources(updates);
    }

    pub fn load_rusttype(&self, fid: FontId) -> &rusttype::Font<'static> {
        &self.fonts[&fid].rusttype
    }

    pub fn load_instance(&mut self, fid: FontId, size: FontSize, api: &RenderApi) -> Option<FontInstanceKey> {
        if let Some(font) = self.fonts.get_mut(&fid) {
            let key = font.instances.get(&size).cloned().unwrap_or_else(|| {
                let key = api.generate_font_instance_key();
                api.update_resources(vec![ResourceUpdate::AddFontInstance(AddFontInstance {
                    key,
                    font_key: font.webrender,
                    glyph_size: app_units::Au::from_px(size),
                    options: None,
                    platform_options: None,
                    variations: Vec::new(),
                })]);
                font.instances.insert(size, key);
                key
            });
            Some(key)
        } else {
            log::error!("Tried to receive instance of no-existend {:?}", fid);
            None
        }
    }
}
