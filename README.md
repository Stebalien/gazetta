Gazetta is a static site generator written in rust. There are four parts:

1. [Horrorshow][horrorshow], a rust-macro based HTML generator.
2. The [framework][framework]. This set of libraries contains the bulk of the
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

## Directory Layout and File Format

The general data flow is:

```text
Input Files → Models → Views → Output HTML
```

### Input Files

#### Config

file: `config.yaml`

This is the website's core config. It can be used to specify shared variables
available when rendering any page on the site. It must specify:

* title: the website's title.
* base: the website's base url.

If you're using the default renderer ([gazetta-bin][bin]), you must also specify
an `author` in the format:

```text
author:
  (mandatory) name: My Name
  (opt) email: email
  (opt) photo: photo
  (opt) key: pgp_key_url
  (opt) nicknames: author nicknames
  (opt) also: 
     - "GitHub": github url (example)
     - "reddit": reddit profile url (example)
```

And may specify a set of navigational links:

```text
nav:
  - Blog: blog
  - Projects: projects
  - Github: http://github.com/Bitdiddle
```

#### Assets

files:

```text
assets/
├── javascript/
│   ├── 0-example1.js
│   └── 1-example2.js
├── stylesheets/
│   ├── 0-example1.css
│   └── 1-example2.css
└── icon.png
```

All files are optional.

* `icon.png`: The website's icon.
* `javascript`: The website's javascript files. They will be concatenated
  in lexicographical on build.
* `stylesheets`: The website's stylesheets. They will be concatenated in
  lexicographical on build.

All other files in `assets` will be ignored.

#### Entries

An entry consists of:

1. A mandatory index file (`index.txt`, `index.html`, or `index.md`).
2. An optional `static` directory containing static files.
3. Any number of child entries.

All index files *must* have a yaml header. This header must include a `title`
and anything else required by your specify renderer. By default, the header may
also include:

* date: A date in the format `YYYY-MM-DD`.
* index: This indicates that the current page is an index. If specified, all
  children will be appended to the page as sub pages. See indexing for more
  info.
* cc: Link this page into the specified index page as if it were a child. This
  is useful for making pages show up in multiple tags/categories.

If you're using the default renderer, the header may also include:

* author: Author information in the same format as the site author.
* about: A person description (in the same format as the site author). This
  description will be rendered at the top of the page. This is useful for
  profile pages. TODO: I'd like to generalize this. It feels a bit hacky.

Directories and files inside the static directory will be copied as-is to the
output directory. This is a good place to put per-page static media. 

#### Indexing

*TODO*

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


[framework]: https://github.com/Stebalien/gazetta
[bin]: https://github.com/Stebalien/gazetta-bin
[data]: https://github.com/Stebalien/www
[horrorshow]: https://github.com/Stebalien/horrorshow-rs
[bootstrap]: https://github.com/Stebalien/gazetta-bootstrap
