use std::fmt;
/*
 * A struct, is like a class in Java
 * a struct defines something, or
 * a named type to which you can assign state (attributes and fields) and behavior
 * (methods and functions)
 * if we want to make a book stuct, we can do it the following way
*/
struct Book {
    id: u8,
    name: String,
    author: String,
    love_reactions: i8,
}

/*
 * Now, to implement a method, we'll do the next thing,
 * also, the first parameter in a method is always **self**,
 * which represents the instance on which the method is being invoked (just like in python)
*/

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\n Id: {} \n Name: {} \n Author: {} \n Positive reactions: {}",
            self.id, self.name, self.author, self.love_reactions,
        )
    }
}

impl Book {
    fn new_book(id: u8, name: String, author: String, love_reactions: i8) -> Book {
        if love_reactions < 0 {
            eprint!("Reactions can't be smaller than 0 :(");
            std::process::abort();
        }
        Book {
            id,
            name,
            author,
            love_reactions,
        }
    }

    fn loved_or_not(&self) -> bool {
        if self.love_reactions >= 20 {
            true;
        }
        false
    }

    fn book_status_string(&self) -> String {
        let dislike_message: String = String::from(format!(
            "This book is not very liked, it has less than 10 reactions :("
        ));
        let like_message: String = String::from(format!(
            "This book is liked!, it has {} reactions :)",
            self.love_reactions,
        ));
        let love_message: String = String::from(format!(
            "Wow, this book is on fire, it has {} reactions :0",
            self.love_reactions,
        ));

        if self.love_reactions <= 10 {
            dislike_message
        } else if self.love_reactions > 10 && self.love_reactions <= 20 {
            like_message
        } else {
            love_message
        }
    }
}

fn main() {
    let book_1 = Book::new_book(
        1,
        String::from("Por dónde se sale"),
        String::from("Anabel Gonzalez"),
        25,
    );

    println!("{}", book_1);
    //println!("Loved or not? {}", book_1.loved_or_not());
    println!("Loved or not with string: {}", book_1.book_status_string());
}
