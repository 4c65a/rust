

fn tuples(){
    //A tuple is a collection of values that can be of different types.
    //Tuples are usually used to return multiple values from a function.
    //Tuples can contain other tuples.

    let user = ("Alexis", "Lean", "Liu", "Ismael");

    let (a, b, c, d) = user;

    println!("{},{},{},{}", a, b, c, d);

    println!("");
    println!("---------------------------");
    println!("");

    //Tuples index
    println!("{}",user.0);
}
