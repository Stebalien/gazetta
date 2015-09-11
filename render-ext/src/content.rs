use horrorshow::prelude::*;
use gazetta_core::render::Gazetta;
use gazetta_core::view::Page;

/// Renders a page's content
pub struct Content<'a, G>(pub &'a Page<'a, G>)
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a;

impl<'a, G> RenderOnce for Content<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl<'a, G> RenderMut for Content<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl<'a, G> Render for Content<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render(&self, tmpl: &mut TemplateBuffer) {
        match self.0.content.format {
            "mkd"|"md"|"markdown"   => tmpl << ::Markdown::new(self.0.content.data, self.0.href),
            "html"                  => tmpl << raw!(self.0.content.data),
            ""|"text"|"txt"         => tmpl << self.0.content.data,
            format                  => tmpl.record_error(format!("unknown format '{}'", format)),
        }
    }
}

