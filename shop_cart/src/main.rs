trait Item {
    fn name(&self) -> &str;
    fn price(&self) -> f64;
}

struct Book {
    name: String,
    price: f64,
}
struct Pen {
    name: String,
    price: f64,
}
struct Cable {
    name: String,
    price: f64,
}

impl Item for Book {
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> f64 {
        self.price
    }
}
impl Item for Pen {
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> f64 {
        self.price
    }
}
impl Item for Cable {
    fn name(&self) -> &str {
        &self.name
    }
    fn price(&self) -> f64 {
        self.price
    }
}

struct Cart<T: Item> {
    items: Vec<T>,
}
impl<T: Item> Cart<T> {
    fn new() -> Self {
        Cart { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    fn count(&self) -> usize {
        self.items.len()
    }
    fn total(&self) -> f64 {
        let mut total = 0.0;
        for item in &self.items {
            total += item.price();
        }
        total
    }
}

fn main() {
    let item1 = Book {
        name: "clean Code".to_string(),
        price: 33.20,
    };
    let item2: Book = Book {
        name: "The Rust Book".to_string(),
        price: 49.0,
    };

    let item3: Pen = Pen {
        name: "red pen".to_string(),
        price: 3.20,
    };
    let item4: Pen = Pen {
        name: "blue pen".to_string(),
        price: 3.20,
    };

    let mut books: Cart<Book> = Cart::new();

    let mut pens: Cart<Pen> = Cart::new();

    books.add(item1);
    books.add(item2);
    pens.add(item3);
    pens.add(item4);

    println!("books:{} items  total :{}", books.count(), books.total());

    for item in &books.items {
        println!("-{}", item.name());
    }

    println!("pens:{} items  total :{}", pens.count(), pens.total());
    for item in &pens.items {
        println!("-{}", item.name());
    }
    println!(
        "everything: {} + {} = {}",
        books.total(),
        pens.total(),
        books.total() + pens.total()
    )
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_empty_cart() {
        let cart1: Cart<Book> = Cart::new();
        assert_eq!(cart1.count(), 0);
        assert_eq!(cart1.total(), 0.0);
    }
}
