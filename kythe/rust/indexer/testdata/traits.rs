//- ModuleAnchor.node/kind anchor
//- ModuleAnchor.loc/start 0
//- ModuleAnchor.loc/end 0
//- ModuleAnchor defines/implicit Module

//- @Trait defines/binding Trait
//- Trait childof Module
//- Trait.node/kind interface
trait Trait {}

fn main() {}

//- @Trait ref Trait
fn f(_: &impl Trait) {}