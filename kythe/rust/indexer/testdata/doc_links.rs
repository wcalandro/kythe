//- @Greeter defines/binding Greeter
struct Greeter {}

impl Greeter {
    //- @hello_world defines/binding HelloWorld
    pub fn hello_world() {
        println!("Hello, World!");
    }
}

//- @+2"call_greeter" defines/binding CallFn
/// Uses [`Greeter`] and calls [`hello_world`](Greeter::hello_world())
fn call_greeter() {
    Greeter::hello_world();
}
//- CallFnDoc documents CallFn
//- CallFnDoc param.0 Greeter
//- CallFnDoc param.1 HelloWorld
