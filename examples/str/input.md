There are two types of strings in Rust: `String` and `&str`.

A `String` is stored as a collection of bytes (`Vec<u8>`), but guaranteed to
always be a valid UTF-8 sequence. `String` is heap allocated, growable and not
null terminated.

`&str` is a slice (`&[u8]`) that always points to a valid UTF-8 sequence, and
can be used to view into a `String`, just like `&[T]` is a view into `Vec<T>`.

{str.rs}

More `str`/`String` methods can be found under the
[std::str](http://static.rust-lang.org/doc/master/std/str/index.html) and
[std::strbuf](http://static.rust-lang.org/doc/master/std/string/struct.String.html)
modules
