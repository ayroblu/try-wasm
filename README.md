Try WebAssembly
===============

Trying out the web assembly integrations

Ideas
-----

### Rust vs WASM bindgen

Make a rust lang only application which performs a thing (I'm thinking traveling salesman problem).
Check how that performs both natively and in wasm

Doc to follow: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

#### TSP Algorithm

- Problem generator
  - Randomly generate points on a 2D space, list of possible paths then becomes a complete graph with all the points, i.e. an NxM table

### Cross platform flexibility, with strings

Implement fzf's algo.go in rust and/or AssemblyScript, then try use it both in the browser + in node.

### WASI and native rust

Implement a full fzf clone but in rust + WASI
See reference: https://www.secondstate.io/articles/wasi-access-system-resources/
