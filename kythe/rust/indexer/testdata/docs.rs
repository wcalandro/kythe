//- @+2"module" defines/binding Module
/// This is a comment for Module
mod module {}
//- ModuleComment documents Module
//- ModuleComment.node/kind doc
//- ModuleComment.text "This is a comment for Module"

//- @+2"Enum" defines/binding Enum
/// This is a comment for Enum
enum Enum {
    //- @+2"A" defines/binding Variant
    /// This is a comment for Variant
    A,
    //- VariantComment documents Variant
    //- VariantComment.node/kind doc
    //- VariantComment.text "This is a comment for Variant"
}
//- EnumComment documents Enum
//- EnumComment.node/kind doc
//- EnumComment.text "This is a comment for Enum"

//- @+2"Struct" defines/binding Struct
/// This is a comment for Struct
struct Struct {
    //- @+2"field" defines/binding Field
    /// This is a comment for Field
    pub field: i32,
    //- FieldComment documents Field
    //- FieldComment.node/kind doc
    //- FieldComment.text "This is a comment for Field" 
}
//- StructComment documents Struct
//- StructComment.node/kind doc
//- StructComment.text "This is a comment for Struct"

//- @+2"Union" defines/binding Union
/// This is a comment for Union
union Union {}
//- UnionComment documents Union
//- UnionComment.node/kind doc
//- UnionComment.text "This is a comment for Union"

//- @+2"ONE" defines/binding Const
/// This is a comment for Const
const ONE: i32 = 1;
//- ConstComment documents Const
//- ConstComment.node/kind doc
//- ConstComment.text "This is a comment for Const"

//- @+2"function" defines/binding Function
/// This is a comment for Function 
fn function() {}
//- FunctionComment documents Function
//- FunctionComment.node/kind doc
//- FunctionComment.text "This is a comment for Function"

//- @+2"add" defines/binding Macro
/// This is a comment for Macro
macro_rules! add {
    ($a:expr,$b:expr) => {{ $a + $b }};
}
//- MacroComment documents Macro
//- MacroComment.node/kind doc
//- MacroComment.text "This is a comment for Macro"

//- @+2"TWO" defines/binding Static
/// This is a comment for Static
static TWO: i32 = 2;
//- StaticComment documents Static
//- StaticComment.node/kind doc
//- StaticComment.text "This is a comment for Static"

//- @+2"Trait" defines/binding Trait
/// This is a comment for Trait
trait Trait {}
//- TraitComment documents Trait
//- TraitComment.node/kind doc
//- TraitComment.text "This is a comment for Trait"

//- @+2"Alias" defines/binding Alias
/// This is a comment for Alias
type Alias = i32;
//- AliasComment documents Alias
//- AliasComment.node/kind doc
//- AliasComment.text "This is a comment for Alias"
