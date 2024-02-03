fn main() {
    let args: Vec<String>= std::env::args().collect();

    //println!("{:?}",args);
    if args[1] == "add" && args.len() >2 {
        println!("Added {}",args[2])
    } else {
        println!("not enough args");
    }

}
