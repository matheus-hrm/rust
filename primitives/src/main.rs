fn main() {
    let logical: bool = true;
    let a_float: f64 = 1.0; // Regular annotation
    let an_integer: i64 = 5; // Suffix annotation

    println!("logical: {}", logical);
    println!("a_float: {}", a_float);
    println!("an_integer: {}", an_integer);

    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`
    
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    let mutable = 12; // Mutable `i32`
    //mutable = 21;

    let mutable = true; // Mutable `bool`

    println!("default_float: {}", default_float);
    println!("default_integer: {}", default_integer);
    println!("inferred_type: {}", inferred_type);
    println!("mutable: {}", mutable);
    println!("type of mutable: {}", type_of(mutable));
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

