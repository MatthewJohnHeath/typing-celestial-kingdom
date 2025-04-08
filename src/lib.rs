pub mod function_trait;

use std::marker::PhantomData;

//use proc_macro2::TokenStream;
//use quote::quote;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct False();

#[derive(Debug, PartialEq, Eq, Default)]
pub struct True();

pub trait TypeBool {
    type Not: TypeBool;
    const VALUE: bool;
}

impl TypeBool for False {
    type Not = True;
    const VALUE: bool = false;
}

impl TypeBool for True {
    type Not = False;
    const VALUE: bool = true;
}

pub struct TypePair<T, S>(PhantomData<T>, PhantomData<S>);

pub trait And {
    type BoolType: TypeBool;
}

impl<T: TypeBool> And for TypePair<True, T> {
    type BoolType = T;
}

impl<T: TypeBool> And for TypePair<False, T> {
    type BoolType = False;
}

pub trait Or {
    type BoolType: TypeBool;
}

impl<T> Or for TypePair<True, T> {
    type BoolType = True;
}

impl<T: TypeBool> Or for TypePair<False, T> {
    type BoolType = T;
}

pub trait Same {
    type BoolType: TypeBool;
}

impl Same for TypePair<False, False> {
    type BoolType = False;
}

impl Same for TypePair<True, True> {
    type BoolType = True;
}

impl Same for TypePair<True, False> {
    type BoolType = False;
}

impl Same for TypePair<False, True> {
    type BoolType = False;
}

#[macro_export]
macro_rules! type_not {
    ($bool_t:ty) => {
        <$bool_t as TypeBool>::Not
    };
}

#[macro_export]
macro_rules! type_and {
    ($first:ty, $second:ty) => {
        <TypePair<$first, $second> as And>::BoolType
    };
}

#[macro_export]
macro_rules! type_or {
    ($first:ty, $second:ty) => {
        <TypePair<$first, $second> as Or>::BoolType
    };
}

#[macro_export]
macro_rules! type_bool_eq {
    ($first:ty, $second:ty) => {
        <TypePair<$first, $second> as Equal>::BoolType
    };
}

#[cfg(test)]
mod logic_tests {
    use super::*;
    #[test]
    fn not() {
        assert!(<False as TypeBool>::Not::VALUE);
        assert!(!<True as TypeBool>::Not::VALUE);
    }
    #[test]
    fn and() {
        assert!(<TypePair<True, True> as And>::BoolType::VALUE);
        assert!(!<TypePair<False, True> as And>::BoolType::VALUE);
        assert!(!<TypePair<True, False> as And>::BoolType::VALUE);
        assert!(!<TypePair<False, False> as And>::BoolType::VALUE);
    }
    #[test]
    fn or() {
        assert!(<TypePair<True, True> as Or>::BoolType::VALUE);
        assert!(<TypePair<False, True> as Or>::BoolType::VALUE);
        assert!(<TypePair<True, False> as Or>::BoolType::VALUE);
        assert!(!<TypePair<False, False> as And>::BoolType::VALUE);
    }
}

pub struct Zero();
pub struct Succ<T>(PhantomData<T>);
pub struct Negative<T>(PhantomData<T>);

pub trait TypeInt {
    type Previous: TypeInt;
    type Next: TypeInt;
    type Negation: TypeInt;
    const VALUE: i64;
}

impl TypeInt for Zero {
    type Previous = Negative<Succ<Self>>;
    type Next = Succ<Self>;
    type Negation = Self;
    const VALUE: i64 = 0;
}

impl<T: TypeInt> TypeInt for Succ<T> {
    type Previous = T;
    type Next = Succ<Self>;
    type Negation = Negative<Self>;
    const VALUE: i64 = T::VALUE + 1;
}

impl<T: TypeInt> TypeInt for Negative<T> {
    type Previous = Negative<T::Next>;
    type Next = Negative<T::Previous>;
    type Negation = T;
    const VALUE: i64 = -T::VALUE;
}

#[cfg(test)]
mod type_int_tests {
    use super::*;
    type One = Succ<Zero>;
    type Two = Succ<One>;
    type MinusOne = Negative<One>;
    #[test]
    fn values() {
        assert_eq!(Zero::VALUE, 0);
        assert_eq!(One::VALUE, 1);
        assert_eq!(Two::VALUE, 2);
        assert_eq!(MinusOne::VALUE, -1);
    }

    type MinusTwo = Negative<Two>;
    #[test]
    fn next() {
        assert_eq!(<Zero as TypeInt>::Next::VALUE, 1);
        assert_eq!(<One as TypeInt>::Next::VALUE, 2);
        assert_eq!(<MinusOne as TypeInt>::Next::VALUE, 0);
        assert_eq!(<MinusTwo as TypeInt>::Next::VALUE, -1);
    }

