//- TestModAnchor.node/kind anchor
//- TestModAnchor.loc/start 0
//- TestModAnchor.loc/end 0
//- TestModAnchor defines/implicit TestMod
//- TestMod.node/kind record
//- TestMod.subkind module
//- TestMod.complete definition
//- TestMod childof ImplicitMod

//- @crate ref ImplicitMod
//- @explicit_module ref ExplicitMod
use crate::explicit_module;