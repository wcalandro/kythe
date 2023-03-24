//- ModuleAnchor.node/kind anchor
//- ModuleAnchor.loc/start 0
//- ModuleAnchor.loc/end 0
//- ModuleAnchor defines/implicit Module

//- @E defines/binding Enum
//- Enum childof Module
//- Enum.node/kind sum
//- Enum.complete definition
//- Enum.subkind enum 
enum E {
    A,
}

//- @S defines/binding Struct
//- Struct childof Module
//- Struct.node/kind record
//- Struct.complete definition
//- Struct.subkind struct
struct S {
    x: i32,
}

//- @S ref Struct
impl S {}

//- @U defines/binding Union
//- Union childof Module
//- Union.node/kind record
//- Union.complete definition
//- Union.subkind union
union U {
    f1: u32,
    f2: f32,
}

fn main() {
    //- @E ref Enum
    let _x = E::A;
    //- @S ref Struct
    let _y = S { x: 1 }
    //- @U ref Union
    let _z = U { f1: 1 }
}