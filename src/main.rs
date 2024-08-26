use dyn_clone::{clone_box, DynClone};
use std::{marker::PhantomData, sync::Arc};

trait MyTrait: DynClone {}

#[derive(Clone)]
struct MyStruct {}

impl MyTrait for MyStruct {}
impl MyTrait for Box<dyn MyTrait> {}

impl<T: MyTrait + Clone> MyTrait for Box<T> {}
impl<T: MyTrait + Clone> MyTrait for &T {}
impl MyTrait for &dyn MyTrait {}
dyn_clone::clone_trait_object!(MyTrait);

struct MyThing {
    owned: Box<dyn MyTrait>,
}

struct MyBox<T>(T);

impl MyThing {
    pub fn new(thing: impl MyTrait) -> Self {
        let thing: Box<dyn MyTrait> = clone_box(&thing);
        MyThing { owned: thing }
    }
}

fn main() {
    println!("Hello, world!");

    // These should both work
    let my_struct = MyStruct {};
    let _ = MyThing::new(&my_struct);
    let _ = MyThing::new(my_struct);
}
