```rust
use std::io

fn main() {
	let mut input = String::new(); // variable to assign input
	io::stdin().read_line(&mut input).expect("failed to read line") // collect input and assign to input variable, &mut is a mutable refrence, expect is an expection
}
```