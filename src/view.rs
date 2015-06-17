use std::ops::Deref;

use ::markdown::Markdown;
use ::model::{Date, Person};

#[derive(Copy, Clone, Debug)]
pub struct Page<'a, M> where M: ::model::Meta + 'a {
    pub title: &'a str,
    pub date: Option<&'a Date>,
    pub content: Option<Markdown<'a>>,
    pub href: &'a str,
    pub index: Option<Index<'a, M>>,
    pub meta: &'a M,
}

impl<'a, M> Deref for Page<'a, M> where M: ::model::Meta + 'a {
    type Target = M;
    fn deref(&self) -> &M {
        self.meta
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Paginate<'a> {
    /// Index of current page.
    pub current: usize,
    pub pages: &'a [&'a str],
}

#[derive(Copy, Clone, Debug)]
pub struct Index<'a, M> where M: ::model::Meta + 'a {
    pub entries: &'a [Page<'a, M>],
    pub paginate: Option<Paginate<'a>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Site<'a, M> where M: ::model::Meta + 'a {
    pub title: &'a str,
    pub author: &'a Person,
    pub meta: &'a M,
}

impl<'a, M> Deref for Site<'a, M> where M: ::model::Meta + 'a {
    type Target = M;
    fn deref(&self) -> &M {
        self.meta
    }
}

