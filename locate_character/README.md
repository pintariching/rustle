# locate-character

Get the line and column number of a particular character in a string.

## Usage

To search for a particular character, using the index or a search string, use `locate`:

```rust
use locate_character::{locate, Search};

const SAMPLE: &'static str = r#"A flea and a fly in a flue
Were imprisoned, so what could they do?
Said the fly, "let us flee!"
"Let us fly!" said the flea.
So they flew through a flaw in the flue."#;

fn main() {
    // Using a character index
    let _ = locate(SAMPLE, Search::Index(13), None);
    // -> Some({ line: 0, column: 13, character: 13 })

    // Using the string itself
    let _ = locate(SAMPLE, Search::Word("fly"), None);
    // -> Some({ line: 0, column: 13, character: 13 })

    // Using the string with a start index
    let _ = locate(
        SAMPLE,
        Search::Word("fly"),
        Some(Options::from_start_index(14)),
    );
    // -> Some({ line: 2, column: 9, character: 76 })
}
```

If you will be searching the same string repeatedly, it's much faster if you use `getLocator`:

```rust
use locate_character::{get_locator, Search};

const SAMPLE: &'static str = r#"A flea and a fly in a flue
Were imprisoned, so what could they do?
Said the fly, "let us flee!"
"Let us fly!" said the flea.
So they flew through a flaw in the flue."#;

fn main() {
    let mut locate = get_locator(SAMPLE);

    let mut location = locator(Search::Index(13), None);
    // -> Some({ line: 0, column: 13, character: 13 })

    location = locate(Search::Word("fly"), Some(location.unwrap().character + 1));
    // -> Some({ line: 0, column: 13, character: 13 })

    location = locate(Search::Word("fly"), Some(location.unwrap().character + 1));
    // -> Some({ line: 0, column: 13, character: 13 })
}
```

In some situations (for example, dealing with sourcemaps), you need one-based line numbers:

```rust
use locate_character::{locate, get_locator, Search};

const SAMPLE: &'static str = r#"""#;

fn main() {
    get_locator(SAMPLE, Seach::Index(0), Some(Options {
        offset_line: Some(1),
        ..Default::default()
    }));
    locate(SAMPLE, Search::Index(0), Some(Options {
        offset_line: Some(1),
        ..Default::default()
    }));
}
```

There's also an `offset_column` option which is less useful in real-world situations.


## License

MIT