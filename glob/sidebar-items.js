initSidebarItems({"fn":[["glob","Return an iterator that produces all the Paths that match the given pattern, which may be absolute or relative to the current working directory.This may return an error if the pattern is invalid.This method uses the default match options and is equivalent to calling `glob_with(pattern, MatchOptions::new())`. Use `glob_with` directly if you want to use non-default match options.When iterating, each result is a `GlobResult` which expresses the possibility that there was an `IoError` when attempting to read the contents of the matched path.  In other words, each item returned by the iterator will either be an `Ok(Path)` if the path matched, or an `Err(GlobError)` if the path (partially) matched _but_ its contents could not be read in order to determine if its contents matched.See the `Paths` documentation for more information.ExampleConsider a directory `/media/pictures` containing only the files `kittens.jpg`, `puppies.jpg` and `hamsters.gif`:The above code will print:If you want to ignore unreadable paths, you can use something like `filter_map`:"],["glob_with","Return an iterator that produces all the Paths that match the given pattern, which may be absolute or relative to the current working directory.This may return an error if the pattern is invalid.This function accepts Unix shell style patterns as described by `Pattern::new(..)`.  The options given are passed through unchanged to `Pattern::matches_with(..)` with the exception that `require_literal_separator` is always set to `true` regardless of the value passed to this function.Paths are yielded in alphabetical order."]],"struct":[["GlobError","A glob iteration error.This is typically returned when a particular path cannot be read to determine if its contents match the glob pattern. This is possible if the program lacks the permissions, for example."],["MatchOptions","Configuration options to modify the behaviour of `Pattern::matches_with(..)`"],["Paths","An iterator that yields `Path`s from the filesystem that match a particular pattern.Note that it yields `GlobResult` in order to report any `IoErrors` that may arise during iteration. If a directory matches but is unreadable, thereby preventing its contents from being checked for matches, a `GlobError` is returned to express this.See the `glob` function for more details."],["Pattern","A compiled Unix shell style pattern.`?` matches any single character`*` matches any (possibly empty) sequence of characters`**` matches the current directory and arbitrary subdirectories. This sequence **must** form a single path component, so both `**a` and `b**` are invalid and will result in an error.  A sequence of more than two consecutive `*` characters is also invalid.`[...]` matches any character inside the brackets. Character sequences can also specify ranges of characters, as ordered by Unicode, so e.g. `[0-9]` specifies any character between 0 and 9 inclusive. An unclosed bracket is invalid.`[!...]` is the negation of `[...]`, i.e. it matches any characters **not** in the brackets.The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets (e.g. `[?]`).  When a `]` occurs immediately following `[` or `[!` then it is interpreted as being part of, rather then ending, the character set, so `]` and NOT `]` can be matched by `[]]` and `[!]]` respectively. The `-` character can be specified inside a character sequence pattern by placing it at the start or the end, e.g. `[abc-]`."],["PatternError","A pattern parsing error."]],"type":[["GlobResult","An alias for a glob iteration result.This represents either a matched path or a glob iteration error, such as failing to read a particular directory's contents."]]});