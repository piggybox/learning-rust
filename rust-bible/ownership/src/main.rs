// Rust ownership takeaways:
// - Every value needs to have an owner
// - Every value can only have one owner
// - Whenever the owner leaves the scope, the value will be dropped
// - Stack and heap exhibit different copy behaviors

fn main() {
    let s = "hello"; // static compiled string, immutable

    let mut t = String::from("hello"); // String is dynamically allocated on heap, thus expandable
    t.push_str(", world!");

    let t2 = t; // this is called ownership "move"
    println!("{}", t); // error: borrow of moved value: `t`
}
