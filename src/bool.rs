use crate::*;
use crate::function_trait::Valued;

pub struct True();
assign_value!(True, bool, true);
pub struct False();
assign_value!(False, bool, false);
function_trait!(TypeNot {True => False, False => True});
function_trait!(TypeAnd {{True, True} => True, {True, False} => False, {False, True} => False, {False, False} => False});
function_trait!(TypeOr {{True, True} => True, {True, False} => True, {False, True} => True, {False, False} => False});
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{evaluate, apply_function_trait};
    #[test]
    fn apply() {
        assert!(evaluate!(apply_function_trait!(TypeNot, False)));
        assert!(!evaluate!(apply_function_trait!(TypeNot, True)));
        assert!(!evaluate!(apply_function_trait!(TypeAnd, {True, False})));
        assert!(evaluate!(apply_function_trait!(TypeOr, {False, True})));
    }
}