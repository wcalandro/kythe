//- @E defines/binding Enum
enum E {
    //- @A defines/binding VariantA
    //- VariantA childof Enum
    //- VariantA.node/kind constant
    A,
    //- @B defines/binding VariantB
    //- VariantB childof Enum
    //- VariantB.node/kind record
    //- VariantB.complete definition
    //- VariantB.subkind tuplevariant
    B(i32),
    //- @C defines/binding VariantC
    //- VariantC childof Enum
    //- VariantC.node/kind record
    //- VariantC.complete definition
    //- VariantC.subkind structvariant
    C { x: i32 },
}

fn main() {
    //- @A ref VariantA
    let _x = E::A;
    //- @B ref VariantB
    let _y = E::B(1);
    //- @C ref VariantC
    let _z = E::C { x: 1};
}