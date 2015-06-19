use std::ops::Deref;
use std::fmt;

use ::render::Gazetta;
use ::model::Date;
use ::markdown::Markdown;
use ::horrorshow::prelude::*;

use super::Index;

/// Represents an indevidual page to be rendered.
pub struct Page<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a,
{
    /// The page's title.
    pub title: &'a str,

    /// The page's date.
    pub date: Option<&'a Date>,

    /// The page's location.
    pub href: &'a str,

    /// The index contained in this page, if any.
    pub index: Option<Index<'a, G>>,

    /// Extra metadata specified in the Entry.
    pub meta: &'a G::PageMeta,

    /// The page's content.
    ///
    /// If you want to use the default renderer, just render the page itself.
    /// 
    /// ```norun
    /// html! {
    ///     div(id="content") : page;
    /// }
    /// ```
    pub content: Content<'a>,
}

#[derive(Copy, Clone, Debug)]
pub struct Content<'a> {
    pub data: &'a str,
    pub format: &'a str,
}

// Implement these manually. Derive requires that G: Trait.

impl<'a, G> Copy for Page<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a,
{ }

impl<'a, G> Clone for Page<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, G> fmt::Debug for Page<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: fmt::Debug + 'a,
          G::SiteMeta: 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Page")
            .field("title", &self.title)
            .field("date", &self.date)
            .field("href", &self.href)
            .field("index", &self.index)
            .field("meta", &self.meta)
            .field("content", &self.content)
            .finish()
    }
}

impl<'a, G> RenderOnce for Page<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_once( self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl<'a, G> RenderMut for Page<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) {
        self.render(tmpl);
    }
}

impl<'a, G> Render for Page<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    fn render(&self, tmpl: &mut TemplateBuffer) {
        match self.content.format {
            "mkd"|"md"|"markdown"   => tmpl << Markdown::new(self.content.data, self.href),
            "html"                  => tmpl << raw!(self.content.data),
            ""|"text"|"txt"         => tmpl << self.content.data,
            format => {
                tmpl.record_error(format!("unknown format '{}'", format));
                tmpl
            }
        };
    }
}

impl<'a, G> Deref for Page<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a,
{
    type Target = G::PageMeta;
    fn deref(&self) -> &Self::Target {
        self.meta
    }
}

