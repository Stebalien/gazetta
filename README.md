Gazetta is a static site generator written in rust. There are four parts:

1. [Horrorshow][horrorshow], a rust-macro based HTML generator.
2. The core framework (this repository). This library contains the bulk of the
   logic and is responsible for generating models and views from your site's
   data files.
3. The [renderer][bin]. This is where the logic to actually render static site
   views into html lives. If you need fine control over your website's html, you
   should fork this repo. However, 99% of the time, you should be able to
   sufficiently customize your site without modifying this project.
4. Your website data. This includes both your website's content and assets
   (stylesheets/javascript). You can find mine [here][data] and a relatively
   clean repository [here][bootstrap].

# Quick Start

1. Download and compile [gazetta-bin][bin] (`cargo build --release`).
2. Fork and clone [gazetta-bootstrap][bootstrap].
3. Edit the config, homepage, theme to your liking.
4. Create some new pages either manually or using the gazetta binary. For
   example: `gazetta new blog "Hello World"` will create a hello world blog
   post.
5. Run `gazetta render /path/to/output` (in the root of the repository) to
   render your website.


[framework]: https://github.com/Stebalien/gazetta
[bin]: https://github.com/Stebalien/gazetta
[data]: https://github.com/Stebalien/www
[horrorshow]: https://github.com/Stebalien/horrorshow-rs
[bootstrap]: https://github.com/Stebalien/gazetta-bootstrap
