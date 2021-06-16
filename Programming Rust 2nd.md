### 2. A Tour Of Rust

- if a function body ends with an expression which is not followed by a semicolon, that's a function return value
  ```rust
  fn demo() -> i32 { 1 }
  ```
- block surrounded by curly braces can function as an expression and last expression without semicolon can be treated as its value
  ```rust
  let x = { 1 };
  ```

### Fundamental Types

- soundness type system
  - type system is sound implies all of type-checked programs are correct (i.e. no [false negative](https://en.wikipedia.org/wiki/False_positives_and_false_negatives#False_negative_error)).
  - type system is complete implies all of correct programs can be accepted by type-checked (i.e. no [false positive](https://en.wikipedia.org/wiki/False_positives_and_false_negatives#False_positive_error)).
  - [soundness vs completeness](https://en.wikipedia.org/wiki/Soundness#Relation_to_completeness): a soundness means a program passed type check is valid (run without any errors) while completeness means a vaid program can be type checked.
- `isize/usize` has the same size of an address on the machine (32 or 64 bits).
- `char` is unicode character, 32 bits wide.
- if an integer literal lacks a type suffix, its type is determinated when there is a clue (stored in a variable of a particule type, passed to a function, or return to a function ...). If there are multiple possiblitie, i32 will be picked (same for f64).
  ```rust
  println!("{}", (-4).abs());
  ```
  doesn't compile since Rust doesnt know which integer type a value belongs to, default i32 applies only if the type is still ambiguous after all method calls have been resolved which is too late here.
- integer downcast from N to M bits (N >> M) is doing the truncation of N - M most significant bits.
- signed integer overflow is undefined in C/C++, Rust introduces _Checked, Wrapping, Saturating and Overflowing_ to handle in the way you want. By default, doing that will cause panic in debug build and modulo wrap in release build.
- function returns no value has type of `()` (unit type)
  ```rust
  fn foo(x: i32, y: i32);
  ==
  fn foo(x: i32, y: i32) -> ();
  ```
- Rust permits an extra trailling comma in function arguments, arrays, struct, enum definition ... and it has no meaning
  ```rust
  (&str, i32, ) == (&str, i32)
  ```
- Rust borrow rules: you either have any number of immutable references or only one mutable reference at a given time
- Rust has no notation for an uninitialized object (array, tuple ...) to prevent junk values
- Slices `&[T]` are always passed by reference and are converted automatically from vector and array
  ```rust
  let v: Vec<f64> = vec![0.0,  0.707,  1.0,  0.707];
  let a: [f64; 4] =     [0.0, -0.707, -1.0, -0.707];
  ---
  let sv: &[f64] = &v;
  let sa: &[f64] = &a;
  ```
  ![slice](https://i.imgur.com/kM9W5Qh.png)
- C/C++ doesn't support multiline string while Rust does.
  ```rust
  println!("hello world, Rust
    the weathere is good today");
  ```
  output will includes newline and space at the beginning of the second line
