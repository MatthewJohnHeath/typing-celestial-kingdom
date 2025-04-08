use crate::*;
use crate::function_trait::Valued;

pub struct True();
assign_value!(True, bool, true);
pub struct False();
assign_value!(False, bool, false);
function_trait!(TypeNot {True => False, False => True});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{evaluate, apply_function_trait};
    #[test]
    fn apply() {
        assert!(evaluate!(apply_function_trait!(TypeNot, False)));
        assert!(!evaluate!(apply_function_trait!(TypeNot, True)));
    }
}