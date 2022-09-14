use crate::web::ctx::PageCtx;

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("rendering error: {0}")]
    Render(#[from] handlebars::RenderError)
}

// handlebars require lifetime
pub struct Renderer<'a> (handlebars::Handlebars<'a>);

impl<'a> Renderer<'a> {
    pub fn new(template_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer.register_templates_directory(".hbs", &template_dir).expect("failed to register handlebars templates");
        Self(renderer)
    }

    // convert a serializable struct into JSON
    fn serialize<S>(serializable: &S) -> serde_json::Value
        where S: serde::Serialize + std::fmt::Debug
    {
        serde_json::to_value(&serializable).expect("failed to serialized struct into value") // should not fail as Serialize is derived almost everywhere
    }

    pub fn render<P>(&self, ctx:P, errors:&[&str]) -> String
    where P: PageCtx + serde::Serialize + std::fmt::Debug
    {
        let mut value = Self::serialize(&ctx);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), ctx.title().into());
            value.insert("_base".into(), ctx.parent().into());
        }

        self.do_render(ctx.template_path(), value)
    }

    pub fn render_with_data<P, D>(&self, ctx:P, data:(&str, D), errors:&[&str]) -> String
    where
        P: PageCtx + serde::Serialize + std::fmt::Debug,
        D: serde::Serialize + std::fmt::Debug
    {
        use handlebars::to_json;

        let mut value = Self::serialize(&ctx);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), ctx.title().into());
            value.insert("_base".into(), ctx.parent().into());
            value.insert(data.0.into(), to_json(data.1));
        }

        self.do_render(ctx.template_path(), value)
    }

    fn do_render(&self, path:&str, ctx: serde_json::Value) -> String {
        self.0.render(path, &ctx).expect("error rendering template")
    }
}
