fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1; 
    }
    println!("x: {}", x); //Output: 6

    let z = &x; // z is an immutable reference to x
    println!("z: {}", *z); //Output: 6
    //Modifying z would result in another error because the reference is immutable
    // *z += 1;
} 