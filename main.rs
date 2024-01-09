use std::io;

fn main() {
    loop {
        println!("please input title !");
        let title = input_title();
        if title.trim() == "exit" {
            break;
        }
        println!("title is {}", title);
    }
}

fn input_title() -> String {
    let mut title = String::new();
    io::stdin()
        .read_line(&mut title)
        .expect("Failed to read line");
    title
}
