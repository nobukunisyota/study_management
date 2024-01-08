use std::io;

fn main() {
    println!("please input title !");
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");

}
