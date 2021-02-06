# rust-lang-async-book

Rust Language - Async Book

## Introduction

Rust programming language is quickly evolving to become a powerful tool for new and veteran systems programmers.

This is due to clean abstractions, excellent documentation, sensible error handling, wide-range of applications, rapidly maturing crates community, and extreme performance while maintaining safety.

Software can quickly reach performance limitations executing on a single thread. Today's systems are powered by many cores across many machines as members of clusters around the globe. In order to take advantage of processing resources problems need to be solved using sub-problems in parallel execution. In addition, the same problems need to be solved concurrently with varying arguments.

Rust provides abstractions to perform standard OS multi-threading; however, threads can be expensive and difficult to maintain. Modern languages require modern solutions and parallel/concurrent programming has rightfully become a first-class citizen today's stack. Rust's solution, async.

At this time, async is still very much in development and requires support for the open source community to power Rust applications.

## Requirements

- rustc 1.45.2 (d3fb005a3 2020-07-31)
- mdbook v0.4.6 or greater (for convenience)
- mdbook-linkcheck 0.7.2

## Workspace

This project is dependent on the cargo build system and leverages workspaces to package chapter work respectively.  

Not every chapter in the Rust Async Book has transferable source code but for the chapters that do a package is included in the root directory of this repository.

## Structure

A Rust Async Book chapter package has been created in this repository workspace for chapters including source code examples.

Path Convention: `rust-lang-async-book/<chapter-uri-path>`

The Rust Language Async Book itself is included as a GIT submodule. This tethers chapter work to a specific version of the book.

*Hint: use mdbook for stable references even if outdated*

## References

- [Rust Async Book](https://doc.rust-lang.org/async-book/)
- [Rust Standard Library](https://doc.rust-lang.org/std/)

## License

[MIT](LICENSE)