# rusty-fizzbuzz
YAFBI - Yet another FizzBuzz implementation. This time in [Rust](https://www.rust-lang.org/)!

I'm going to use this as an initial examples of some ideas of mine. 

Dependencies
============

[Quickcheck](https://github.com/BurntSushi/quickcheck)

> Haven't added this to the program yet.

Some helpful pages:
===================

[The Rust Guide to Strings](http://smallcultfollowing.com/rust-int-variations/imem-umem/guide-strings.html)

> This came in handy when I was working on a function that returned a string. 
> There are actually two kinds of strings in Rust. Fancy that.


[Understanding Pointers, Ownership and Lifetimes in Rust](http://paulkoerbitz.de/posts/Understanding-Pointers-Ownership-and-Lifetimes-in-Rust.html)

> This was useful when I was trying to create a larger sample of integers to assert against, such as:

```rust
let fizzies = [3, 6, 9, 12, 300, 60];
  for x in fizzies.iter() {
  assert!(is_fizz(*x));
}
```

Versions
========

v.0.0.1 - Initial binary version.