    #[test]
    fn previous() {
        assert_eq!(<Zero as TypeInt>::Previous::VALUE, -1);
        assert_eq!(<One as TypeInt>::Previous::VALUE, 0);
        assert_eq!(<Two as TypeInt>::Previous::VALUE, 1);
        assert_eq!(<MinusOne as TypeInt>::Previous::VALUE, -2);
    }

    #[test]
    fn negation() {
        assert_eq!(<Zero as TypeInt>::Negation::VALUE, 0);
        assert_eq!(<One as TypeInt>::Negation::VALUE, -1);
        assert_eq!(<MinusOne as TypeInt>::Negation::VALUE, 1);
    }
}

pub trait Add {
    type Sum: TypeInt;
}

impl<T: TypeInt> Add for TypePair<Zero, T> {
    type Sum = T;
}
impl<T: TypeInt, S: TypeInt> Add for TypePair<Succ<T>, S>
where
    TypePair<T, S>: Add,
    <TypePair<T, S> as Add>::Sum: TypeInt,
{
    type Sum = <<TypePair<T, S> as Add>::Sum as TypeInt>::Next;
}
impl<T: TypeInt, S: TypeInt> Add for TypePair<Negative<T>, S>
where
    <S as TypeInt>::Negation: TypeInt,
    TypePair<T, <S as TypeInt>::Negation>: Add,
    <TypePair<T, <S as TypeInt>::Negation> as Add>::Sum: TypeInt,
    <<TypePair<T, <S as TypeInt>::Negation> as Add>::Sum as TypeInt>::Negation: TypeInt,
{
    type Sum = <<TypePair<T, <S as TypeInt>::Negation> as Add>::Sum as TypeInt>::Negation;
}

#[macro_export]
macro_rules! type_neg {
    ($num:ty) => {
        <$num as TypeInt>::Negation
    };
}

#[macro_export]
macro_rules! type_add {
    ($first:ty, $second:ty) => {
        <TypePair<$first, $second> as Add>::Sum
    };
}

#[macro_export]
macro_rules! type_sub {
    ($first:ty, $second:ty) => {
        type_add!($first, type_neg!($second))
    };
}

#[cfg(test)]
mod arithmetic_tests {
    use super::*;
    type One = Succ<Zero>;
    type Two = Succ<One>;
    type MinusOne = Negative<One>;

    #[test]
    fn add() {
        assert_eq!(<TypePair<Zero, Zero> as Add>::Sum::VALUE, 0);
        assert_eq!(<TypePair<Zero, One> as Add>::Sum::VALUE, 1);
        assert_eq!(<TypePair<One, Zero> as Add>::Sum::VALUE, 1);
        assert_eq!(<TypePair<One, Two> as Add>::Sum::VALUE, 3);
        assert_eq!(<TypePair<Zero, MinusOne> as Add>::Sum::VALUE, -1);
        assert_eq!(<TypePair<One, MinusOne> as Add>::Sum::VALUE, 0);
        assert_eq!(<TypePair<MinusOne, Zero> as Add>::Sum::VALUE, -1);
        assert_eq!(<TypePair<MinusOne, One> as Add>::Sum::VALUE, 0);
        assert_eq!(<TypePair<MinusOne, MinusOne> as Add>::Sum::VALUE, -2);
    }

    #[test]
    fn neg_macro() {
        assert_eq!(<type_neg!(Two)>::VALUE, -2);
        assert_eq!(<type_neg!(Zero)>::VALUE, 0);
        assert_eq!(<type_neg!(MinusOne)>::VALUE, 1);
    }

    #[test]
    fn add_macro() {
        assert_eq!(<type_add!(One, One)>::VALUE, 2);
        assert_eq!(<type_add!(One, MinusOne)>::VALUE, 0);
        assert_eq!(<type_add!(type_add!(One, One), One)>::VALUE, 3);
    }

    #[test]
    fn sub_macro() {
        assert_eq!(<type_sub!(One, One)>::VALUE, 0);
        assert_eq!(<type_sub!(One, MinusOne)>::VALUE, 2);
        assert_eq!(<type_sub!(One, type_sub!(One, One))>::VALUE, 1);
    }
}

trait ComparisonType {
    type IsLess;
    type IsEqual;
    type IsMore;
}
struct Less();
impl ComparisonType for Less {
    type IsLess = True;
    type IsEqual = False;
    type IsMore = False;
}
struct Equal();
impl ComparisonType for Equal {
    type IsLess = False;
    type IsEqual = True;
    type IsMore = False;
}
struct More();
impl ComparisonType for More {
    type IsLess = False;
    type IsEqual = False;
    type IsMore = True;
}

trait Compared {
    type Comparison: ComparisonType;
}

impl<T> Compared for TypePair<Succ<T>, Zero> {
    type Comparison = More;
}
impl<T, S> Compared for TypePair<Succ<T>, Negative<S>> {
    type Comparison = More;
}

