use std::marker::PhantomData;
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

trait And {
    type BoolType: TypeBool;
}

impl<T: TypeBool> And for TypePair<True, T> {
    type BoolType = T;
}

impl<T: TypeBool> And for TypePair<False, T> {
    type BoolType = False;
}

trait Or {
    type BoolType: TypeBool;
}

impl<T> Or for TypePair<True, T> {
    type BoolType = True;
}

impl<T: TypeBool> Or for TypePair<False, T> {
    type BoolType = T;
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
}

trait ComparisonType{}
struct Less();
impl ComparisonType for Less{}
struct Equal();
impl ComparisonType for Equal{}
struct More();
impl ComparisonType for More{}

trait Compared{
    type Comparison: ComparisonType;
}

impl<T> Compared for TypePair<Succ<T>, Zero>{
    type Comparison = More;
}
impl<T, S> Compared for TypePair<Succ<T>, Negative<S>>{
    type Comparison = More;
}

impl<T, S> Compared for TypePair<Succ<T>, Succ<S>>
where TypePair<T,S>: Compared{
    type Comparison = <TypePair<T,S> as Compared>::Comparison;
}

impl<T> Compared for TypePair<Zero, Succ<T>>{
    type Comparison = Less;
}

impl Compared for TypePair<Zero, Zero>{
    type Comparison = Equal;
}

impl<T> Compared for TypePair<Zero, Negative<T>>{
    type Comparison = More;
}

impl<T, S> Compared for TypePair<Negative<T>, Succ<S>>
{
    type Comparison = Less;
}

impl<T> Compared for TypePair<Negative<T>, Zero>
{
    type Comparison = Less;
}

impl<T, S> Compared for TypePair<Negative<T>, Negative<S>>
where  TypePair<S,T>: Compared{
    type Comparison = <TypePair<S,T> as Compared>::Comparison;
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

pub const fn count_different(first: &[u8], second: &[u8]) -> usize {
    let mut first_index = 0;
    let mut second_index = 0;
    let mut count = 0;
    while first_index < first.len() || second_index < second.len() {
        if first_index == first.len() {
            second_index += 1;
        } else if second_index == second.len() {
            first_index += 1;
        } else if first[first_index] < second[second_index] {
            first_index += 1;
        } else if first[first_index] > second[second_index] {
            second_index += 1;
        } else {
            first_index += 1;
            second_index += 1;
        }

        count += 1;
    }
    count
}
#[cfg(test)]
mod count_different_tests {
    use super::*;
    #[test]
    fn runtime() {
        let zero_to_two = [0, 1, 2];
        let one_to_five = [1, 2, 3, 4, 5];
        assert_eq!(count_different(&zero_to_two, &one_to_five), 6);
    }

    #[test]
    fn comp_time() {
        const SIX: usize = count_different(&[0, 1, 2], &[1, 2, 3, 4, 5]);
        assert_eq!(SIX, 6);
    }
}

const fn union<const N: usize>(first: &[u32], second: &[u32]) -> [u32; N] {
    let mut out = [0; N];
    let mut first_index = 0;
    let mut second_index = 0;
    let mut out_index = 0;
    while out_index < N {
        if first_index == first.len() {
            out[out_index] = second[second_index];
            second_index += 1;
        } else if second_index == second.len() {
            out[out_index] = first[first_index];
            first_index += 1;
        } else if first[first_index] < second[second_index] {
            out[out_index] = first[first_index];
            first_index += 1;
        } else if first[first_index] > second[second_index] {
            out[out_index] = second[second_index];
            second_index += 1;
        } else {
            out[out_index] = first[first_index];
            first_index += 1;
            second_index += 1;
        }
        out_index += 1;
    }
    out
}

#[cfg(test)]
mod union_tests {
    use super::*;
    #[test]
    fn comp_time() {
        const NUMBERS: [u32; count_different(&[0, 1, 2], &[1, 2, 3])] =
            union(&[0, 1, 2], &[1, 2, 3]);
        assert_eq!(NUMBERS, [0, 1, 2, 3]);
    }
}

struct NumberAndTail<const N: u32, T>(PhantomData<T>);
trait UnionCombine {
    type UnionType;
}

impl<T> UnionCombine for TypePair<(), T> {
    type UnionType = T;
}

impl<const N: u32, T> UnionCombine for TypePair<NumberAndTail<N, T>, ()> {
    type UnionType = NumberAndTail<N, T>;
}

struct NumberAndTailCombine<
    const N1: u32,
    Tail1,
    const N2: u32,
    Tail2,
    const EQ: bool,
    const LESS: bool,
>(PhantomData<Tail1>, PhantomData<Tail2>);

impl<const N1: u32, Tail1, const N2: u32, Tail2, const LESS: bool> UnionCombine
    for NumberAndTailCombine<N1, Tail1, N2, Tail2, true, LESS>
where
    TypePair<Tail1, Tail2>: UnionCombine,
{
    type UnionType = NumberAndTail<N1, <TypePair<Tail1, Tail2> as UnionCombine>::UnionType>;
}

impl<const N1: u32, Tail1, const N2: u32, Tail2> UnionCombine
    for NumberAndTailCombine<N1, Tail1, N2, Tail2, false, true>
where
    TypePair<Tail1, NumberAndTail<N2, Tail2>>: UnionCombine,
{
    type UnionType =
        NumberAndTail<N1, <TypePair<Tail1, NumberAndTail<N2, Tail2>> as UnionCombine>::UnionType>;
}

impl<const N1: u32, Tail1, const N2: u32, Tail2> UnionCombine
    for NumberAndTailCombine<N1, Tail1, N2, Tail2, false, false>
where
    TypePair<NumberAndTail<N2, Tail2>, Tail1>: UnionCombine,
{
    type UnionType =
        NumberAndTail<N2, <TypePair<NumberAndTail<N2, Tail2>, Tail1> as UnionCombine>::UnionType>;
}

// impl<const N1: u32, T1, const N2: u32, T2> UnionCombine
//     for TypePair<NumberAndTail<N1, T1>, NumberAndTail<N2, T2>>
// {
//     type UnionType = <NumberAndTailCombine<N1, T1, N2, T2, {N1 == N2}, {N1 < N2} > as UnionCombine>::UnionType;
// }
