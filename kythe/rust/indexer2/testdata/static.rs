//- ModuleAnchor.node/kind anchor
//- ModuleAnchor.loc/start 0
//- ModuleAnchor.loc/end 0
//- ModuleAnchor defines/implicit Module

//- @ONE defines/binding OneStatic
//- OneStatic childof Module
//- OneStatic.node/kind variable
//- OneStatic.complete definition
static ONE: i32 = 1;

fn main() {
    //- @ONE ref OneStatic
    let _x = ONE + 1;

    //- @STATIC_VAR defines/binding StaticVar1
    //- StaticVar1 childof Module
    //- StaticVar1.node/kind variable
    //- StaticVar1.complete definition
    static STATIC_VAR: i32 = 1;

    //- @STATIC_VAR ref StaticVar1
    //- !{ @STATIC_VAR ref StaticVar2 }
    let _y = STATIC_VAR;
}

fn main2() {
    //- @STATIC_VAR defines/binding StaticVar2
    //- StaticVar2 childof Module
    //- StaticVar2.node/kind variable
    //- StaticVar2.complete definition
    static STATIC_VAR: i32 = 1;
    
    //- @STATIC_VAR ref StaticVar2
    //- !{ @STATIC_VAR ref StaticVar1 }
    let _y = STATIC_VAR;
}

extern "C" {
    //- @VAR defines/binding ExternStatic
    //- ExternStatic childof Module
    //- ExternStatic.node/kind variable
    //- ExternStatic.complete incomplete
    static mut VAR: i32;
}