fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");

    let a = String::from("Hello");
    let b = "Hi";
    let res = longest(a.as_str(), b);
    println!("Longer {}", res);

    let n_list = vec![32, 54, 32, 101, 44];
    let res = largest(&n_list);
    println!("Largest element of {:?} is {}", n_list, res);

    let c_list = vec!['a', 'd', 'e', 'f'];
    let res = largest(&c_list);
    println!("Largest element of {:?} is {}", c_list, res);

    pts();
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more)") //default implementation
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Summary for Point<T> {}

fn pts() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    //let wontwork = Point { x: 5, y: 1.0};

    println!("p.x = {}", integer.x());

    let art = NewsArticle {
        headline: String::from("Winind and woow"),
        location: String::from("Koeln"),
        author: String::from("jj"),
        content: String::from("Blah blabh"),
    };

    println!("THe summary is {}", art.summarize());
    println!("Summary of a point: {}", float.summarize());

    notify(&art);
    notify(&integer);
}
//fn notify(item: &impl Summary)
fn notify<T: Summary>(item: &T) {
    println!("News: {}", item.summarize());
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}
