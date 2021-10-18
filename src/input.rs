use std:: io;
pub fn run(){
    let mut line = String::new();
   println!("Enter your name :");
   let b1 = io::stdin().read_line(&mut line).unwrap();
   println!("Hello , {}", line);
   println!("no of bytes read , {}", b1)
}