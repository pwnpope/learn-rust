```rust
// trim() will remove the newline character that we recieve from user input
// parse() parses a string into seeing if it can be returned as the respected type in this case we're using i64
// unwrap() takes any valid integer result and type defines it into the respected type

use std::io;

fn main() {
	let mut input: i64 = String::new();
	io::stdin().read_line(&mut input).expect("exception");

	let converted_input: i64 = input.trim().parse().unwrap();
	println!("{}", int_input);
}
```