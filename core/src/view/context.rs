use std::fmt::{self, Display};

use horrorshow::{Render, RenderMut, RenderOnce};

use crate::render::Gazetta;

use super::{Page, Site};

pub struct Context<'a, G>
where
    G: Gazetta,
{
    pub page: &'a Page<'a, G>,
    pub site: &'a Site<'a, G>,
}

impl<'a, G> Context<'a, G>
where
    G: Gazetta,
{
    pub fn canonical_url(&self) -> CanonicalUrl<'a> {
        CanonicalUrl {
            origin: self.site.origin,
            prefix: self.site.prefix,
            href: self.page.href,
        }
    }
}

impl<'a, G> Copy for Context<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
}

impl<'a, G> Clone for Context<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: 'a,
    G::SiteMeta: 'a,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, G> fmt::Debug for Context<'a, G>
where
    G: Gazetta + 'a,
    G::PageMeta: fmt::Debug + 'a,
    G::SiteMeta: fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Context")
            .field("site", &self.site)
            .field("page", &self.page)
            .finish()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CanonicalUrl<'a> {
    origin: &'a str,
    prefix: &'a str,
    href: &'a str,
}

impl<'a> Display for CanonicalUrl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.origin)?;
        f.write_str(self.prefix)?;
        f.write_str(self.href)?;

        // Find the first non-empty component.
        let terminal = [&self.href, &self.prefix, &self.origin]
            .into_iter()
            .find(|h| !h.is_empty());

        // And add a slash if the last non-empty component doesn't end with a slash.
        if terminal.is_none_or(|t| !t.ends_with("/")) {
            f.write_str("/")?;
        }

        Ok(())
    }
}

impl Render for CanonicalUrl<'_> {
    fn render(&self, tmpl: &mut horrorshow::TemplateBuffer<'_>) {
        tmpl.write_fmt(format_args!("{}", self))
    }
}

impl RenderMut for CanonicalUrl<'_> {
    fn render_mut(&mut self, tmpl: &mut horrorshow::TemplateBuffer<'_>) {
        self.render(tmpl)
    }
}

impl RenderOnce for CanonicalUrl<'_> {
    fn render_once(self, tmpl: &mut horrorshow::TemplateBuffer<'_>)
    where
        Self: Sized,
    {
        self.render(tmpl)
    }

    fn size_hint(&self) -> usize {
        self.origin.len() + self.prefix.len() + self.href.len() + 1
    }
}
