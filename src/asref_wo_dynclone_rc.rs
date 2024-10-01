use std::rc::Rc;

trait MyTrait {}

#[derive(Clone)]
struct MyStruct {}

impl AsRef<MyStruct> for MyStruct {
    fn as_ref(&self) -> &MyStruct {
        self
    }
}
impl MyTrait for MyStruct {}

impl AsRef<MyStruct> for MyStruct {
    fn as_ref(&self) -> &Self {
        self
    }
}

struct MyThing {
    owned: Box<dyn MyTrait>,
}

impl MyThing {
    // This should work
    pub fn new<T: MyTrait + Clone + 'static>(thing: impl AsRef<T>) -> Self {
        let thing: &T = thing.as_ref();
        let thing: T = thing.clone();
        let thing: Box<T> = Box::new(thing);
        MyThing { owned: thing }
    }
}

fn main() {
    println!("Hello, world!");

    // These should both work
    let my_struct = MyStruct {};
    // T = &'a MyStruct
    let _ = MyThing::new(&my_struct);
    let _ = MyThing::new(my_struct);
    let _ = MyThing::new(Box::new(MyStruct {}));
    let _ = MyThing::new(Rc::new(MyStruct {}));
}
