//- ImplicitModAnchor.node/kind anchor
//- ImplicitModAnchor.loc/start 0
//- ImplicitModAnchor.loc/end 0
//- ImplicitModAnchor defines/implicit ImplicitMod
//- ImplicitMod.node/kind record
//- ImplicitMod.subkind module
//- ImplicitMod.complete definition

//- @test ref TestMod
mod test;

//- @explicit_module defines/binding ExplicitMod
//- ExplicitMod.node/kind record
//- ExplicitMod.subkind module
//- ExplicitMod.complete definition
//- ExplicitMod childof ImplicitMod
mod explicit_module {
    //- @super ref ImplicitMod
    //- @test ref TestMod
    use super::test;
}
