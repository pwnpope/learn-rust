- rust structs
	- similiar to classes or objects
	- structs represent complex data types that you define
	- structs are like "objects", but have differences
		- rust doesn't have inheritance
		- rust does have methods
		- rust has "traits" - similiar to polymorphism for OOP

```rust
struct pwnpwn {
        x: i32,
        y: i32,
        z: bool,
}

fn main() {
        let mut x_var = pwnpwn {
                x: 25000,
                y: 500,
                z: true,
        };
        x_var.x = 100; // changing a field's value
}

```
