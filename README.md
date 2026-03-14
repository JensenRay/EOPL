# EOPL Notes

This repository is a personal record of studying *Essentials of Programming Languages* (EOPL) in Rust.

The code is incomplete and should be read as a set of learning exercises rather than a finished implementation. The goal is to keep small interpreters, experiments, and chapter-by-chapter notes in one place.

## How the Current Interpreter Works

- `main.rs`: command-line entry point. It reads the input program and prints the final evaluation result.
- `lib.rs`: the glue layer of the interpreter. It exposes the public functions that run the full pipeline, such as reading a file, tokenizing, parsing, and evaluating.
- `token.rs`: token definitions used by the lexer and parser. These are the intermediate symbols between plain text and the AST.
- `lexer.rs`: lexical analysis. It splits source text into token pieces and converts them into typed `Token` values such as numbers, identifiers, keywords, and punctuation.
- `parser.rs`: syntax analysis. It reads the token stream and constructs the AST. This is where forms like `let`, `if`, arithmetic expressions, `proc`, function calls, and `letrec` are recognized.
- `ast.rs`: AST definitions. It describes the tree-shaped structure produced by the parser, using the `Expr` enum and related expression variants.
- `interpreter.rs`: evaluator/runtime. It walks the AST, manages environments, handles closures, and evaluates expressions into runtime values such as numbers, booleans, and procedures.

In short:

`source text -> tokens -> AST -> value`

## Notes

- The implementation is still evolving.
- The project is meant for learning and experimentation, not for completeness or stability.
