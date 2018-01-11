// https://doc.rust-lang.org/book/first-edition/variable-bindings.html
fn main() {
    let x: i32 = 17;

    {
        let y: i32 = 3;
        println!("The value of x is {} and the value of y is {}", x, y);
    }

    println!("The value of x is {} and the value of y is {}", x, y);
}