impl<T, S> Compared for TypePair<Succ<T>, Succ<S>>
where
    TypePair<T, S>: Compared,
{
    type Comparison = <TypePair<T, S> as Compared>::Comparison;
}

impl<T> Compared for TypePair<Zero, Succ<T>> {
    type Comparison = Less;
}

impl Compared for TypePair<Zero, Zero> {
    type Comparison = Equal;
}

impl<T> Compared for TypePair<Zero, Negative<T>> {
    type Comparison = More;
}

impl<T, S> Compared for TypePair<Negative<T>, Succ<S>> {
    type Comparison = Less;
}

impl<T> Compared for TypePair<Negative<T>, Zero> {
    type Comparison = Less;
}

impl<T, S> Compared for TypePair<Negative<T>, Negative<S>>
where
    TypePair<S, T>: Compared,
{
    type Comparison = <TypePair<S, T> as Compared>::Comparison;
}

#[macro_export]
macro_rules! type_compare {
    ($first:ty, $second:ty) => {
        <TypePair<$first, $second> as Compared>::Comparison
    };
}

#[macro_export]
macro_rules! type_lt {
    ($first:ty, $second:ty) => {
        <type_compare!($first, $second) as ComparisonType>::IsLess
    };
}

#[macro_export]
macro_rules! type_gt {
    ($first:ty, $second:ty) => {
        type_lt!($second, $first)
    };
}

#[macro_export]
macro_rules! type_le {
    ($first:ty, $second:ty) => {
        type_not!(type_gt!($first, $second))
    };
}

#[macro_export]
macro_rules! type_ge {
    ($first:ty, $second:ty) => {
        type_not!(type_lt!($first, $second))
    };
}

#[macro_export]
macro_rules! type_eq {
    ($first:ty, $second:ty) => {
        type_and!(type_le!($first, $second), type_ge!($first, $second))
    };
}

#[cfg(test)]
mod comparison_tests {
    use super::*;
    type One = Succ<Zero>;
    type MinusOne = Negative<One>;

    #[test]
    fn lt_macro() {
        assert!(<type_lt!(Zero, One)>::VALUE);
        assert!(!<type_lt!(Zero, Zero)>::VALUE);
        assert!(!<type_lt!(One, MinusOne)>::VALUE);
    }

    #[test]
    fn gt_macro() {
        assert!(!<type_gt!(Zero, One)>::VALUE);
        assert!(!<type_gt!(Zero, Zero)>::VALUE);
        assert!(<type_gt!(One, MinusOne)>::VALUE);
    }

    #[test]
    fn le_macro() {
        assert!(<type_le!(Zero, One)>::VALUE);
        assert!(<type_le!(Zero, Zero)>::VALUE);
        assert!(!<type_le!(One, MinusOne)>::VALUE);
    }

    #[test]
    fn ge_macro() {
        assert!(!<type_ge!(Zero, One)>::VALUE);
        assert!(<type_ge!(Zero, Zero)>::VALUE);
        assert!(<type_ge!(One, MinusOne)>::VALUE);
    }

    #[test]
    fn eq_macro() {
        assert!(!<type_eq!(Zero, One)>::VALUE);
        assert!(<type_eq!(Zero, Zero)>::VALUE);
        assert!(!<type_eq!(One, MinusOne)>::VALUE);
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ChoiceType<const BOOL: bool, F, S>(PhantomData<F>, PhantomData<S>);

pub trait Associated {
    type AssociatedType;
}

impl<F, S> Associated for ChoiceType<true, F, S> {
    type AssociatedType = F;
}

impl<F, S> Associated for ChoiceType<false, F, S> {
    type AssociatedType = S;
}

pub const fn before(first: &str, second: &str) -> bool {
    let mut i = 0;
    let n = if first.len() < second.len() {
        first.len()
    } else {
        second.len()
    };
    while i < n {
        if first.as_bytes()[i] < second.as_bytes()[i] {
            return true;
        }
        i = i + 1;
    }
    return first.len() < second.len();
}

#[cfg(test)]
mod choice_type_tests {
    use super::*;
    #[test]
    fn with_bool_literal() {
        assert!(<ChoiceType<true, True, False> as Associated>::AssociatedType::VALUE);
        assert!(!<ChoiceType<false, True, False> as Associated>::AssociatedType::VALUE);
    }

    #[test]
    fn with_expression() {
        assert!(<ChoiceType<{ 1 < 2 }, True, False> as Associated>::AssociatedType::VALUE);
        assert!(!<ChoiceType<{ 1 > 2 }, True, False> as Associated>::AssociatedType::VALUE);
    }

    #[test]
    fn with_before() {
        assert!(<ChoiceType< {before("Hello", "World!")}, True, False> as Associated>::AssociatedType::VALUE);
        assert!(!<ChoiceType< {before("Bye", "Bye")}, True, False> as Associated>::AssociatedType::VALUE);
    }
}
