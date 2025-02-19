fn main() {
    let vec = vec![1, 2, 3];

    // Use a for loop to iterate over the vector
    for element in &vec {
        println!("Element: {:?}", element);
    }

    // Alternatively, clone the vector to iterate multiple times
    let vec_clone = vec.clone();
    let mut iter = vec_clone.iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());
    println!("Third element: {:?}", iter.next());
}