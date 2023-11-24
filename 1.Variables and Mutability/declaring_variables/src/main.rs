fn main() {

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>());
    }

    // Initializing an i32 type variable that is immutable without the type
    let x = 4;
    println!("Variable x was declared without a type annotation, so by default it is:");
    print_type_of(&x);

    // Initializing an i32 type variable that is immutable with a type
    let y: i64 = 4;
    println!("Variable y was declared with a type annotation, so it is:");
    print_type_of(&y);

    // Initializing a float type constant variable that is always immutable
    const IMMUTABLE: f64 = 5.78;
    println!("Constant IMMUTABLE has to be declared with a type annotation, so it is:");
    print_type_of(&IMMUTABLE);
    println!("This constant can never be modified");
}
