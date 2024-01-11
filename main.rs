use std::io;

struct Content {
    title: String,
    content: String,
    time: String,
}
enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn create_content(title: String, content: String, time: String) -> Content {
    Content {
        title,
        content,
        time,
    }
}

fn input(mut content: String) -> String {
    io::stdin()
        .read_line(&mut content)
        .expect("Failed to read");
    content
}

fn main() {
    let mut title = String::new();
    let mut content = String::new();
    let mut time = String::new();
    title = input(title);
    content = input(content);
    time = input(time);
    create_content(title, content, time);
}
