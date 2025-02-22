fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    // This is safe because z is not modified and there is only one mutable reference
    *y += 1;
    println!("x: {}", x); //Output: 6
    println!("y: {}", *y); //Output: 6
    println!("z: {}", *z); //Output: 6

    //However, this will cause a compile-time error
    //This is because we have two mutable references to x at the same time
    // let y2 = &mut x;
    //*y2 += 1;
}