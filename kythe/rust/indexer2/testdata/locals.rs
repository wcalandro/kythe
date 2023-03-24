//- @main defines/binding MainFn
fn main() {
    //- @x defines/binding VarX
    //- VarX childof MainFn
    //- VarX.node/kind variable
    //- VarX.subkind local
    let x = 1;
    //- @y defines/binding VarY
    //- VarY childof MainFn
    //- VarY.node/kind variable
    //- VarY.subkind local
    //- @x ref VarX
    let y = x + 1;
    //- @x defines/binding VarX2
    let x = 2;
    //- @x ref VarX2
    //- !{ @x ref VarX }
    let z = x;
}