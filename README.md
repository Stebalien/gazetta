Gazetta is a static site generator written in rust. There are four parts:

1. [Horrorshow][horrorshow], a rust-macro based HTML generator.
2. The core [framework][framework]. This library contains the bulk of the
   logic and is responsible for generating models and views from your site's
   data files.
3. The [renderer][bin]. This is where the logic to actually render static site
   views into html lives. If you need fine control over your website's html, you
   should fork this repo. However, 99% of the time, you should be able to
   sufficiently customize your site without modifying this project.
4. Your website data. This includes both your website's content and assets
   (stylesheets/javascript). You can find mine [here][data] and a relatively
   clean repository [here][bootstrap].

## Platforms

I've only tested Gazetta on 64bit Linux but it should work on all *nix
platforms. However, it probably won't on windows (I assume forward slashes in
paths). Patches welcome!

## Code quality

I'm happy with the overall architecture but some of the code could use a little
love (the main render function is especially atrocious). Again, patches welcome.

## Performance

Gazetta is pretty damn fast; there's a reason I don't bother displaying
progress. However, there is room for improvement:

1. Gazetta makes a lot of system calls (file IO). We could probably reduce this
   significantly. On my machine, more then half the runtime is system time.
2. yaml-rust is slow (according to callgrind).
3. The index building algorithm is `Θ(num_indices*num_entries)`. This could be
   reduced to `Θ(num_entries)` however, `num_indices` is usually rather low so
   this only makes sense if we can keep the constant multiplier in the new
   algorithm's runtime low.
4. Do we even care? My website (~75 pages) builds in 50ms on my laptop.

## Quick Start

1. Download and compile [gazetta-bin][bin] (`cargo build --release`). Build in
   release mode unless you want to pay a 15x-30x performance penalty.
2. Fork and clone [gazetta-bootstrap][bootstrap].
3. Edit the config, homepage, and theme to your liking.
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
