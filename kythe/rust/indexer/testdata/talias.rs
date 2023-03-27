//- ModuleAnchor.node/kind anchor
//- ModuleAnchor.loc/start 0
//- ModuleAnchor.loc/end 0
//- ModuleAnchor defines/implicit Module

//- @Trait defines/binding Trait
trait Trait {
    //- @Int defines/binding TraitTAlias
    //- TraitTAlias childof Trait
    //- TraitTAlias.node/kind talias
    type Int = i64;

    // Uncomment once scope definition bug in fixed in rust-analyzer
    // //- @Int ref TraitTAlias
    // fn random() -> Int;
}

//- @Struct defines/binding Struct
struct Struct {}

impl Struct {
    //- @Int defines/binding StructTAlias
    //- StructTAlias childof Struct
    //- StructTAlias.node/kind talias
    type Int = f32;

    // Uncomment once scope definition bug in fixed in rust-analyzer
    // //- @Int ref StructTAlias
    // fn woah_thats_not_an_integer() -> Int {
    //     1.0
    // }
}

//- @Int defines/binding ModuleTAlias
//- ModuleTAlias childof Module
//- ModuleTAlias.node/kind talias
type Int = i32;

fn main() {
    //- @Int ref ModuleTAlias
    let _x: Int = 1;
}