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
