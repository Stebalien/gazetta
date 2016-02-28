initSidebarItems({"fn":[["unidecode","This function takes any Unicode string and returns an ASCII transliteration of that string.GuaranteesThe `String` returned will be valid ASCII; the decimal representation of every `char` in the string will be between 0 and 127, inclusive. Every ASCII character (0x0000 - 0x007F) is mapped to itself. All Unicode characters will translate to a string containing newlines (`\"\\n\"`) or ASCII characters in the range 0x0020 - 0x007E. So for example, no Unicode character will translate to `\\u{01}`. The exception is if the ASCII character itself is passed in, in which case it will be mapped to itself. (So `'\\u{01}'` will be mapped to `\"\\u{01}\"`.) WarningsAs stated, some transliterations do produce `\\n` characters. Some Unicode characters transliterate to an empty string, either on purpose or because `rust-unidecode` does not know about the character. Some Unicode characters are unknown and transliterate to `\"[?]\"`. Many Unicode characters transliterate to multi-character strings. For example, 北 is transliterated as \"Bei \". These guarantees/warnings are paraphrased from the original `Text::Unidecode` documentation."],["unidecode_char","This function takes a single Unicode character and returns an ASCII transliteration.The warnings and guarantees of `unidecode()` apply to this function as well.Examples"]]});