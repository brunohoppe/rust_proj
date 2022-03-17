fn main() {
    // let name = String::from("hello");
    // let a = name;
    // name = foo(a);
    // foo(name); // used after move
    // bar(&name); // borrow of moved value
    // println!("{}", name); // borrow of moved value
    // let mut hello = String::from("hello");
    // let c = &hello;
    // let b = &mut hello; // cannot borrow because it also immutable
    // let c = &mut hello; // cannot borrow more than once at time
    // println!("{}",c);
    // println!("{}", hello);

}
fn make_vec() -> Vec<i32> {
    let vec = vec![0,1];
    vec
}
fn print_vec(vec: Vec<i32>) {
    for i in vec.iter() {
        println!("{}", i);
    }
}
fn foo(name: String) -> String{
    println!("{}", name);
    name
}
fn bar(name: &String) {
    println!("{}", name);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}