fn main() {
	// another_function();
    // print_number(5);
    // print_sum(5, 6);

    // statements_and_expressions();
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

// Rust uses _snake case_, meaning all lower case, underscores for spaces
// fn another_function() {
//     println!("Another function.");
// }

// In function signatures, you must declare the type of each parameter. 
// fn print_number(x: i32) {
//     println!("x is: {}", x);
// }

// fn print_sum(x: i32, y: i32) {
//     println!("sum is: {}", x + y);
// }

// fn statements_and_expressions() {
// 	// Illegal:
//     // let x = (let y = 6);

//     // The let y = 6 statement does not return a value, 
//     // so there isn’t anything for x to bind to. 
//     // This is different than in other languages, such as C and Ruby, 
//     // where the assignment returns the value of the assignment. 
//     // In those languages, you can write x = y = 6 
//     // and have both x and y have the value 6; 
//     // that is not the case in Rust.
    
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1 
//         // If you add a semicolon to the end of an expression, 
//         // you turn it into a statement, which will then not return a value
//     };

//     println!("The value of y is: {}", y);
// }

// We don’t name return values, but we do declare their type after an arrow (`->`)
fn plus_one(x: i32) -> i32 {
    x + 1 // no semi-colon on returns
}
