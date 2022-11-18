// use std::rc::Rc;

// fn main() {
//     println!("Hello, world!");
//     let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
//     // let t = s; // Value moved in this exmaple
//     // let u = s; here the compiler would throw, since s was moved into t already and is
//     // uninitalized. instead we can .clone() the value
//     let t = s.clone();
//     let u = s.clone();

//     let rc: Rc<String> = Rc::new("hello rc".to_string());
//     println!("{}", rc);
//     let rt = rc.clone();
//     println!("{}", rt);
//     println!("rc reffed: {} times", Rc::strong_count(&rc));
// }

/*
* NOTES:
* Box<T> is a pointer to a value of type T stored on the heap.
* let box = Box::new(()) here box is just a point to the memory in heap land
* the memory is dropped when leaving the scope aka {inside here}
*
* ## Values are IMMUTABLE, rust assumes value might be shared
* Arc<> - Atomic Ref count, if passing values between threads
* Rc<> - Ref count, if cloning/copying values within scope(single thread)
*/

use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    // If we didn't pass table by ref this outer for loop would take ownership
    // of the table variable, causing the memory to be dropped at the end
    for (artist, works) in table {
        println!("works by: {}", artist);
        for work in works {
            println!("\t{}", work)
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    // Short hand for
    // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Options<&'a String>
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn main() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    // instead of moving the Table into the show func
    // we can pass a ref, allowing us the print the HashMap on the next line
    show(&table);
    sort_works(&mut table);
    show(&table);
}
