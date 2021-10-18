enum Movement{
    Up,
    Down,
    Left,
    Right
}
fn move_avatar(m:Movement){
     //perform action depending on info
     match m{
         Movement::Up=>println!("Avatart Moving Up"),
         Movement::Down=>println!("Avatart Moving Down"),
         Movement::Left=>println!("Avatart Moving Left"),
         Movement::Right=>println!("Avatart Moving Right"),

     }
}
pub fn run(){
    let avatar1= Movement::Left;
    let avatar2= Movement::Right;
    let avatar3= Movement::Down;
    let avatar4= Movement::Up;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}