// Mutability
// Variable declarations are by default immutable
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6; // because of "mut", you can change the value
//     println!("The value of x is: {}", x);
// }

// Shadowing
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

// The other difference between mut and shadowing is that 
// because we’re effectively creating a new variable when 
// we use the let keyword again, we can change the type 
// of the value, but reuse the same name. 