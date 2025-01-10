fn main() {
    let mut x = 5;
    { // Creating a scope to limit the lifetime of the mutable borrow
        let y = &mut x;
        *y += 1; 
    }
    { // Creating another scope to limit the lifetime of another mutable borrow
        let z = &mut x; 
        *z += 1; 
    }
    println!("x = {}", x);
}