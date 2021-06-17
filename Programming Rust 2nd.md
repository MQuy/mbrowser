### 2. A Tour Of Rust

- if a function body ends with an expression which is not followed by a semicolon, that's a function return value
  ```rust
  fn demo() -> i32 { 1 }
  ```
- block surrounded by curly braces can function as an expression and the last expression without semicolon can be treated as its value
  ```rust
  let x = { 1 };
  ```

### Fundamental Types

- soundness type system
  - type system is sound implies all of the type-checked programs are correct (i.e. no [false negative](https://en.wikipedia.org/wiki/False_positives_and_false_negatives#False_negative_error)).
  - type system is complete implies all of the correct programs can be accepted by type-checked (i.e. no [false positive](https://en.wikipedia.org/wiki/False_positives_and_false_negatives#False_positive_error)).
  - [soundness vs completeness](https://en.wikipedia.org/wiki/Soundness#Relation_to_completeness): soundness means a program passed type check is valid (run without any errors) while completeness means a valid program can be type-checked.
- `isize/usize` has the same size of an address on the machine (32 or 64 bits).
- `char` is unicode character, 32 bits wide.
- if an integer literal lacks a type suffix, its type is determined when there is a clue (stored in a variable of a particular type, passed to a function, or return to a function ...). If there are multiple possibilities, i32 will be picked (same for f64).
  ```rust
  println!("{}", (-4).abs());
  ```
  doesn't compile since Rust doesn't know which integer type a value belongs to, default i32 applies only if the type is still ambiguous after all method calls have been resolved which is too late here.
- integer downcast from N to M bits (N >> M) is doing the truncation of N - M most significant bits.
- signed integer overflow is undefined in C/C++, Rust introduces _Checked, Wrapping, Saturating and Overflowing_ to handle in the way you want. By default, doing that will cause panic in debug build and modulo wrap in release build.
- function returns no value has type of `()` (unit type)
  ```rust
  fn foo(x: i32, y: i32);
  ==
  fn foo(x: i32, y: i32) -> ();
  ```
- Rust permits an extra trailing comma in function arguments, arrays, struct, enum definition ... and it has no meaning
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
    the weather is good today");
  ```
  the output will include newline and space at the beginning of the second line

### 4. Ownership and Moves

- there are two popular ways (using garbage collection or explicitly allocate/free) to manage memory while running. Rust uses a different approach, memory is managed through ownership which the compiler checks at compile-time.
- ownership rules:
  - each value in Rust has a variable called the owner.
  - there can be only one owner at a time.
  - when the owner goes out of scope, the value is dropped.
- for most types (except _Copy_ trait), operations like assigning a value to a variable, passing it to a function, or returning it from a function don't copy the value, _move_ is used.
  ```rust
  let s = vec!["udon", "ramen", "soba"];
  let t = s;
  println!("{:?}", t); // cannot use s, since value is moved to u
  ```
  ![move](https://i.imgur.com/BiEl2by.png)
- if you move a value into a variable that was already initialized, the prior value is dropped.
  ```rust
  let mut s = String::from("hello");
  s = String::from("world"); // string hello is dropped
  ---
  let mut s = String::from("hello");
  let t = s;
  s = String::from("world"); // nothing is dropped here
  ```
- if there is a possibility a value is moved, it is considered moved
  ```rust
  fn foo(c: u32) {
    let x = vec![10, 20, 30];
    if c > 10 {
        baz(x);
    }
    baz(x); // error: x is uninitialized here since x might be moved in if
  }
  ```
- any type that needs to do something special when a value is dropped cannot be _Copy_ (whole value is on stack)
