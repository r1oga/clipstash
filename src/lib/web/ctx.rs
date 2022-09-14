use serde::Serialize;
use derive_more::Constructor;

pub trait PageCtx {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {}
impl Home {
    fn default() -> Self{Self{}}
}

impl PageCtx for Home {
    fn title(&self) -> &str { "Stash Your Clipboard!" }
    fn template_path(&self) -> &str { "home" }
    fn parent(&self) ->&str { "base" }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::Clip
}

impl PageCtx for ViewClip {
    fn title(&self) -> &str { "View Clip" }
    fn template_path(&self) -> &str { "clip" }
    fn parent(&self) ->&str { "base" }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired{
    shortcode: crate::ShortCode
}

impl PageCtx for PasswordRequired {
    fn title(&self) -> &str { "Password Required" }
    fn template_path(&self) -> &str { "clip_need_password" }
    fn parent(&self) ->&str { "base" }
}
