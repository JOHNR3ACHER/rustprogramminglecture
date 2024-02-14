

fn main(){
    use std::time::{SystemTime, Duration};
    use chrono::{DateTime, Utc, TimeZone};

    enum BookStatus {
        Available,
        CheckedOut(SystemTime), // Due date
        InRepair(String), // Notes on the repair
    }
    
    struct Book {
        title: String,
        status: BookStatus,
    }
    

    fn display_book_status(book: &Book) {
        match &book.status {
            BookStatus::Available => println!("{} is available for borrowing.", book.title),
            BookStatus::CheckedOut(due_date) => {
                // Convert SystemTime to DateTime<Utc>
                let datetime: DateTime<Utc> = due_date.clone().into();
                // Format the DateTime<Utc> to a string
                let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                println!("{} is checked out. Due date: {}", book.title, formatted_date);
            },
            BookStatus::InRepair(notes) => println!("{} is in repair. Notes: {}", book.title, notes),
        }
    }

    
    let due_date = SystemTime::now() + Duration::from_secs(60 * 60 * 24 * 14); // 14 days from now

    println!("{:?}", due_date);

    let book = Book {
        title: String::from("Rust Programming"),
        status: BookStatus::CheckedOut(due_date),
    };

    display_book_status(&book);
    

    }


fn reading_from_console(){
    use std::io;

    println!("What's your favorite city?");

    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Failed to read line");

    println!("Your favorite city is: {}", city.trim());

}
