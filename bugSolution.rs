fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    //Safe way to access elements: Check bounds before accessing
    if vec.len() > 2 {
        println!("Third element: {}", vec[2]);
    } else {
        println!("Vector does not have a third element");
    }
    //Another safe way to access elements using get method which returns an Option
    match vec.get(2) {
        Some(third) => println!("Third element: {}", third),
        None => println!("Vector does not have a third element")
    }
} 