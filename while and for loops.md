```rust
// while
fn main() {
        let mut counter = 0;
        let condition:bool = true;

        while counter != 150 {
                counter = counter + 1;
                println!("{}", counter);
        }
}
```

```rust
// for loops
fn divide(a:f64,b:f64) -> f64 {
        return a/b;
}

fn main() {
        let mut counter = 150;
        let mut adder_one:f64 = 8.0;
        let mut adder_two:f64 = 8.0;

        for i in 0..counter {
                let value:f64 = divide(adder_one, adder_two);
                println!("VALUE {}", value);
                adder_one = adder_one + 8.0;
                adder_two = adder_two + 8.0;
                println!("one: {}   |   two: {}", adder_one, adder_two);
        }
}

```