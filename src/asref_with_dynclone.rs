use std::ops::Deref;

use dyn_clone::{clone_box, DynClone};

trait MyTrait: DynClone {
    fn clone_boxed(&self) -> Box<dyn MyTrait>
    where
        Self: Sized + 'static,
    {
        clone_box(&*self.clone())
    }
}

#[derive(Clone)]
struct MyStruct {}

impl AsRef<MyStruct> for MyStruct {
    fn as_ref(&self) -> &MyStruct {
        self
    }
}
impl MyTrait for MyStruct {}
impl MyTrait for Box<dyn MyTrait> {}

impl<T: MyTrait + Clone> MyTrait for Box<T> {}
impl<T: MyTrait + Clone> MyTrait for &T {}
// impl MyTrait for &dyn MyTrait {}
dyn_clone::clone_trait_object!(MyTrait);

struct MyThing {
    owned: Box<dyn MyTrait>,
}

struct MyBox<T>(T);

impl MyThing {
    // This should work
    pub fn new<T: MyTrait + 'static>(thing: impl AsRef<T>) -> Self {
        // But it still complains thing might not live long enough
        let thing = thing.as_ref().clone_boxed();
        MyThing { owned: thing }
    }
}

fn example() {
    println!("Hello, world!");

    // These should both work
    let my_struct = MyStruct {};
    // T = &'a MyStruct
    let _ = MyThing::new(&my_struct);
    let _ = MyThing::new(my_struct);
    // T = MyStruct
    // let _ = MyThing::new(my_struct);
}
