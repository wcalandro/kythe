//- @main defines/binding MainFn
fn main() {
    //- @"'outer" defines/binding OuterLabel
    //- OuterLabel childof MainFn
    //- OuterLabel.node/kind variable
    //- OuterLabel.complete definition
    //- OuterLabel.subkind label
    'outer: loop {
        //- @"'inner" defines/binding InnerLabel
        //- InnerLabel childof MainFn
        //- InnerLabel.node/kind variable
        //- InnerLabel.complete definition
        //- InnerLabel.subkind label
        'inner: loop {
            if 1 == 1 {
                //- @"'outer" ref OuterLabel
                break 'outer;
            } else {
                //- @"'inner" ref InnerLabel
                break 'inner;
            }
        }
    }
}
