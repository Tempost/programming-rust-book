// Box<T> is a pointer to a value of type T stored on the heap.
// let box = Box::new(()) here box is just a point to the memory in heap land
// the memory is dropped when leaving the scope aka {inside here}
fn main() {
    println!("Hello, world!");
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    // let t = s; // Value moved in this exmaple
    // let u = s; here the compiler would throw, since s was moved into t already and is
    // uninitalized. instead we can .clone() the value
    let t = s.clone();
    let u = s.clone();
}
