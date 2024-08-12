# Datrie reimplementation in Rust

datrie is a [libdatrie](https://linux.thai.net/~thep/datrie/datrie.html) ported with C2Rust and rewritten into safe Rust.

This project is intended to be a drop in replacement for original libdatrie, except for unexported functions. It has
been tested with original libdatrie's test.

Using this library in Rust is possible, but in many places it expects and returns null terminated arrays.
There's `as_alphachar` and `ac_to_string` adapter provided for &str/String respectively, so at least it should be able
to store strings. There's no proper reexport yet and things may move.

Data types stored in Trie should implement the following traits:

* Default
* Clone
* TrieSerializable if you want to save from file
* TrieDeserializable if you want to load from file

All these are supported out of the box for i32, Vec<u8> and `Option<T>` of any supporting types.

## Feature flags

* cffi: Enable building of C binding (default on)
* ctest: Enable running C compatibility tests. These are the test from original libdatrie copied almost verbatim
  to ensure that our trie.h is backwards compatible
* bin: Enable building of trietool utility

## On speed

It is currently unclear whether this library is faster or slower than the original library:

**Pros**

* All data structure are stack-allocated, reducing memory allocation. Of course, this excludes arrays and types passing
  through the FFI boundary.
* With generic and inlining, Rust can generate more optimal code than C, which might not inline exported
  function (even if exported internally) - at least on how it was used to compile original libdatrie.

**Cons**

* All array size are computed in advance (require a loop) while the C version sometimes do not check array size
* All array access are bound checked which create overhead
* Some memory access in C use uninitialized memory, but only if the user call things in the wrong order. In the Rust
  version all data are either initialized (adding initialization cost) or behind `Option<>` (adding additional checks).

## License
As a derivative work of libdatrie, this package is licensed under the [LGPL 2.1](LICENSE) license.
