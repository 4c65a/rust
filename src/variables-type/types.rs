//Variable data types
fn types() {
    /*
        signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
        unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
        floating point: f32, f64
        char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
        bool either true or false
        and the unit type (), whose only possible value is an empty tuple: ()   
    */
    let a: u64 = 81;
    let l: i64 = -51;
    let k: f32 = 31.3;
    let n: char = 'b';
    let j: bool = true;

    println!("a = {}", a);
    println!("l = {}", l);
    println!("k = {}", k);
    println!("n = {}", n);
    println!("j = {}", j);
    /*We dont need to specify it is type like in other languages like java or c ,
    which means that rust will automatically determine the type of the variable.
    */
    let g = "Type";
    let c = 32;
    let t = true;

    println!("g = {}", g);
    println!("c = {}", c);
    println!("t = {}", t);

}
