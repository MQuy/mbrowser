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
- panic is safe (catch before it happens), it doesn't violate any of Rust's safety rules since stack (including heap segments linked to variables) is cleanup -> there is no dangling pointer. Panic is like `RuntimeException` in C++.
- second panic happens during the cleanup of the first panic causes fatal -> thread is aborted. You can also config panic behavior like `-C panic=abort` (abort in the first panic).
- there is a shortcut for handling `Result` (like `unwrap/expect`) and error propagation (`?`).

### 8. Crates and Modules

- There are two kinds of crate: binary or library. You can either specify or let Rust figure it out by looking at `src/lib.rs` or `src/main.rs`.
- program can mix crates written in different editions since edition only affects how source code is construed.
- modules can be nested and be specified with `pub(super)/pub(in <path>)` to make them visible to a specific parent or its descendants.
- a path (similar to filesystem) can take two forms:
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
- `const` is similar to C `#define`, `static` is similar to C `static`
  ```rust
  pub const FOO: i32 = 10;
  pub static BAZ: &str = "hello world";
  ```
- tests are ordinary functions marked with the `#[test]` attribute. You can either use `assert!` family macros to validate or return `Result<(), E>`.
- tests are compiled conditionally, `cargo build` skips the testing code.
- there are 3 styles of testings:
  - unit testing lives right alongside your code.
  - integration testing, files (each file is compiled as a separate crate) in `tests` directory.
  - documentation testing, code block in your document. `rustdoc` stores each code sample in a separate file, adding boilerplate code to produce programs. You disable if needed via `no_run/ignore`.
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
- cargo uses semantic versioning, lock mechanism, and workspace (probably like `yarn`).

### 8. Structs

- There are 3 kinds of struct types: named-field, tuple-like and unit-like.
- struct and its fields are private by default. Visibility is for different modules, creating a struct value requires all the struct's fields are visible.
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
- not like the spread operator in javascript, `..` in rust only takes fields (move) not mentioned.
  ```rust
  struct Point {
    x: i32,
    y: i32,
  }
  let p1 = Point { x: 10, y: 20 };
  let p2 = Point { x: 15, ..p1 }; // p2 = { 15, 20 }
  ```
- tuple-like struct is good for _newtypes_ since you get stricter type checking.
  ```rust
  struct Ascii(Vec<u8>);
  ```
- values of unit-like struct occupy no memory and no generated code.
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

- enum is similar to Haskell's algebraic data types.
- enum without data is similar to C enum (default and following values). You can cast enum to integer, but integer to enum is not allowed.
  ```rust
  enum Status {
    Created,
    Pending = 10,
    Completed,
  }
  println!("{}", Status::Completed as i32); // 11
  ```
- there are 3 kinds of enum variants, echoing 3 kinds of struct. All constructions and fields share the same visibility of the enum.
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
- pattern matching supports various types: literal, variable, tuple, struct, array, reference ...
  ```rust
  match get_account(id) {
    Some(Account { name, language, ..}) => something // match name, language and ignore other fields
  }
  ```
- matching a non copyable value moves the value.
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
  - irrefutable pattern always matches, `let` and `for` only accept this pattern.
  - refutable pattern might not match.

### 11. Traits and Generics

