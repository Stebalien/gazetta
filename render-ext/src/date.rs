use horrorshow::prelude::*;
use gazetta_core::prelude::*;
use gazetta_core::model::Date as DateModel;

pub struct Date<'a>(pub &'a DateModel);

impl<'a> RenderOnce for Date<'a> {
    fn render_once(self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a> RenderMut for Date<'a> {
    fn render_mut(&mut self, tmpl: &mut TemplateBuffer) { self.render(tmpl) }
}

impl<'a> Render for Date<'a> {
    fn render(&self, tmpl: &mut TemplateBuffer) { 
        tmpl << html! {
            time(datetime=format_args!("{:04}-{:02}-{:02}",
                                       self.0.year(),
                                       self.0.month(),
                                       self.0.day())
                ) : format_args!("{:04}-{:02}-{:02}",
                                 self.0.year(),
                                 self.0.month(),
                                 self.0.day())
        }
    }
}

