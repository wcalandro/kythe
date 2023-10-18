//- @Struct defines/binding Struct
struct Struct;
//- @Trait defines/binding Trait
trait Trait {}
//- @Enum defines/binding Enum
enum Enum {}
//- @Union defines/binding Union
union Union {__: ()}
//- @Type defines/binding Type
type Type = ();
//- @module defines/binding Module
mod module {}
//- @function defines/binding Function
fn function() {}
//- @CONST defines/binding Const
const CONST: () = ();
//- @STATIC defines/binding Static
static STATIC: () = ();

//- @+2"doc1" defines/binding Doc1Fn
/// [`Struct`] [`Trait`] [`Enum`] [`Union`] [`Type`] [`module`] [`function`] [`CONST`] [`STATIC`]
fn doc1() {}
//- Doc1 documents Doc1Fn
//- Doc1 param.0 Struct
//- Doc1 param.1 Trait
//- Doc1 param.2 Enum
//- Doc1 param.3 Union
//- Doc1 param.4 Type
//- Doc1 param.5 Module
//- Doc1 param.6 Function
//- Doc1 param.7 Const
//- Doc1 param.8 Static

//- @+5"Trait2" defines/binding Trait2
//- @"[`Trait2::Type`]" ref/doc Trait2Type
//- @"[`Trait2::CONST`]" ref/doc Trait2Const
//- @"[`function`](Trait2::function)" ref/doc Trait2Function
/// [`Trait2::Type`] [`invalid`] [`Trait2::CONST`] [`function`](Trait2::function)
trait Trait2 {
    //- @Type defines/binding Trait2Type
    type Type;
    //- @CONST defines/binding Trait2Const
    const CONST: usize;
    //- @function defines/binding Trait2Function
    fn function();
}
//- Doc2 documents Trait2
//- Doc2 param.0 Trait2Type
//- Doc2 param.1 Trait2Const
//- Doc2 param.2 Trait2Function
//- Doc2.text "[`Trait2::Type`] `invalid` [`Trait2::CONST`] [`function`]"
