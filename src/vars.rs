//variables are mutabkle, rust is block- scoped language

pub fn rubn(){
    let name= "Ritwik";
    let mut age= 37; //to mutable vars
    println!("My name is {} and I am {} years old", name, age);
    age= 40;
    println!("My name is {} and I am {} years old", name, age);
    //define constant
    const ID: i32=001;
    println!("ID: {}",ID);
    //assign multiple variables
    let (my_name,my_age)=("Ansh",20);
    println!("{} is {}", my_name,my_age);
}