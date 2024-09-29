fn main() {
    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let slice_vector: &[i32] = &v[2..4];    // Non-owning Reference. Points to group of elements as opposed to one
    println!("{:?}", slice_vector);
}
