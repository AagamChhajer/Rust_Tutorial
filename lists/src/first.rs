pub mod lib;

enum List<T> {
    Cons(T, Box<List<t>>),
    Nil,
}

fn main(){
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}