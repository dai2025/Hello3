use std::io::{self, Write};

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "Adding {} and {}", a, b).unwrap();
    let result = a + b;
    writeln!(handle, "Result: {}", result).unwrap();
    result
}

#[no_mangle]
pub extern "C" fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub2() -> i32 {
    -1
}

/*
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;

struct Class {
name: String,
gc: RefCell<Vec<Weak<String>>>,
}

//#[allow(dead_code)]
fn println_gc(class: &Class) {
println!("In {} class:", class.name);
for weak_name in class.gc.borrow().iter() {
if let Some(name) = weak_name.upgrade() {
println!("{}", name);
} else {
println!("unavailable");
}
}
}

fn print_gc(class: &Class) {
print!("{} ", class.name);
for weak_name in class.gc.borrow().iter() {
if let Some(name) = weak_name.upgrade() {
print!("{} ", name);
} else {
print!("unavailable");
}
}
}

fn main() {
let world = Rc::new("world!".to_string());
let three = Rc::new("3!".to_string());
let class : Rc<Class> = Rc::new(
Class {
name: String::from("Hello"),
gc: RefCell::new(vec![]),
}
);
class.gc.borrow_mut().push(Rc::downgrade(&world));
class.gc.borrow_mut().push(Rc::downgrade(&three));
//println!("{} {:?}", class1.name, class1);
println!("{}", class.name);
println_gc(&class);
class.gc.borrow_mut().pop();
print_gc(&class);
}
*/

