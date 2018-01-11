fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!"); // Type annotation needed

    // If we don’t add the type annotation here, Rust will display the following error, 
    // which means the compiler needs more information from us to know which possible 
    // type we want to use

    // 4 type of scalar types:

    // 1. integer
    
	// Length	Signed	Unsigned
	// 8-bit	i8		u8
	// 16-bit	i16		u16
	// 32-bit	i32		u32
	// 64-bit	i64		u64
	// arch		isize	usize

	// Signed: positive only (like indexing)
	// Unsigned: positive and negative

    // 2. Floating-point

    // let x = 2.0; // f64	

    // let y: f32 = 3.0; // f32

    // Numerical Operations
    // addition
    // let sum = 5 + 10;

    // subtraction
    // let difference = 95.5 - 4.3;

    // multiplication
    // let product = 4 * 30;

    // division
    // let quotient = 56.7 / 32.2;

    // remainder
    // let remainder = 43 % 5;

    // 3. Boolean
	// let t = true;

 	// let f: bool = false; // with explicit type annotation

    // 4. Characters
    
	// let c = 'z';
	// let z = 'ℤ';
	// let heart_eyed_cat = '😻';

	// Compound types
	
	// 1. Tuples
	// Elements can be of different type
	
	// let tup: (i32, f64, u8) = (500, 6.4, 1); // type annotation optional
	// let tup = (500, 6.4, 1);

	// destructuring
    // let (x, y, z) = tup; // unused variables can be replaced with "_"

    // println!("The value of y is: {}", y);

    // let x: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching
    // let five_hundred = x.0;

    // let six_point_four = x.1;

    // let one = x.2;

	// 2. Arrays
	// Elements must be of same type
	
	// let a = [1, 2, 3, 4, 5];

	// let months = ["January", "February", "March", "April", "May", "June", "July", 
	// 	"August", "September", "October", "November", "December"];

	// let a = [1, 2, 3, 4, 5];

	// accessing array elements:
	// let first = a[0];
	// let second = a[1];
	
	// let a = [1, 2, 3, 4, 5];
	// let index = 10;

	// let element = a[index]; // causes runtime error, but does not exit

	// println!("The value of element is: {}", element);
}