- traits is inspired by Haskell's typeclass, while generics is similar to C++ template (generate machine code for each type `T` that you use).
- to use traits methods for a type, you have to explicitly import that traits.
- C# interface, a value of type T is a reference to any object that implements T, the same for Rust.
  ```rust
  use std::io::Write;
  let mut buf: Vec<u8> = vec![];
  let writer: dyn Write = buf;  // error: `Write` does not have a constant size
  let writer: &dyn Write = &buf;  // ok
  let writer: &mut dyn Write = &mut buf;  // ok, creating fat pointer (trait object) contains a pointer to data and a pointer to vtable
  ```
  [trait object](https://i.imgur.com/RSudX7c.png)
- unlike C++, the vtable pointer is stored as part of the struct, Rust uses flat pointers so a struct can implement dozens of traits without containing dozens of vtable pointers -> a method call will be dynamic dispatching.
- references and smart pointers (`Box`, `Rc` ...) are converted to trait objects when needed.
- difference between generic and trait
  - generic generates machine code for each type you use (easy to optimize and better speed).
  - trait object uses dynamic dispatch.
  ```rust
  fn say_hello(out: &mut dyn Write)     // plain
  fn say_hello<W: Write>(out: &mut W)   // generic
  fn say_hello(out: &mut dyn Write)     // trait
  ```
- everything defined in a trait `impl` must be a feature of trait (if you need a helper method, defining in `impl` of that type).
  ```rust
  trait Visible {
    fn draw(&self, canvas: &mut Canvas);
    fn hit_test(&self, x: i32, y: i32) -> bool;
  }
  impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }
    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x
        && self.y - self.height - 1 <= y
        && y <= self.y
    }
  }
  impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.height - 1 .. self.y
    }
  }
  ```
- when implementing a trait, either the trait or the type must be local in the current crate (_orphan rule_) to ensure that other people's code cannot break yours and vice versa.
- a trait that uses `Self` is incompatible with trait objects
  ```rust
  pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self; // requires self and other are the same type
  }
  impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self {
    }
  }
  impl Spliceable for Mammoth {
    fn splice(&self, other: &Self) -> Self {
    }
  }
  fn splice_anything(left: &dyn Spliceable, right: &dyn Spliceable) {
    let combo = left.splice(right); // error since Rust doesn't know at compile type if `left and `right` are the same type as required.
  }
  // if we replace with the trait below, it will works.
  pub trait MegaSpliceable {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
  }
  ```
- trait can include type-associated functions. If you want to use `&dyn StringSet` (trait object), you have to add the bounding and those bounding functions are excluded.
  ```rust
  trait StringSet {
    fn new() -> Self;
    fn contains(&self, string: &str) -> bool;
    ----
    fn new() -> Self          // for trait object usage since it is not included -> only can use contains
        where Self: Sized;
  }
  ```
- associated types trait defines one specific related type for each implementation.
  ```rust
  pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
  }
  impl Iterator for Args {
    type Item = String;
    fn next(&mut self) -> Option<String> {
    }
  }
  ```
- generic type get a special dispensation, you can implement a foreign trait for a foreign type, as long as one of the trait's type parameters is defined in the local crate.
  ```rust
  struct Meters(u32);
  impl Add<Meters> for Millimeters {            // Milimeters is the 3rd crate
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
      Millimeters(self.0 + (other.0 * 1000))
    }
  }
  ```
- associated trait consts can be declared without giving a value, then implementors of that trait can define those.
  ```rust
  trait Greet {
    const GREETING: &'static str = "Hello";
    const ZERO: Self;
  }
  impl Greet for f32 {
    const ZERO: f32 = 0.0;
  }
  ```

### 12. Operator Overloading

- arithmetic expressions are shorthand for method calls like `a + b` -> `a.add(b)` (`add` belongs `std::ops::Add`).
  ```rust
  trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
  }
  assert_eq!(10.add(20), 10 + 20); // ok
  ```
- `std::cmp::PartialEq` contains 2 methods. Since `ne` method has a default definition, you only need to define `eq`.
  ```rust
  trait PartialEq<Rhs = Self>
  where
      Rhs: ?Sized,
  {
      fn eq(&self, other: &Rhs) -> bool;
      fn ne(&self, other: &Rhs) -> bool {
          !self.eq(other)
      }
  }
  ```
- math equivalence relation imposes three requirements below. `PartialEq` is used for the `==` operator's built-in trait because `==` doesn't meet the third one like `NaN` is not equal to `NaN`. There is `Eq` trait that represents all.
  - `x == y` -> `y == x`.
  - `x == y` and `y == z` -> `x == z`.
  - `x == x`.
- among all primitive types, only comparisons between floating-point values return `None` (`NaN` with anything else returns `None`).
- `std::cmp::PartialOrd` contains 5 methods but 4 methods have default definition. The only you have to implement is `partial_cmp`.
  ```rust
  trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
  where
      Rhs: ?Sized,
  {
      fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>; // None means self and other are unordered with each other (like NaN)
      fn lt(&self, other: &Rhs) -> bool { ... }
      fn le(&self, other: &Rhs) -> bool { ... }
      fn gt(&self, other: &Rhs) -> bool { ... }
      fn ge(&self, other: &Rhs) -> bool { ... }
  }
  ----
  enum Ordering {
    Less,       // self < other
    Equal,      // self == other
    Greater,    // self > other
  }
  ```
- depends on the expression (like borrowing), Rust translates to either `std::ops::Index` or `std::ops::IndexMut`.
  ```rust
  let mut m = HashMap::new();
  assert_eq!(m["十"], 10);
  assert_eq!(m["千"], 1000);
  // ->
  assert_eq!(*m.index("十"), 10);
  assert_eq!(*m.index("千"), 1000);
  ```
  ```rust
  let mut desserts = vec!["Howalon".to_string(), "Soan papdi".to_string()];
  desserts[0].push_str(" (fictional)");
  desserts[1].push_str(" (real)");
  // ->
  (*desserts.index_mut(0)).push_str(" (fictional)");
  (*desserts.index_mut(1)).push_str(" (real)");
  ```
- not all operators can be overloaded like `?`, `=`, `&&`, `||`, function call operator ...

### 13. Utility Traits

- There are 3 categories of traits
  - Language extension traits: integrate your own types closely to the language like `Drop`, `Deref` ...
  - Marker traits: express constaints to bound generic type variables like `Sized` and `Copy`.
  - Public vocabulary traits like `Default`, `Borrow` ...
- `Drop` trait which is analogous to C++ destructor is called (dropping in stack, heap and system resources) when a value's owner goes away (out of scope, end of expression statement, truncate a vector ...). Usually, it is handled automatically but you can define it needed.
  ```rust
  struct Appellation {
    name: String,
    nicknames: Vec<String>
  }
  impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
  }
  ```
- `Copy` trait is for types whose values can be duplicated simply by copying bits (on stack) while `Clone` trait is always explicit and may or may not expensive.
- `std::ops::Deref/DerefMut` traits specify how deferencing operators `*` and `.` behave. They are designed for implementating smart pointers like `Box`, `Rc`, `Arc` ...
  ```rust
  trait Deref {
    type Target: ?Sized;
    fn deref(&self) -> &Self::Target;
  }
  trait DerefMut: Deref {
      fn deref_mut(&mut self) -> &mut Self::Target;
  }
  ```
- Since `deref` takes `&Self` reference and returns a `&Self::Target` reference, references can be converted to the latter. Rust will apply several deref coercions in succession if necessary.
  ```rust
  struct Selector<T> {
    /// Elements available in this `Selector`.
    elements: Vec<T>,
    /// The index of the "current" element in `elements`. A `Selector`
    /// behaves like a pointer to the current element.
    current: usize
  }
  impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
  }
  let mut s = Selector { elements: vec!['x', 'y', 'z'],
                       current: 2 };
  // Because `Selector` implements `Deref`, we can use the `*` operator to
  // refer to its current element.
  assert_eq!(*s, 'z');
  // Assert that 'z' is alphabetic, using a method of `char` directly on a
  // `Selector`, via deref coercion.
  assert!(s.is_alphabetic());
  // Change the 'z' to a 'w', by assigning to the `Selector`'s referent.
  *s = 'w';
  // ---------
  assert_eq!(s.elements, ['x', 'y', 'w']);
  let s = Selector { elements: vec!["good", "bad", "ugly"],
                   current: 2 };
  fn show_it(thing: &str) { println!("{}", thing); }
  show_it(&s);
  // type of &s is &Selector<&str>, due to Deref<Target=str> -> show_it(s.deref())
  // if you change show_it to generic function, it doesn't work since Rust doesn't try deref coercions to satisfy type variable bounds
  ```
- `AsRef<T>/AsMut<T>` is used to make functions more flexiable in the argument types they accept
  ```rust
  fn open<P: AsRef<Path>>(path: P) -> Result<File>
  let dot_emacs = std::fs::File::open("/home/jimb/.emacs")?;
  ```
  `open` wants `&Path` (filesystem path type). By using `AsRef<Path>`, `open` accepts anything it can borrow a `&Path` from, in that case `str`. Rust rewrite like `std::fs::File::open("/home/jimb/.emacs".as_ref())?`
- `std::borrow::Borrow` trait is similar to `AsRef1`, except it imposes more restrictions: a type should implement `Borrow<T>` when a `&T` hashes and compares the same way as the value it's borrowed from.
- `std::convert::From/Into` are infalliable trait and take ownership of their argument, transform it, and return ownership of the conversion. `Into` is generally used to make your function's arguments are flexiable while `From` serves as a generic constructor for producing an instance of type from another value.
  ```rust
  use std::net::Ipv4Addr;
  fn ping<A>(address: A) -> std::io::Result<bool>
      where A: Into<Ipv4Addr>
  {
      let ipv4_address = address.into();
      ...
  }
  println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141))); // pass an Ipv4Addr
  println!("{:?}", ping([66, 146, 219, 98]));             // pass a [u8; 4]
  println!("{:?}", ping(0xd076eb94_u32));                 // pass a u32
  ```
  ```rust
  let addr1 = Ipv4Addr::from([66, 146, 219, 98]);
  let addr2 = Ipv4Addr::from(0xd076eb94_u32);
  ```
- `TryFrom/TryInto` are fallible cousins of `From/Into` which including expressive error handling.
  ```rust
  let smaller: i32 = huge_number.try_into().unwrap_or(i32::MAX);
  ```

### 14. Closures

- There are two ways for closures to get data from enclosing scopes: move or borrowing.
  ```rust
  let color = String::from("green");
  // A closure to print `color` which immediately borrows (`&`) `color` and
  // stores the borrow and closure in the `print` variable. It will remain
  // borrowed until `print` is used the last time.
  //
  // `println!` only requires arguments by immutable reference so it doesn't
  // impose anything more restrictive.
  let print = || println!("`color`: {}", color);
  // Call the closure using the borrow.
  print();
  //
  // using move to specify a closure will steal color
  let print_move = move || { println!("{}", color); };
  print_move();
  let color1 = color; // error since color ownership is already moved to print_move
  ```
- all functions and most closures (others are implemented `FnOnce` and `FnMut`) are implemented automatically `Fn` trait. Every closure has an ad hoc type created by the compiler, no two closures have exactly the same type.
  ```rust
  fn(&City) -> bool    // fn type (functions only)
  Fn(&City) -> bool    // Fn trait (both functions and closures)
  ```
- there are three kinds of closure traits below. Without specifing `move`, closure prefer capturing variables by reference and only go lower (by mutable reference -> by value) when required.
  - `Fn` for functions and closures which you can call multiple times without restriction.
  - `FnMut` for closures which you can call multiple time if closure itself is declared `mut`.
  - `FnOnce` for closures which you can call once, if the caller owns the closure.
- in memory, closure looks like a small structure containing references to the variables or values it uses.
  ![closure](https://i.imgur.com/NiQrkPc.png)
- rules for `Copy` and `Clone` on closures are like for regular struct mentioned above
  - non-`move` closure only hold shared references which are `Clone/Copy` supports `Clone/Copy`.
  ```rust
  let y = 10;
  let add_y = |x| x + y;
  let copy_of_add_y = add_y;                // This closure is `Copy`, so...
  assert_eq!(add_y(copy_of_add_y(22)), 42); // ... we can call both.
  ```
  - non-`move` closure mutates references in its body doesn't support `Clone/Copy`.
  ```rust
  let mut x = 0;
  let mut add_to_x = |n| { x += n; x };
  let copy_of_add_to_x = add_to_x;         // this moves, rather than copies
  assert_eq!(add_to_x(copy_of_add_to_x(1)), 2); // error: use of moved value
  ```
  - `move` closure, everything captured is either `Copy` -> `Copy` or `Clone` -> `Clone`.
  ```rust
  let mut greeting = String::from("Hello, ");
  let greet = move |name| {
      greeting.push_str(name);
      println!("{}", greeting);
  };
  greet.clone()("Alfred"); // Hello, Alfred
  greet.clone()("Bruce");  // Hello, Bruce
  ```
  `greeting` is moved to `greet`, and when `greet` (is a structure) is cloned, `greeting` is cloned as well -> two copies of `greeting` which are modified separately when the clones of `greet` are called.

### 15. Iterators

- an iterator is any value that implements the `std::iter::Iterator` trait and any type that implements `IntoIterator` is an `iterable`.
  ```rust
  trait Iterator {
      type Item;
      fn next(&mut self) -> Option<Self::Item>;
      ... // many default methods
  }
  trait IntoIterator where Self::IntoIter: Iterator<Item=Self::Item> {
    type Item;                  // type of iterator produces
    type IntoIter: Iterator;    // type of iterator value
    fn into_iter(self) -> Self::IntoIter;
  }
  ```
- under the hood, every `for` loop is just shorthand for calls to `IntoIterator` and `Iterator` methods
  ```rust
  println!("There's:");
  let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
  // for
  for element in &v {
      println!("{}", element);
  }
  // sugar syntax for while
  let mut iterator = (&v).into_iter();
  while let Some(element) = iterator.next() {
      println!("{}", element);
  }
  ```
- `into_iter` returns iterator which yields values, immutable references or mutable references is context dependent. While `iter` and `iter_mut` return iterator which yields `&T` and `&mut T`, respectively.
- iterator adapter is the concept that consumes on iterator and build a new one with useful behaviors. There are two important points:
  - simply calling an adapter on an iterator doesn't consume any items, just returns a new iterator. In a chain of adapters, the only way to make any work actually get done is to call `next` on the final iterator (output is lazy evaluation, each value is constructed via `next` call)
  ```rust
  ["earth", "water", "air", "fire"]
    .iter().map(|elt| println!("{}", elt)); // nothing happens until you actually demand a value via .next method
  ```
  - adapter is zero-overhead abstraction, for example applying `map`, `filter` ... to an iterator will specialize their code
  ```rust
  let text = "  ponies  \n   giraffes\niguanas  \nsquid".to_string();
  let v: Vec<&str> = text.lines()
      .map(str::trim)
      .filter(|s| *s != "iguanas")
      .collect();
  // ->
  let mut v = Vec::new();
  for line in text.lines() {
    let line = line.trim();
    if line != "iguanas" {
        v.push(line);
    }
  }
  ```
- adapters take ownership of the underlying iterator. Using `by_ref`, you can borrow a mutable reference to the iterator
  ```rust
  let message = "To: jimb\r\n\
               From: id\r\n\
               \r\n\
               Oooooh, donuts!!\r\n";
  let mut lines = message.lines();
  println!("Headers:");
  for header in lines.by_ref().take_while(|l| !l.is_empty()) {  // lines iterator changes its internal position to 3
      println!("{}" , header);
  }
  println!("\nBody:");
  for body in lines {         // same iterator -> lines iterator continues consuming with internal position 3
      println!("{}" , body);
  }
  ```
- the standard library provides a blanket implementation of `IntoIterator` for every type that implements `Iterator` (iterator and iterable are the same type)
  ```rust
  struct I32Range {
    start: i32,
    end: i32
  }
  impl Iterator for I32Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        }
        let result = Some(self.start);
        self.start += 1;
        result
    }
  }
  for k in (I32Range { start: 0, end: 14 }) {   // works
  }
  ```
  for other cases, you can create another structure to keep track current progress.
  ```rust
  enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
  }
  struct TreeNode<T> {
      element: T,
      left: BinaryTree<T>,
      right: BinaryTree<T>
  }
  struct TreeIter<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>
  }
  impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter { unvisited: Vec::new() };
        iter
    }
  }
  ```

### 16. Collections

- Rust's collections are different comparing to other languages' collections:
  - move and borrowing are everywhere to void deep-coping values, for example `Vec<T>::push(item)` takes its argument by value not reference.
  - doesn't have invalidation errors when a collection is resized (program is holding a pointer to data inside it).
- collections interal structures
  ![VecDeque](https://i.imgur.com/JlymONY.png)
  ![Hashmap](https://i.imgur.com/zMDvHwl.png)
  ![BTreeMap](https://i.imgur.com/pqrQtez.png)
- both `HashMap` and `BTreeMap` have a corresponding `Entry` type to eliminate redudant map lookups.
  ```rust
  // Do we already have a record for this student?
  if !student_map.contains_key(name) {
      // No: create one.
      student_map.insert(name.to_string(), Student::new());
  }
  // Now a record definitely exists.
  let record = student_map.get_mut(name).unwrap();
  // ->
  let record = student_map.entry(name.to_string()).or_insert_with(Student::new); // lookup only once and use for subsequent operations
  ```
- Sets are collections of values arranged for fast membership testing and never contains multiple copies of the same value. Internally, a set is like a map with only keys. In fact, `HashSet<T>` and `BTreeSet<T>` are thin wrappers around `HashMap<T, ()>` and `BTreeMap<T, ()>`.
  ```rust
  let b1 = large_vector.contains(&"needle");    // slow, checks every element
  let b2 = large_hash_set.contains(&"needle");  // fast, hash lookup
  ```

### 17. Strings and Text

- `String` and `str` types represent text using UTF-8 encoding form.
  [UTF-8](https://i.imgur.com/TPdrP0u.png)
- there are two restrictions on well-formed UTF-8 sequences (first bytes and following bytes are always distinct):
  - only the shortest encoding for any given code point is considered well-formed.
  - exclude encode numbers from `0xd800-0xdfff` and `> 0x10ffff`.
- Unicode stores characters in order in which they would normally be written or read.
  ```rust
  assert_eq!("ערב טוב".chars().next(), Some('ע'));
  ```
- use byte offset in the midst of some UTF-8 characters encoding -> the method panics since it causes ill-formed UTF-8.
  ```rust
  let full = "xin chào";
  println!("Result {}", full[7..]); // panic
  ```
- `String` is implemented as a wrapper around `Vec<u8>`, it encourages to build strings from begin to end by appending small pieces like the way vector works.
  ```rust
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s = s1 + &s2;      // s1 is moved and s2 is appended to s1
  // s1.add(&s2);
  ```
- iterate over a slice
  ![iterate slice](https://i.imgur.com/dEEEttj.png)
- if a type implements `Display`, the standard library automatically implements the `std::str::ToString` trait for it.
- there are two main ways to get the bytes representing text:
  - `slice.as_bytes()` is borrow bytes as `&[u8]` and it is immutable reference -> its bytes will remain well-formed.
  - `slice.into_bytes()` take ownership of `string` and returns a `Vec<u8>`, since `string` no longer exists -> no need for bytes to be in well-formed.
- Unicode has two ways to represent the text:
  - composed form, for example `thé` == [`t`, `h`, `é`] where `é` is a single Unicode character.
  - decomposed form, for example `thé` == [`t`, `h`, `e`, `\u{301`] where `0x301` adds an acute accent to character it follows.

### 18. Input and Output

- input and output are organized around three traits `Read`, `BufRead` and `Write`:
  - `Read` for byte-oriented input (_readers_).
  - `BufRead` for buffered readers. They support all methods of `Read` plus methods for reading lines of text and so forth.
    ![buffer](https://i.imgur.com/crqeiBS.png)
  - `Writer` for both byte-oriented and UTF-8 output (_writers_).
    ![io](https://i.imgur.com/GBSUTml.png)
- `OpenOptions` is advanced usage when `File` doesn't fit your requirement
  ```rust
  use std::fs::OpenOptions;
  let log = OpenOptions::new()
      .append(true)  // if file exists, add to the end
      .open("server.log")?;
  let file = OpenOptions::new()
      .write(true)
      .create_new(true)  // fail if file exists
      .open("new_file.txt")?;
  ```
- operating system doesn't force filenames to be valid Unicode. You have to use `OsStr/OsString` and `Path` for filenames since `str/String` only accept well-formed Unicode.
  ```rust
  $ echo "hello world" > ô.txt                                            // valid
  $ echo "O brave new world, that has such filenames in't" > $'\xf4'.txt  // invalid
  ```
- use `Path` for both absolute and relative paths, `OsStr` for an individual component of a path. `Path` is exactly like `OsStr`, it just adds many handy filename-related methods.
  | | str | OsStr | Path |
  | ------------- |:-------------:| -----:| -----:|
  | Unsized type, always passed by reference | Yes | Yes | Yes |
  | Can contain any Unicode text | Yes | Yes | Yes |
  | Looks just like UTF-8, normally | Yes | Yes | Yes |
  | Can contain non-Unicode data | No | Yes | Yes |
  | Text processing methods | Yes | No | Yes |
  | Filename-related methods | No | No | No |
  | Owned, growable, heap-allocated equivalent| `String` | `OsString` | `PathBuf` |
  | Convert to owned type | `.to_string()` | `.to_os_string()` | `.to_path_buf()` |
- there are standard libaries, like `std::os::unix::fs::symlink`, which are only available on certain platforms and you can conditional compile for those.
  ```rust
  #[cfg(unix)]
  use std::os::unix::fs::symlink;
  /// Stub implementation of `symlink` for platforms that don't provide it.
  #[cfg(not(unix))]
  fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, _dst: Q)
      -> std::io::Result<()>
  {
  }
  ```

### 19. Concurrency

- fork-join parallelism for handling completely independent tasks at the same time.
  ![fork](https://i.imgur.com/7B2497R.png)
  - you can use atomic reference counting like `Arc` to share data between threads.
    ```rust
    fn process_files(filenames: Vec<String>) -> io::Result<()> {
        Ok(())
    }
    fn process_files_in_parallel(filenames: Vec<String>, glossary: Arc<GigabyteMap>) -> io::Result<()>
    {
        const NTHREADS: usize = 8;
        let worklists = split_vec_into_chunks(filenames, NTHREADS);
        for worklist in worklists {
            // This call to .clone() only clones the Arc and bumps the
            // reference count. It does not clone the GigabyteMap.
            let glossary_for_child = glossary.clone();
            thread_handles.push(
                spawn(move || process_files(worklist, &glossary_for_child))
            );
        }
    }
    ```
  - unlike java and c# (exceptions in child thread are dumped to terminal and forgotten) and c++ (abort the process), in Rust errors are `Result` values (data) instead of exceptions (control flow) which are passed back to parent thread.
- channel is one-way pipe (thread-safe queue) for sending values (while Unix pipe for sending bytes) from one thread to another and data is moved between from sender to receiver. Channel is implemented via `std::sync::mpsc` stands for multiproduce and single-consumer.
  ![channel](https://i.imgur.com/9Y1r6el.png)
  - internally, Rust uses different queue implementation when using channel. When a channel is created, special "one-shot" queue implementation. Second value is sent, different queue implementation is used and if you clone sender, another queue implementation is used.
  - there is the case when sending values faster than receiving, you can use _synchronous channel_ which specifies how many values can be hold.
    ```rust
    let (sender, receiver) = mpsc::sync_channel(1000);
    ```
  - under the hood, thread safety (no data races and undefined behaviors) is based on two builtin traits `std::marker::Send` (for move) and `std::marker::Sync` (for non-mut references).
    ![marker](https://i.imgur.com/jACF7Yw.png)
    if `Rc<String>` is `Sync` and both threads happen to clone the `Rc` at the same time -> data races in shared reference count.
    ![Rc data race](https://i.imgur.com/NGxQ6Oa.png)
  - `OffThreadExt` allows us to unify iterator pipelines and thread pipelines.
    ```rust
    documents.into_iter()
      .map(read_whole_file)
      .errors_to(error_sender)   // filter out error results
      .off_thread()              // spawn a thread for the above work
      .map(make_single_file_index)
      .off_thread()              // spawn another thread for stage 2
    // ----
    impl<T> OffThreadExt for T
    where T: Iterator + Send + 'static,
          T::Item: Send + 'static
    {
        /// Transform this iterator into an off-thread iterator: the
        /// `next()` calls happen on a separate worker thread, so the
        /// iterator and the body of your loop run concurrently.
        fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
            // Create a channel to transfer items from the worker thread.
            let (sender, receiver) = mpsc::sync_channel(1024);
            // Move this iterator to a new worker thread and run it there.
            thread::spawn(move || {
                for item in self {
                    if sender.send(item).is_err() {
                        break;
                    }
                }
            });
            // Return an iterator that pulls values from the channel.
            receiver.into_iter()
        }
    }
    ```
- mutex is used to force multiple threads to take turns when accessing certain data. In Rust mutex and data are combined into one form `Mutex<T>` (in C++, mutex type and data are separated).
  ```c++
  class FernEmpireApp {
  private:
      // List of players waiting to join a game. Protected by `mutex`.
      vector<PlayerId> waitingList;
      // Lock to acquire before reading or writing `waitingList`.
      Mutex mutex;
  };
  ```
  ```rust
  /// All threads have shared access to this big context struct.
  struct FernEmpireApp {
      waiting_list: Mutex<WaitingList>,
  }
  impl FernEmpireApp {
    /// Add a player to the waiting list for the next game.
    /// Start a new game immediately if enough players are waiting.
    fn join_waiting_list(&self, player: PlayerId) {
        // Lock the mutex and gain access to the data inside.
        // The scope of `guard` is a critical section.
        let mut guard = self.waiting_list.lock().unwrap();
        guard.push(player);
        if guard.len() == GAME_SIZE {
            let players = guard.split_off(0);
            self.start_game(players);
        }
    }
  }
  ```
  - we don't need `&mut self` in `join_waitting_list` because `Mutex` is the lock which provides exclusive (`mut`) access to the data inside, even though many threads may have share (non-`mut`) access to the `Mutex`.
  - if a thread panics while holding a `Mutex`, the `Mutex` is marked as _poisoned_. Any subsequent attempt to `lock` will get an error result. You can still lock and access the data inside poisoned mutex with fully enforced via `std::sync::PoisonError`.
  - `RwLock` provides read/write locking methods `read` (non-`mut` access) and `write` (`mut` access).
  - `CondVar` has `.wait()` and `.notify_all()`, `.wait()` blocks until other threads call `.notify_all()`.
  - Rust provides atomic types which are similar to C++ atomics.
  ```rust
  let cancel_flag = Arc::new(AtomicBool::new(false));
  let worker_cancel_flag = cancel_flag.clone();
  let worker_handle = thread::spawn(move || {
    for pixel in animation.pixels_mut() {
        render(pixel); // ray-tracing - this takes a few microseconds
        if worker_cancel_flag.load(Ordering::SeqCst) {
            return None;
        }
    }
    Some(animation)
  });
  // Cancel rendering.
  cancel_flag.store(true, Ordering::SeqCst);
  // Discard the result, which is probably `None`.
  worker_handle.join().unwrap();
  ```
  - for global variables, we either use atmoic types (numbers and boolean) or custom types with two restrictions:
    - the variable must be thread safety `Sync` and non-`mut` (workaround to use `Mutex`, `RwLock` and atomic types to modify).
    - static initializer can only call functions which are marked as `const` (smiliar to C++ `constexpr`).
    ```rust
    const fn mono_to_rgba(level: u8) -> Color {
        Color {
            red: level,
            green: level,
            blue: level,
            alpha: 0xFF
        }
    }
    const WHITE: Color = mono_to_rgba(255);
    const BLACK: Color = mono_to_rgba(000);
    static HOSTNAME: Mutex<String> = Mutex::new(String::new()); // error Mutex::new is not const
    lazy_static! {  // lazy_static crate which allows you to use any expression you like to initialize
        static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
    }
    ```

### 23. Foreign Functions
