fn scalar_types() {
	let name: i32 = 0;     // signed integer 32-bits
	let name_one: u32 = 0; // unsigned integer 32-bits
	let one: bool = true;  // true 
	let two: bool = false; // false
	let letter: char = 'A'; // single character must be within single qoutes
	let fp: f64 = 10.5; // double-precision | default is also f64 if not type-casted and the value is a floating-point, single-precision is `f32`
}

fn compound_types() {
	// tuples:
	let point = (1,2,3);
	println!("{} {} {}", point.0, point.1, point.2);
	
	// tuples manually defined types
	let rgba: (u8, i8, i16, i32, i64, f32,) = (100,10,25,30,50,0.5); 
	println!("{} {} {} {} {} {}", rgba.0,rgba.1,rgba.2,rgba.3,rgba.4,rgba.5);

	// immutable arrays:
	let arr = [1,2,3,4,5]; // index(s): 0=1,1=2,2=3,3=4,4=5
	println!("index 0: {}", arr[0]);

	// mutable arrays
    let mut m_h:[i32;5]=[9,3,2,8,1];
    for (i,h) in m_h.iter_mut().enumerate(){
        *h = *h+1;
        println!("{}{}", i, h);
    }
    
    // another way to iterate
    for i in 0..m_h.len() {
	    m_h[i]=m_h[i]+1;
	    println!("{}}",m_h[i]);
    }
}

fn main() {
	scalar_types();
	compound_types();
}
