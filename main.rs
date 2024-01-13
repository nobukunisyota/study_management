struct StudyData {
    title: String,
    time: u8,
}

fn create_study_data(title: String, time: u8) -> StudyData {
    StudyData {
        title: title,
        time: time,
    }
}

fn main() {
    let mut data = create_study_data("Rust".to_string(), 10);
    println!("{}: {} hours", data.title, data.time);
}
