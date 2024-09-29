// Vectors are a resizable array of elements allocated on the heap.
fn main() {
    // Using the vec! macro
    let mut v1 = vec![1, 2, 3, 4];
    
    // Using Vec::new
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    
    v1.push(5);
    println!("{:?}", v1);
    println!("{:?}", v2);

    v2.pop();
    println!("{:?}", v2);

    v2.insert(0, 0);
    println!("{:?}", v2);

    v2.remove(0);
    println!("{:?}", v2);

    v1.reverse();
    println!("{:?}", v1);

    v1.clear();
    println!("{:?}", v1);

    // To save reversed vector in a new variable to not change original 
    let mut v2_reversed: Vec<i32> = v2.clone();
    v2_reversed.reverse();
    println!("{:?}", v2_reversed);

    // Setting vector capacity on creation
    let v3: Vec<i32> = Vec::with_capacity(2); // prints []
    println!("{}", v3.capacity()); // Returns the total number of elements the vector can hold without reallocating to a new memory buffer. When the vector grows past this point it allocates a larger buffer and copy the contents into this new larger buffer and then it updates our vector variable to point to the new buffer before it frees the old vector from memory.

    // Creating vector using iterator
    let v4: Vec<i32> = (-1..5).collect();
    print!("{:?}", v4);

    // Loop/Iterate through vector
    for value in v4.iter() {
        println!("{}", value);
    }

    // Return number of elements in the vector
    println!("Length: {}", v4.len());

    // Returns true if vector is empty
    println!("Is empty: {}", v4.is_empty());

    println!("Capacity: {}", v4.capacity());

    let mut v5: Vec<i32> = vec![9, 8, 6, 5]; 

    // split vector at a position
    let v5_split: Vec<i32> = v5.split_off(1);
    v5.retain(|&x| x % 2 == 0); // Retains only the elements specified by the predicate
    println!("{:?}", v5_split);

}
