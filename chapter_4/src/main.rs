// Box<T> is a pointer to a value of type T stored on the heap.
// let box = Box::new(()) here box is just a point to the memory in heap land
// the memory is dropped when leaving the scope aka {inside here}
fn main() {
    println!("Hello, world!");
}
