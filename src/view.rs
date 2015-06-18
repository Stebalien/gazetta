use std::ops::Deref;
use std::fmt;

use ::markdown::Markdown;
use ::horrorshow::prelude::*;
use ::model::{Date, Person};
use ::Gazetta;

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


#[derive(Copy, Clone, Debug)]
pub struct Content<'a> {
    pub data: &'a str,
    pub format: &'a str,
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

/// Page pagination information.
#[derive(Copy, Clone, Debug)]
pub struct Paginate<'a> {
    /// Index of current page.
    pub current: usize,
    /// The list of pages (links only) in this pagination.
    pub pages: &'a [&'a str],
}

/// Page index information
pub struct Index<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
          G::PageMeta: 'a
{
    /// Pages to be indexed.
    pub entries: &'a [Page<'a, G>],
    /// Pagination information (if any).
    pub paginate: Option<Paginate<'a>>,
}

// Implement these manually. Derive requires that G: Trait.

impl<'a, G> Copy for Index<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a
{}

impl<'a, G> Clone for Index<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: 'a,
          G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

// Manually implement because rust isn't correctly adding the Debug constraint when deriving.
impl<'a, G> fmt::Debug for Index<'a, G>
    where G: Gazetta + 'a,
          G::PageMeta: fmt::Debug + 'a,
          G::SiteMeta: 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Index")
            .field("entries", &self.entries)
            .field("paginate", &self.paginate)
            .finish()
    }
}

/// A "website"
pub struct Site<'a, G>
    where G: Gazetta + 'a,
          G::SiteMeta: 'a,
{
    /// The website's title
    pub title: &'a str,
    /// The website's author
    pub author: &'a Person,
    /// Extra metadata specified in the Source.
    pub meta: &'a G::SiteMeta,
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
            .field("author", &self.author)
            .field("meta", &self.meta)
            .finish()
    }
}
