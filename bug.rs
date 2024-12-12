fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    //Potential error here: Accessing index out of bounds
    println!("Third element: {}", vec[2]);
}