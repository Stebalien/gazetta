use std::ops::Deref;
use std::fmt;

use ::horrorshow::prelude::*;
use ::render::Gazetta;
use ::model::Source;


/// A "website"
///
/// You should include this view in your websites "head". It renders into script, stylesheet, icon
/// tags metadata tags.
///
/// ```norun
/// html! {
///     html {
///         head {
///             // ...
///             : site;
///         }
///     }
/// }
/// ```
pub struct Site<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
{
    /// The website's title
    pub title: &'a str,
    /// The concatinated stylesheets.
    pub stylesheets: Option<&'a str>,
    /// The concatinated javascript.
    pub javascript: Option<&'a str>,
    /// The icon.
    pub icon: Option<&'a str>,
    /// Extra metadata specified in the Source.
    pub meta: &'a G::SiteMeta,
}

impl<'a, G> RenderOnce for Site<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
{
    fn render_once(self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a, G> RenderMut for Site<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a, G> Render for Site<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
{
    fn render(&self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            meta(charset="utf-8");
            @ if let Some(css) = self.stylesheets {
                link(rel="stylesheet", href=css);
            }
            @ if let Some(js) = self.javascript {
                script(src=js) {}
            }
            @ if let Some(icon) = self.icon {
                link(rel="shortcut icon", href=icon);
            }
        };
    }
}


impl<'a, G> From<&'a Source<G::SiteMeta, G::PageMeta>> for Site<'a, G> where G: Gazetta {
    fn from(source: &Source<G::SiteMeta, G::PageMeta>) -> Site<G> {
        Site {
            title: &source.title,
            meta: &source.meta,
            javascript: if !source.javascript.is_empty() {
                Some("assets/javascript.js")
            } else { None },
            stylesheets: if !source.stylesheets.is_empty() {
                Some("assets/stylesheets.css")
            } else { None },
            icon: if source.icon.is_some() {
                Some("assets/icon.png")
            } else { None },
        }
    }
}

impl<'a, G> Deref for Site<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
{
    type Target = G::SiteMeta;
    fn deref(&self) -> &Self::Target {
        self.meta
    }
}

impl<'a, G> Copy for Site<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a
{}

impl<'a, G> Clone for Site<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

// Manually implement because rust isn't correctly adding the Debug constraint when deriving.
impl<'a, G> fmt::Debug for Site<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Page")
            .field("title", &self.title)
            .field("stylesheets", &self.stylesheets)
            .field("javascript", &self.javascript)
            .field("icon", &self.icon)
            .field("meta", &self.meta)
            .finish()
    }
}
