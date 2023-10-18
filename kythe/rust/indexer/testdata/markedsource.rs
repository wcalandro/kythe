#[feature(trait_alias)]

/// MODULE

//- @pub_module defines/binding PubModule
//- PubModule.code/rendered/signature "pub mod pub_module"
pub mod pub_module {}

//- @module defines/binding Module
//- Module.code/rendered/signature "mod module"
mod module {}

/// CONST

//- @PUB_CONST defines/binding PubConst
//- PubConst.code/rendered/signature "pub const PUB_CONST: i32"
pub const PUB_CONST: i32 = 1;

//- @CONST defines/binding Const
//- Const.code/rendered/signature "const CONST: f32"
const CONST: f32 = 1.0;

/// STATIC

//- @PUB_STATIC defines/binding PubStatic
//- PubStatic.code/rendered/signature "pub static PUB_STATIC: i64"
pub static PUB_STATIC: i64 = 1;

//- @STATIC defines/binding Static
//- Static.code/rendered/signature "static STATIC: f64"
static STATIC: f64 = 1.0;

/// MACRO

//- @add defines/binding Macro
//- Macro.code/rendered/signature "macro_rules! add"
macro_rules! add {
    ($a:expr, $b:expr) => {{ $a + $b }};
}

/// STRUCT

//- @Struct defines/binding Struct
//- Struct.code/rendered/signature "struct Struct"
struct Struct {
    //- @field1 defines/binding Field1
    //- Field1.code/rendered/signature "field1: i32"
    field1: i32,
    //- @field2 defines/binding Field2
    //- Field2.code/rendered/signature "pub field2: f64"
    pub field2: f64,
}

//- @PubStruct defines/binding PubStruct
//- PubStruct.code/rendered/signature "pub struct PubStruct<'a, T, const U: usize>"
pub struct PubStruct<'a, T, const U: usize> {}

/// UNION

//- @Union defines/binding Union
//- Union.code/rendered/signature "union Union"
union Union {}

//- @PubUnion defines/binding PubUnion
//- PubUnion.code/rendered/signature "pub union PubUnion"
pub union PubUnion {}

/// ENUM

//- @Enum defines/binding Enum
//- Enum.code/rendered/signature "enum Enum"
enum Enum {}

//- @PubEnum defines/binding PubEnum
//- PubEnum.code/rendered/signature "pub enum PubEnum<'a, T, const U: usize>"
pub enum PubEnum<'a, T, const U: usize> {}

/// TRAIT

//- @Trait defines/binding Trait
//- Trait.code/rendered/signature "trait Trait"
trait Trait {}

//- @PubTrait defines/binding PubTrait
//- PubTrait.code/rendered/signature "pub trait PubTrait<'a, T, const U: usize>"
pub trait PubTrait<'a, T, const U: usize> {}

/// TRAIT ALIAS

//- @TraitAlias defines/binding TraitAlias
//- TraitAlias.code/rendered/signature "trait TraitAlias"
trait TraitAlias = Trait + Clone;

// NOTE: There is a bug here where if you do PubTraitAlias<'a, T> = PubTrait<'a, T, 12>,
// rust-analyzer incorrectly set's the local_id when getting the definition of T so that
// it doesn't match when you get the definition's text range.

//- @PubTraitAlias defines/binding PubTraitAlias
//- PubTraitAlias.code/rendered/signature "pub trait PubTraitAlias<'a>"
pub trait PubTraitAlias<'a> = PubTrait<'a, String, 12>;

/// TYPE ALIAS

//- @TypeAlias defines/binding TypeAlias
//- TypeAlias.code/rendered/signature "type TypeAlias"
type TypeAlias = i32;

//- @PubTypeAlias defines/binding PubTypeAlias
//- PubTypeAlias.code/rendered/signature "pub type PubTypeAlias<'a, const U: usize>"
pub type PubTypeAlias<'a, const U: usize> = PubStruct<'a, String, U>;

/// FUNCTIONS

//- @fn1 defines/binding Fn1
//- Fn1.code/rendered/signature "fn fn1()"
fn fn1() {}

//- @fn2 defines/binding Fn2
//- Fn2.code/rendered/signature "pub fn fn2()"
pub fn fn2() {}

//- @fn3 defines/binding Fn3
//- Fn3.code/rendered/signature "pub const fn fn3() -> i32"
pub const fn fn3() -> i32 {
    1
}

//- @fn4 defines/binding Fn4
//- Fn4.code/rendered/signature "unsafe fn fn4()"
unsafe fn fn4() {}

//- @fn5 defines/binding Fn5
//- Fn5.code/rendered/signature "fn fn5<T>(x: T) -> T"
fn fn5<T>(x: T) -> T {
    x
}

/// LOCALS

fn fn6() {
    //- @x defines/binding LocalX
    //- LocalX.code/rendered/signature "mut x: i32"
    let mut x = 1i32;
}
