//! Provides `struct`s and `trait`s that are the same for any back end
//! as well as back end specific code.
//!
//! Independent of the back end it should export `Renderer`

pub mod webrender;
pub mod winit;

/// Contains all data and functions required to render something with the chosen back end
pub type Renderer = self::webrender::Webrenderer;
