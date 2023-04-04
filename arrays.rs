// enumerate() returns a tuple: (index, value)
// iter_mut() used for iterating over mutable arrays
// iter() used for interating over immutable arrays
// * derfrences a variable

fn main() {
	// static array
	let arr: [i32; 4] = [1334,1335,1336,1337];
	for (i,h) in arr.iter().enumerate() {println!("index {} value {}",i, h);}
	// create a mutable array of 5 elements
	let mut m_h:[i32;5]=[9,3,2,8,1];
	for (i,h) in m_h.iter_mut().enumerate(){ *h = *h+1; println!("index {} value {}", i, h); }
}


