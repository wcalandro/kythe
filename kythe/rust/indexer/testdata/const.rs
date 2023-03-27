//- ModuleAnchor.node/kind anchor
//- ModuleAnchor.loc/start 0
//- ModuleAnchor.loc/end 0
//- ModuleAnchor defines/implicit Module

//- @Trait defines/binding Trait
trait Trait {
    //- @ONE defines/binding TraitConst
    //- TraitConst childof Trait
    //- TraitConst.node/kind constant
    const ONE: i32 = 2;

    // Uncomment once scope definition bug in fixed in rust-analyzer 
    // fn two() -> i32 {
    //     //- @ONE ref TraitConst
    //     ONE
    // }
}

//- @Struct defines/binding Struct
struct Struct {}

impl Struct {
    //- @ONE defines/binding StructConst
    //- StructConst childof Struct
    //- StructConst.node/kind constant
    const ONE: i32 = 3;

    // Uncomment once scope definition bug in fixed in rust-analyzer
    // fn three() -> i32 {
    //     //- @ONE ref StructConst
    //     ONE
    // }
}

//- @ONE defines/binding OneConst
//- OneConst childof Module
//- OneConst.node/kind constant
const ONE: i32 = 1;

fn main() {
    //- @ONE ref OneConst
    let _x = ONE + 1;
}