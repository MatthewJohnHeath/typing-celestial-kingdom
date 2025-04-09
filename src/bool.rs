use crate::*;
use crate::function_trait::{Cons, Valued};

pub struct True();
assign_value!(True, bool, true);
pub struct False();
assign_value!(False, bool, false);
function_trait!(TypeNot {True => False, False => True});
function_trait!(TypeAnd {{True True} => True, {True False} => False, {False True} => False, {False False} => False});
function_trait!(TypeOr {{True True} => True, {True False} => True, {False True} => True, {False False} => False});

declare_function_trait!(Ternary);
impl<T,S> Ternary for Cons<True, Cons<T,S>>{
    type Type = T;
    
}

impl<T,S> Ternary for Cons<False, Cons<T,S>>{
    type Type = S;
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{evaluate, call_trait};
    #[test]
    fn ops() {
        assert!(evaluate!(call_trait!(TypeNot, False)));
        assert!(!evaluate!(call_trait!(TypeNot, True)));
        assert!(!evaluate!(call_trait!(TypeAnd, {True False})));
        assert!(evaluate!(call_trait!(TypeOr, {False True})));
    }

    // #[test]
    // fn ternary(){
    //     assert!(evaluate!(call_trait!(Ternary, {True, True, i64})));
    //     assert!(evaluate!(call_trait!(Ternary, {False, i64, True})));
    // }
}