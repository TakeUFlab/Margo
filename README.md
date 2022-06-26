# Margo

A WIP Markdown-Like Lexer

# Dev

We use [just](https://github.com/casey/just) as command runner. 

## Command List

```bash
Available recipes:
    build    # build all code in workspace
    b        # alias for `build`
    check    # check all code in workspace
    c        # alias for `check`
    clean    # clean up all cache workspace
    fmt      # fmt all code in workspace
    lint     # clippy all code in workspace
    run +arg # run src
    r +arg   # alias for `run`
    test     # test all code in workspace
```

To run Margo please run `just run`, more info run `just run help`

```bash
USAGE:
    just run build [OPTIONS] <INPUT> <TARGET>

ARGS:
    <INPUT>     
    <TARGET>    [possible values: json, rust, html]

OPTIONS:
    -h, --help               Print help information
    -o, --output <OUTPUT>    
```

# Roadmap

## Todo

- Block
  - [x] Heading (`# {{text}}`,`## {{text}}`...) 
  - [x] Paragraph (...)
  - [x] Code (` ```{{lang}}\n{{text}}``` `)
  - [ ] List
- Inline
  - [x] Bold (` *{{inline}}* `)
  - [x] Italic (` /{{inline}}/ `)
  - [x] Underline (` _{{inline}}_ `)
  - [x] Strikethrough (` ~{{inline}}~ `)
  - [x] Inline Code (`` `{{text}}` ``)
  - [x] Inline Math (` ${{text}}$ `)
  - [ ] Marker
  - [ ] Emoji
  - [ ] Image
  - [ ] URL

*(You need add newline in the end of file)

*(block separatorm is two newline)

## RFC

- Bullet Journal (`[.]`,`[~]`,`[!]`,`[>]`,`[<]`) 
- Embed