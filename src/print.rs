pub fn run(){
    //print from production
    println!("Hello from the print.rs file");
    println!("Number:{}",1);
    println!("{} is from {}","Ritwik", "Bakura");
    println!(
        "{0} is from {1} and {0 } like to {2}",
        "Ritwik","Bakura","Code"
    );
    //named arguments
    println!("{name} likes to {activity}",
    name= "Ritwik", activity="development" 
    );
    //placeholder traits
    println!(
        "Binary: {:b} Hex:{:x} Octal{:0}", 10,10,10

    );
    //placeholder for debug traits
    println!("{:?}", (12,true,"Hello"));
    //basic math
    println!("10+10={}",10+10);
} 