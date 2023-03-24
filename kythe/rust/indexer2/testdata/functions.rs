//- ModuleAnchor.node/kind anchor
//- ModuleAnchor.loc/start 0
//- ModuleAnchor.loc/end 0
//- ModuleAnchor defines/implicit Module

//- @Trait defines/binding Trait
trait Trait {
    //- @hello defines/binding TraitHelloFn
    //- TraitHelloFn childof Trait
    //- TraitHelloFn.node/kind function
    //- TraitHelloFn.complete definition
    fn hello() -> String {
        //- @get_string ref TraitGetStringFn
        Self::get_string()
    }

    //- @get_string defines/binding TraitGetStringFn
    //- TraitGetStringFn childof Trait
    //- TraitGetStringFn.node/kind function
    //- TraitGetStringFn.complete incomplete
    fn get_string() -> String;
}

//- @TraitStruct defines/binding TraitStruct
struct TraitStruct {}

impl Trait for TraitStruct {
    //- @get_string defines/binding TSGetStringFn
    //- TSGetStringFn childof TraitStruct
    //- TSGetStringFn.node/kind function
    //- TSGetStringFn.complete definition
    fn get_string() -> String {
        format!("Hello from TraitStruct::get_string!")
    }
}

//- @Struct defines/binding Struct
struct Struct {}

impl Struct {
    //- @hello defines/binding StructHelloFn
    //- StructHelloFn childof Struct
    //- StructHelloFn.node/kind function
    //- StructHelloFn.complete definition
    pub fn hello() {
        println!("Hello from Struct!");
    }
}

//- @Enum defines/binding Enum
enum Enum {
    A,
};

impl Enum {
    //- @hello defines/binding EnumHelloFn
    //- EnumHelloFn childof Enum
    //- EnumHelloFn.node/kind function
    //- EnumHelloFn.complete definition
    pub fn hello() {
        println!("Hello from Enum!");
    }
}

//- @Union defines/binding Union
union Union {
    f1: i32,
    f2: f32,
}

impl Union {
    //- @hello defines/binding UnionHelloFn
    //- UnionHelloFn childof Union
    //- UnionHelloFn.node/kind function
    //- UnionHelloFn.complete definition
    pub fn hello() {
        println!("Hello from Union!");
    }
}

//- @main defines/binding MainFn
//- MainFn childof Module
//- MainFn.node/kind function
//- MainFn.complete definition
fn main() {
    //- @hello ref HelloFn
    //- !{ @hello ref StructHelloFn }
    //- !{ @hello ref EnumHelloFn }
    //- !{ @hello ref UnionHelloFn }
    hello();
    
    //- @hello ref StructHelloFn
    //- !{ @hello ref HelloFn }
    //- !{ @hello ref EnumHelloFn }
    //- !{ @hello ref UnionHelloFn }
    Struct::hello();

    //- @hello ref EnumHelloFn
    //- !{ @hello ref HelloFn }
    //- !{ @hello ref StructHelloFn }
    //- !{ @hello ref UnionHelloFn }
    Enum::hello();

    //- @hello ref UnionHelloFn
    //- !{ @hello ref HelloFn }
    //- !{ @hello ref StructHelloFn }
    //- !{ @hello ref EnumHelloFn }
    Union::hello();

    //- @hello ref TraitHelloFn
    let _ = TraitStruct::hello();
    //- @get_string ref TSGetStringFn
    let _ = TraitStruct::get_string();
}

//- @hello defines/binding HelloFn
//- HelloFn childof Module
//- HelloFn.node/kind function
//- HelloFn.complete definition
fn hello() {
    println!("Hello from the module!");
}