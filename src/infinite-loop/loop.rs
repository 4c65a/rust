//Infinite loop
fn loop() {
    //Loop keyword to print hello infinitely
   /*
   loop{
    println!("Hello");
   }
   */

   //So tha it is not infinite, the break keyword allows us to stop the execution of a loop
   let mut count = 0;

    /*loop {

    println!("Hello");

    count = count + 1;

    if count == 10 {
        break;
        }
    }
    */

    /*
    We add 1 to the variable count each
     time we print hello, and then when 
     count is equal to 5, we stop the 
     execution of the loop.
    */

    //The continue
    loop {
    count += 1;

    if count == 2 {
        continue;
    }

    println!("Hello, user{}", count);

    if count == 5 {
        break;
    }
}
}
