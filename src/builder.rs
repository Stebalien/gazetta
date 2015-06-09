use std::io;

pub trait SiteBuilder {
    type Write: io::Write;

    fn build_page<F>(&mut self, page: &str, writer: F) -> io::Result<()>
        where F: FnOnce(&mut Self::Write) -> io::Result<()>;
}

pub struct DebugSiteBuilder(io::Stdout);

impl DebugSiteBuilder {
    pub fn new() -> DebugSiteBuilder {
        DebugSiteBuilder(io::stdout())
    }
}

impl SiteBuilder for DebugSiteBuilder {
    type Write = io::Stdout;
    fn build_page<F>(&mut self, _page: &str, writer: F) -> io::Result<()>
        where F: FnOnce(&mut Self::Write) -> io::Result<()>
    {
        (writer)(&mut self.0)
    }
}

