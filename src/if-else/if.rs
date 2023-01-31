//If else statements
fn ifelse() {
    //Comparison operators
    println!("==  equal to");
    println!("");
    println!("!= not equal to");
    println!("");
    println!("> greater than");
    println!("");
    println!("< less than");
    println!("");
    println!(">= greater than or equal to");
    println!("");
    println!("<= less than or equal to");

    println!("");
    println!("--------------------------");
    println!("");

    let c = 5;
    //Will tell if the value of variable is positive
    if c > 0{
        println!("c is positive");
    }

    let x = -2;
    //WIll tell if the value of variable is negative
    if x < 0{
        println!("c is negative");
    }

    //The code between the {} will be executed because the condition is true or false
    //If the value of x or c to a negative/positive nmber,the code between the {} will not be executed

    let o = -5;
    //Not executed
    if o > 0{
        println!("c is positive");
    }

    let xp = 2;
    //Not executed
    if xp < 0{
        println!("c is negative");
    }

    //Else
    if c == 5 {
        println!("Hello");
    } else {
        println!("Goodbye");
    }


    //Else if
    if c == -5 {
        println!("Hello");
    } else if c == 6 {
        println!("Good morning");
    } else {
        println!("Good night");
    }

}
