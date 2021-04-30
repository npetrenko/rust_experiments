// lib1:
trait Foo {
    fn foo(&self);
}

trait Bar {
    fn bar(&self);
}

// lib2:
// knows nothing about Foo and Bar
trait FooBar {
    fn foo(&self);
    fn bar(&self);
}

// lib3:
// some third-party code to relate FooBar with Foo, Bar:
// "every T that implements both Foo and Bar is a FooBar"
impl<T: Foo + Bar> FooBar for T {
    fn foo(&self) {
        (self as &dyn Foo).foo()
    }

    fn bar(&self) {
        (self as &dyn Bar).bar()
    }
}

// take a reference to T and return reference to FooBar
fn as_foobar<T: Foo + Bar>(x: &T) -> &dyn FooBar {
    x
}

// take a Box<T> and return Box<FooBar>
// Box<T> is like std::unique_ptr<T>
// 'a is a lifetime of a type, and this "cast" preserves it (don't think too much about it
fn as_foobar_box<'a, T: Foo + Bar + 'a>(x: Box<T>) -> Box<dyn FooBar + 'a> {
    x
}





// user code:
struct MyFooBar {
}

impl Foo for MyFooBar {
    fn foo(&self) {
        println!("foo");
    }

}

impl Bar for MyFooBar {
    fn bar(&self) {
        println!("bar");
    }
}

fn my_main() {
    // explicit type annotation is just for clarity
    let x : Box<MyFooBar> = Box::new(MyFooBar{});
    // equilvalently:
    // let dynamic : Box<dyn FooBar> = x;
    let dynamic = as_foobar_box(x);
    let dynamic_explicitw: Box<dyn FooBar> = dynamic;
    dynamic_explicit.foo();
    dynamic_explicit.bar();
}
