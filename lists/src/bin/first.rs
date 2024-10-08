#[derive(Debug)] // Derive the Debug trait to enable printing
enum List<T> {
    Cons(T, Box<List<T>>), // Corrected 't' to 'T'
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}