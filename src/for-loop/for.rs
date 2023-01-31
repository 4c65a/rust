//For loop
fn for(){
    //For loops are used to iterate over a given sequence.
    for o in 0..5{
        println!("{}", o);
    }

    println!("");
    println!("---------------------------");
    println!("");
    

    let range = 0..2;

    for count in range {
    println!("{}", count);
    }   

    //Vector is a group of values that can be iterated over

    let animals = vec!["monkey", "Dog", "Cats", "Unicorn"];
    //Vector can be declared with the vec!

    println!("");
    println!("---------------------------");
    println!("");

    //Iterating over a vector

    for animals in animals.iter(){
        println!("My favorite animal is the {animals}");
    }
    /*
    We use the iter() method to get an iterator over the vector and to 
    prevent the ownership of the vector from being moved and being able to use it after the loop
    */

    println!("");
    println!("---------------------------");
    println!("");

    //Iterating over a vector with index
    //We can do that with the enumerate() method

    let user = vec!["Leandro", "Alexis", "Valentina", "Gigi" ,"Liu"];

    for (index, user) in user.iter().enumerate(){
        println!("User {} at index {}", user, index);
    }
    //We use (index, number) because the enumerate() method returns with the index and the value.


}
