fn main() {
    // string
    let rust = "Rust"; // &str is fixed, actually a &[8]
    println!("Hello, {}", rust);

    // String is dynamically allocated in heap
    let mut string = rust.to_string();
    string.insert(0, 'c');
    assert_eq!("cRust", string);

    // array, fixed size
    let mut array: [i32; 3] = [0; 3]; // = [0, 0, 0] initialization
    for x in array {
        println!("{} ", x);
    }

    array[1] = 1;
    array[2] = 2;
    assert_eq!([1, 2], &array[1..]);

    // vector, dynamically allocated in heap
    // String is actually a vec:Vec<u8>

    // A tuple struct’s constructors can be used as functions.
    // Elements in a tuple struct don't have names
    struct Digit(i32);
    let v = vec![0, 1, 2];
    let d: Vec<Digit> = v.into_iter().map(Digit).collect();
    for x in d {
        println!("{} ", x.0);
    }

    // enum
    pub(crate) enum Message {
        Quit,
        Test,
        Change,
    }
}
