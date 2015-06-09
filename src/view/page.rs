use ::Date;

#[derive(Copy, Clone, Debug)]
pub struct Page<'a> {
    pub title: &'a str,
    pub date: Option<&'a Date>,
    pub content: Option<&'a str>,
    pub href: &'a str,
    pub index: Option<Index<'a>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Paginate<'a> {
    pub prev: Option<(u32, &'a str)>,
    pub next: Option<(u32, &'a str)>,
    pub page_number: u32,
    pub total: u32,
}

#[derive(Copy, Clone, Debug)]
pub struct Index<'a> {
    pub entries: &'a [Page<'a>],
    pub paginate: Option<Paginate<'a>>,
}
/*

fn main() {
    
}

impl Read for Content {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

impl Page {
    pub fn get_content(&self) -> io::Result<Content> {
        let mut f = try!(File::open(self.content_dir.join("index.md")));
        if (self.content_offset as u64) != try!(f.seek(io::SeekFrom::Start(self.content_offset as u64))) {
            return Err(io::Error::new(io::ErrorKind::Other, "content missing"));
        }
        Ok(Content(f))
    }
}
*/
