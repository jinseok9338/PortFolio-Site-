
fn main() {
    let s = String::from("hello");
    let number = 5;
    take_ownerShip(s);
    println!("{}",s)
    make_Copy(number)
}

fn take_ownerShip(some_string:String){
    println!("{}",some_string)
}

fn make_Copy(some_integer:i32){
    println!("{}",some_integer)
}