//- ModuleAnchor.node/kind anchor
//- ModuleAnchor.loc/start 0
//- ModuleAnchor.loc/end 0
//- ModuleAnchor defines/implicit Module

//- @add defines/binding AddMacro
//- AddMacro.node/kind macro
//- AddMacro childof Module
macro_rules! add {
    ($a:expr,$b:expr) => {{ $a + $b }};
}

fn main() {
    //- @add ref/expands AddMacro
    add!(1, 2);
}
