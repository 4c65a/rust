
//Variables
fn variables() {
    //Declaring a variables
    println!("----Declaring a variables----");
    let a = 5;

    //Printing a variables
    println!("How much is the variable a: {}", a);

    //Modify a variables
    println!("----Modify a variables----");
    
    //The default variable is immutable to make it work we have to change it to a mutable variable with mut
    let mut b = 6;
    b = 10;
    println!("The value of the variable was 6 now is: {}", b);

}
