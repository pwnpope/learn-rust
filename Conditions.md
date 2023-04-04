- operators
```rust
// < - less than
let a = (2 as f32) < 2.2;
// > - great than
let b = 5>10;
// <= - less than or equal to
let c = 10<=5;
// >= - greater than or equal to
let d = 20>=10;
// != - not equal to
let e = 10 != 2;
// == - equal to
let f = c==b;
```
- note: running conditions with different types will of course throw an error
	- manually type define your conditions if handling unknown types or different types
---

- compound conditions
	- multiple conditions using (usually) logical operators
```rust
fn main() {
	let x:bool = true;

	// && - AND
	let condition = true && x; // true since the same
	println!("{}", condition);
	// || - OR
	let condition_one = true || x; // true
	println!("{}", condition_one);
	// ! - NOT
	let condition_two = !(true || x); // both false 
	println!("{}", condition_two);
	let condition_three = true || !x; // either or is true and the first operand is true
	println!("{}", condition_three);
}
```

---

- if/else if/else
```rust
fn main() {
let letter: char = 'a';

if letter == 'a' {println!("this happened");}
else if letter != 'b' {println!("this didn't happen");}
else {println!("this did not happen");}}
```