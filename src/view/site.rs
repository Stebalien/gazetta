use std::ops::Deref;
use std::fmt;

use ::render::Gazetta;
use ::model::{Person, Source};

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

impl<'a, G> From<&'a Source<G::SiteMeta, G::PageMeta>> for Site<'a, G> where G: Gazetta {
    fn from(source: &Source<G::SiteMeta, G::PageMeta>) -> Site<G> {
        Site {
            title: &source.title,
            author: &source.author,
            meta: &source.meta,
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
            .field("author", &self.author)
            .field("meta", &self.meta)
            .finish()
    }
}
