#[derive(Debug)]

enum Animal{
    Monkey,
    Dog,
    Unicorn,
}

fn enum() {
    //let animal:Animal = Animal::Monkey;
    //println!("{:?}",animal);
   
    match animal {
        Animal::Monkey => println!(""),
        Animal::Dog => println!(""),
        Animal::Unicorn => println!(""),
        _ => println!("🤖"),
    }

}


