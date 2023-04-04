- Basic Arithmetic
```rust
fn subtract(x:u64, y:u64) {
	let z: u64 = x-y;
	println!("subtracted sum: {}", z)}

fn addition(x:u64, y:u64) {
	let z: u64 = x+y;
	println!("added sum: {}", z);}

fn multiplication(x:u64, y:u64) {
	let z: u64 = x*y;
	println!("multiplied sum: {}", z);}

fn division(x:f64, y:f64) {
	let z: f64 = x/y;
	println!("divided sum: {}", z);}

fn modulo(x:f64, y:f64) {
	let z: f64 = x%y;
	println!("modulo: {}",z)
}
fn main() {
	let x = 10;
	let y = 5;

	subtract(x,y);
	addition(x,y);
	multiplication(x,y);
	division(x as f64,y as f64);}
```

- type casting & conversions
```rust
fn main() {
        let x = 100_i16;
        let y = 2000_u32;
        let z = 150_i32;
        
        let yy = y + (z as u32 - x as u32);

        println!("{}",yy);
}

```


- integer overflows
```rust
fn safe() {	/
	// not able to be compiled
	let x: u8 = 255;
	let y: u8 = 1;
	let z = x+y; 
	// compiler would catch the integer overflow
}

fn vulnerable() {
	let x = (i32::MAX as i64) + 1;
	let y = 10_i32;
	let z = x as i32 / y;
	println!("{}", z)
}

fn main() {
	safe();
	vulnerable();
}
```


- with the help of `wrapping_add()` and `overflowing_add()` you can preform integer overflows.
	- check https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_add for more
