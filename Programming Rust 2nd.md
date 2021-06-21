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
- Rust borrow rules: you either have any number of immutable references or only one mutable reference at a given time (lifetime of references)
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

### 5. References

- if there is an immutable reference, the owner cannot modify its value. If there is a mutable reference, owner cannot be used (until the mutable reference goes away).
- to a function:
  - pass by value <-> move the ownership
  - pass by reference <-> borrow from the owner
- [`.` operator](https://doc.rust-lang.org/nomicon/dot-operator.html) performs a lot of magic to convert types including auto-reference, auto-dereference ...
- Once C++ reference has been initialized, there is no way to change while in Rust reference can be changed.
  ```rust
  let x = 10;
  let y = 20;
  let mut z = &x;
  z = &y;
  ```
- reference to references.
  ```rust
  struct Point { x: i32, y: i32 }
  let point = Point { x: 1000, y: 729 };
  let r: &Point = &point;
  let rr: &&Point = &r;
  let rrr: &&&Point = &rr;
  ```
  ![reference](https://i.imgur.com/KelOvgm.png)
- comparing references performs on their final target. Using `std::ptr:eq`, if you want to compare their addresses
  ```rust
  let x = 10;
  let y = 10;
  let rx = &x;
  let ry = &y;
  let rrx = &rx;
  let rry = &ry;
  assert!(rrx <= rry);
  assert!(rrx == rry);
  assert!(!std::ptr::eq(rx, ry));
  ```
- references are never null. You can reference to the value of any sort of expression.
  ```rust
  let x = &1000; // anonymous variable is created to hold a value
  ```
- every reference (to be correct, variable) has a lifetime which is used by Rust to prevent a dangling pointer.
  ```rust
  let x;
  {
    let y = 10;
    x = &y; // lifetime of &y from this point to end of block
  }
  println!("{}", x); // error
  ```
- you can explicity specify lifetime in function parameters and return.
  ```rust
  fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {}
  ```
- each kind of reference affects what we can do with other values in the same ownership tree.
  ![tree](https://i.imgur.com/52OG5Au.png)

### 6. Expressions

- Rust is expression language (most statements in C are expressions in Rust).
- value of the block is the last expression without a semicolon.
  ```rust
  {
    let x = 10;
    x  // value of this block
  }
  ```
- `if` without `else` must always return `()` and all blocks of `if` expression must produce values of the same type (similar to `match`).
  ```rust
  let suggested_pet =
    if with_wings { Pet::Buzzard } else { Pet::Hyena };  // ok
  let favorite_number =
      if user.is_hobbit() { "eleventy-one" } else { 9 };  // error
  let best_sports_team =
      if is_hockey_season() { "Predators" };  // error
  ```
- `fn` is declared inside a block, its scope is the entire block (outside cannot see it) and it cannot access local variables.
- a condition (in `if`, `while` ...) must be an expression of `bool` (Rust doesn't implicitly convert to bool).
- loops are also expressions in Rust (`while`, `for` produces `()`). `break` can be used with label and value while `continue` only can be used with a label
  ```rust
  let foo = 'outer: loop {
    break 'outer 10;
  }
  ```
- unlike C, Rust differentitate between `()` (unit type) and `!` (never returns).
- `a..b` is end-exclusive range (not include b) while `a..=b` is end-inclusive range.
- unlike C, `%` can be used on floating-point numbers
  ```rust
  let x = 12.5 % 10; // 2.5
  ```
- values of type `bool`, `char` or, C-like `enum` may be cast to any integer type. The opposite direction is not allowed for example cannot cast `u16` to `char` because some `u16` values cannot be presented in `char` (`0xd800`). We can use a standard method `std::char::from_u32()` which returns `Option<char>`

### 7. Error Handling

- There are two kinds of error handling: panic and `Result`.
- panic is safe (catch before it actually happens), it doesn't voliate any Rust's safety rules since stack (including heap segments linked to variables) is cleanup -> there is no dangling pointer . Panic is like `RuntimeException` in C++.
- second panic happens during the cleanup of the first panic causes fatal -> thread is aborted. You can also config panic behavior like `-C panic=abort` (abort in the first panic).
- there is shortcut for handling `Result` (like `unwrap/expect`) and error propagation (`?`).

### 8. Crates and Modules

- There are two kinds of crate: binary or library. You can either specify or let Rust figure it out by looking at `src/lib.rs` or `src/main.rs`.
- program can mix crates written in different editions since edition only affects how source code is construed.
- modules can be nested and be specified with `pub(super)/pub(in <path>)` to make them visible to a specific parent or its descendants.
- a path (smiliar to filesystem) can take two forms:
  - absolute path starts a crate root like `crate`.
  - relative path starts from a current module like `self/super`.
  ```rust
  mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
  }
  pub fn eat_at_restaurant() {
      // Absolute path
      crate::front_of_house::hosting::add_to_waitlist();
      // Relative path
      front_of_house::hosting::add_to_waitlist();
  }
  ```
- struct's fields, even private fields, are accessible in the module and its submodules where the struct is declared. Outside the module, only public fields are accessible.
- `const` is smiliar to C `#define`, `static` is smiliar to C `static`
  ```rust
  pub const FOO: i32 = 10;
  pub static BAZ: &str = "hello world";
  ```
- tests are ordinary functions marked with the `#[test]` attribute. You can either use `assert!` family macros to validate or return `Result<(), E>`.
- tests are compilied conditionally, `cargo build` skips the testing code.
- there are 3 styles of testings:
  - unit testing lives right alongside with your code.
  - integration testing, files (each file is compiled as a separate crate) in `tests` directory.
  - documentation testing, code block in your document. `rustdoc` stores each code sample in a separate file, adding boilterplate code to produce programs. You disable if needed via `no_run/ignore`.
  ```rust
  /// Return true if two ranges overlap.
  ///
  ///     assert_eq!(ranges::overlap(0..7, 3..10), true);
  ///     assert_eq!(ranges::overlap(1..5, 101..105), false);
  ///
  /// If either range is empty, they don't count as overlapping.
  ///
  ///     assert_eq!(ranges::overlap(0..0, 0..10), false);
  ///
  ---- // temporary file 1
  use ranges;
  fn main() {
      assert_eq!(ranges::overlap(0..7, 3..10), true);
      assert_eq!(ranges::overlap(1..5, 101..105), false);
  }
  ---- // temporary file 2
  use ranges;
  fn main() {
      assert_eq!(ranges::overlap(0..0, 0..10), false);
  }
  ```
- cargo uses semantic versioning, lock mechanism and workspace (probably like `yarn`).

### 8. Structs

- There are 3 kinds of struct types: named-field, tuple-like and unit-like.
- struct and its fields are private by default. Visiblity is for different modules, creating a struct value requires all the struct's fields are visible.
  ```rust
  mod Foo {
    #[derive(Debug)]
    pub struct Point {
        pub x: i32,
        y: i32,         // private
    }
  }
  pub struct Distance {
      meter: i32,
  }
  fn main() {
      let p = Foo::Point { x: 10, y: 20 }; // y is private, not visible since Point and main belong to different modules
      let d = Distance { meter: 100 };     // meter is private, Distance and main belong to the same module
  }
  ```
- not like spread operator in javascript, `..` in rust only takes fields (move) not mentioned.
  ```rust
  struct Point {
    x: i32,
    y: i32,
  }
  let p1 = Point { x: 10, y: 20 };
  let p2 = Point { x: 15, ..p1 }; // p2 = { 15, 20 }
  ```
- tuple-like struct is good for _newtypes_ since you get stricker type checking.
  ```rust
  struct Ascii(Vec<u8>);
  ```
- values of unit-like struct occupies no memory and no generated code.
- struct fields' values might be not stored in the order they are in struct (you can specify layout like `#[repr(C)]`).
- a method's `self` argument can also be a `Box<Self>`, `Rc<Self>`, or `Arc<Self>` (same for `&self` and `&mut self`).
  ```rust
  struct Queue {}
  impl Queue {
    pub fn push(&mut self, c: char) {}
  }
  let mut q = Queue {};
  q.push('h');
  let mut bq = Box::new(Queue {});
  bq.push('e');
  ```
- there are associated functions and consts.
  ```rust
  impl Queue {
    const ZERO: Queue = Queue {};
    const NAME: &str = "Queue";
    pub fn new() -> Queue {
      Queue {}
    }
  }
  ```
- generic struct is similar to c++ template, so you can specialize if needed.
- when you need mutable data inside an immutable value, there are `Cell<T>` and `RefCell<T>`.

### 10. Enums and Patterns

- enum is similiar to Haskell algebraic data types.
- enum without data is similar to C enum (default and following values). You can cast enum to integer, but integer to enum is not allowed.
  ```rust
  enum Status {
    Created,
    Pending = 10,
    Completed,
  }
  println!("{}", Status::Completed as i32); // 11
  ```
- there are 3 kinds of enum variant, echoing 3 kinds of struct. All constructions and fields share the same visibility of the enum.
  ```rust
  enum RelationshipStatus {
    Single,
    InARelationship,
    ItsComplicated(Option<String>),
    ItsExtremelyComplicated {
        car: DifferentialEquation,
        cdr: EarlyModernistPoem,
    },
  }
  ```
- enum with data is stored as a small integer tag, plus enough memory to hold all the fields the largest variant.
  ![enum](https://i.imgur.com/O0kGJKX.png)
- there are special cases, Rust can eliminate the tag field. For example, `Option<T>` when T is a reference (`Box` or smart pointer types), since T is cannot be null, so `None` can be represented as 0, `Some` for pointers.
- pattern matching supports a various types: literal, variable, tuple, struct, array, reference ...
  ```rust
  match get_account(id) {
    Some(Account { name, language, ..}) => something // match name, language and ignore other fields
  }
  ```
- matching a noncopyable value moves the value.
  ```rust
  match account {
    Account { name, language, .. } => {
        ui.greet(&name, &language);
        ui.show_settings(&account);  // error: borrow of moved value: `account`
    }
  }
  // borrow instead
  match account {
    Account { ref name, ref language, .. } => {
        ui.greet(name, language);
        ui.show_settings(&account);  // ok
    }
  }
  ```
- there are two kind of patterns:
  - irrefutable pattern always match, `let` and `for` only accepts this pattern.
  - refutable pattern might not match.
