fn main() {
    let v = vec![101, 250, 350, 400];
    //Vector v owns the object in heap

    //only a single variable owns the heap memory at any given time
    let v2 = v;
    //here two variables owns heap value,
    //two pointers to the same content is not allowed in rust

    //Rust is very smart in terms of mmemory access, so it detects a race condiion
    //as two variables point to same heap

    println!("{:?}",v);
}
