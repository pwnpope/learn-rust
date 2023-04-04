### Variables
```rust 
let name = 4; // immuatable
let mut name_one = 0; // mutable
let name_two: u32 = 20; // manually defined type u32
```

---

### Shadowing
- shadowing in rust is the process of redeclaring a variable with the same name,
effectively creating a new variable that "shadows" or "hides" the previous one in the same current scope

```rust
fn main() {
	let x = 4; // x=4

	{ // different scope
		let x = 2+1; // x=3 redifining x to be 3 
	}
	{ // another different scope
                let x = x-2 // x=2 can still use the variable from the outer scope 
	}

	let x = x+1 // x is 5
}
```

### constants
- constants cannot change type or value
- constants types must be defined
- A `const`ant in Rust is immutable. You neither can reassign nor modify it:

```rust
fn main() {
	const SECONDS_IN_MINUTE: u32 = 60;
	println!("{}", SECONDS_IN_MINUTE)
}
```

- A `static` variable _can_ be mutable and therefore can either be modified or reassigned
	- Note that writing/modifying a global `static` variable is unsafe and therefore needs an `unsafe` block:
```rust
static mut foo:i32 = 500;

fn main() {
	unsafe {
		foo = 250;
	}
}
```