# The RCL Configuration Language

RCL is a domain-specific language optimized for specifying human-written data
with just enough abstraction features to avoid repetition. It is a superset
of json that extends it into a simple functional programming language that
resembles [Python][python] and [Nix][nix]. Use cases include:

 * Querying json documents, like [`jq`][jq], but with a more familiar language.
 * Generating repetitive configuration files, such as GitHub Actions workflows
   or Terraform configuration.
 * Sharing configuration between tools that do not natively share data. For
   example, import the same user account definitions into Terraform, Tailscale,
   Kubernetes, and Ansible.

RCL can be used through the `rcl` command-line tool that can export documents
to json, yaml, toml, [and more][output]. It can also be used through a native
Python module, with an interface similar to the `json` module.

RCL is a hobby project without stability promise.

[python]:  https://www.python.org/
[nix]:     https://nixos.org/manual/nix/stable/language/
[jq]:      https://jqlang.github.io/jq/manual/
[output]:  https://docs.ruuda.nl/rcl/rcl_evaluate/#-o-output-format

## Getting started

There are examples and a browser-based interactive demo at
[rcl-lang.org](https://rcl-lang.org). For more detailed information,
[the manual](https://docs.ruuda.nl/rcl/) is the best resource.
The most useful chapters to get started:

 * [Installation](https://docs.ruuda.nl/rcl/installation/)
 * [Tutorial](https://docs.ruuda.nl/rcl/tutorial/)
 * [Syntax guide](https://docs.ruuda.nl/rcl/syntax/)

You may also find the examples in the `examples` directory instructive.

## Rationale

Why another config language?

 * HCL is too ad-hoc to be suitable for any serious abstraction (`setunion` is
   variadic so it only works with a statically known number of sets; `flatten`
   recursively flattens so it can’t be typed properly and breaks generic code,
   for comprehensions can’t do nested loops, `for_each` syntax is bolted on,
   etc.)

 * Nix-the-language is great but forces the entire Nix store on you when all I
   want is to evaluate expressions.

 * Python is great but requires some boilerplate for doing the IO if you want
   to use it as a configuration language. Also the syntactic order of list
   comprehensions prevents autocomplete in editors.

 * Dhall has the right ideas but the syntax and heavy use of Unicode symbols
   make it look ugly.

 * CUE and Nickel were not invented here.

 * For more background, see the blog post:
   [_A reasonable configuration language_][blog].

[blog]: https://ruudvanasseldonk.com/2024/a-reasonable-configuration-language

## Classification

 * **Purely functional:** RCL documents are expressions that evaluate to values,
   rather than sequences of statements that have side effects. Values are
   immutable, there are no mutable objects. Functions are values.

 * **Gradually typed:** Optional type annotations can be used to prevent bugs
   and to make code more self-documenting. All type annotations are enforced.

 * **Json superset:** The RCL syntax is a superset of json. This means RCL can
   natively load json documents, you can use RCL to query json documents, and
   you can incrementally upgrade json documents to RCL.

## Usage

Build:

    cargo build --release

Print usage:

    target/release/rcl
    target/release/rcl eval --help

Evaluate an RCL expression to json:

    target/release/rcl eval --format=json examples/tags.rcl

Query an RCL or json document:

    target/release/rcl query examples/tags.rcl input.tags.ams01

Autoformat an RCL expression (non-destructive, prints to stdout):

    target/release/rcl fmt examples/tags.rcl

Highlight an RCL expression in your terminal:

    target/release/rcl highlight examples/tags.rcl

## Development

Run all tests and checks below in one command:

    nix flake check

Run golden tests:

    cargo build
    golden/run.py

Check the grammar for ambiguities:

    bison -Werror=all grammar/bison/grammar.y

Run unit tests and lints:

    cargo test
    cargo clippy

Typecheck Python sources

    mypy --strict --exclude pyrcl .
    mypy --strict pyrcl

Check formatting:

    cargo fmt
    black .

View coverage of the golden tests:

    nix build .#coverage --out-link result
    xdg-open result/index.html

For how to run the fuzzers, see [`docs/testing.md`](docs/testing.md).

## Building the Python module

Build the shared object:

    cargo build --manifest-path pyrcl/Cargo.toml

Give the shared object the appropriate name for the Python interpreter to
discover it:

    mv target/debug/{libpyrcl,rcl}.so

Tell Python where to find the shared object, run the interpreter:

    PYTHONPATH=target/debug python3
    >>> import rcl
    >>> help(rcl.loads)
    >>> rcl.load_file("examples/buckets.rcl")

## Building WASM

See [the readme in the `wasm` directory](wasm/README.md).

## License

RCL is licensed under the [Apache 2.0][apache2] license. It may be used in
free software as well as closed-source applications, both for commercial and
non-commercial use under the conditions given in the license. If you want to
use RCL in your GPLv2-licensed software, you can add an [exception][except]
to your copyright notice. Please do not open an issue if you disagree with the
choice of license.

[apache2]: https://www.apache.org/licenses/LICENSE-2.0
[except]:  https://www.gnu.org/licenses/gpl-faq.html#GPLIncompatibleLibs
