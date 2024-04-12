# rust-jq

CLI tool for processing JSON. Currently only supports validation of the JSON.

## Overall structure and how it works

- Input file is read.
- Input is tokenized and collected.

```rs
pub enum Token {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}
```

- List of tokens are parsed into AST.
- If the parsing was successfull, we can confidently say that JSON is valid.

## Clone

```

```

## Build

```
cargo build --release
```

## Test

End-to-end tests are in `tests/` folder, unit tests are in their respective modules and files.

```
cargo test
```

## Run

You can find example jsons that are run as integration tests in `tests/testdata` folder if you'd like to do quick runs.

```
./target/release/rust-jq --file <path_to_json_file>
```
