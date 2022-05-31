# Margo

A WIP Markdown-Like Lexer


PLS Read [syntax design](./SYNTAX.md)

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