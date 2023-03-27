enum Enum {
    //- @A defines/binding Variant
    //- @x defines/binding VariantField
    //- VariantField childof Variant
    //- VariantField.node/kind variable
    //- VariantField.complete definition
    //- VariantField.subkind field
    A { x: i32 },
}

//- @Struct defines/binding Struct
struct Struct {
    //- @x defines/binding StructField
    //- StructField childof Struct
    //- StructField.node/kind variable
    //- StructField.complete definition
    //- StructField.subkind field
    pub x: i32,
}

//- @Union defines/binding Union
union Union {
    //- @f1 defines/binding UnionField1
    //- UnionField1 childof Union
    //- UnionField1.node/kind variable
    //- UnionField1.complete definition
    //- UnionField1.subkind field
    f1: i32,
    //- @f2 defines/binding UnionField2
    //- UnionField2 childof Union
    //- UnionField2.node/kind variable
    //- UnionField2.complete definition
    //- UnionField2.subkind field
    f2: f32,
}

fn main() {
    //- @x ref VariantField
    let a = Enum::A { x: 1 };
    //- @x ref VariantField
    if let Enum::A { x: _ } = a {}
    //- @x ref StructField
    let b = Struct { x: 1 };
    //- @x ref StructField
    let _ = b.x;
    //- @f1 ref UnionField1
    let c = Union { f1: 1 };
    //- @f1 ref UnionField1
    let _ = c.f1;
}