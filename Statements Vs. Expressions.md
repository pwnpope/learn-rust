- Rust is an expression-oriented language. This means that most things are expressions, and evaluate to some kind of value. However, there are also statements. - > [Steve Klabnik](https://twitter.com/steveklabnik) (member of the Rust core team)


- i.e: assigning a variable a value is a statement, it doesn't return anything
```rust
let number = 100;
```


- Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
```rust
fn add(x:i32, y:i32) -> i32 {
        return x+y; // returns an expression
		// x+y without 'return' will also return as an expression
}

fn main() {
        let tup = (100,200);
        let result = add(tup.0, tup.1);
        println!("{}", result);
}
```

