pub fn run(){
    let age = 21;
    let check_id: bool = false;
    //if/else
    if age>= 21  && check_id{
        println!("whats your preferences?");
    } else if age <21 && check_id {
        println!("Sorry! your are not allowed here:)");
    }
    else{
        println!("I'll have to check your id.")
    }
    // shorthand if
    let is_of_age = if age>=21 {true} else {false};
    println!("Is_of_age: {}",is_of_age);
}