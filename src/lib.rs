use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Default)]
pub struct False();

#[derive(Debug, PartialEq, Eq, Default)]
pub struct True();


pub trait Deny{
    type Bang;
}

impl Deny for False {
    type Bang = True;
}

impl Deny for  True {
    type Bang = False;
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn not_false_is_true() {
        let factual = True::default();
        let not_false = <False as Deny>::Bang::default();
        assert_eq!(factual, not_false);
    }

    #[test]
    fn not_true_is_false() {
        let fake = False::default();
        let untrue = <True as Deny>::Bang::default();
        assert_eq!(fake, untrue);
    }
}


pub struct Zero();

pub struct Succ<T>{
    _marker: PhantomData<T>,
}

pub struct Negative<T>{
    _marker: PhantomData<T>,
}

pub trait TypeInt{
    type Previous;
    type Next;
    const VALUE: i64;
}

impl TypeInt for Zero{
    type Previous = Negative<Succ<Self>>;
    type Next = Succ<Self>;
    const VALUE:i64 = 0;
}

impl<T:TypeInt> TypeInt for Succ<T>
    {
        type Previous = T;
        type Next = Succ<Self>;
        const VALUE:i64 = T::VALUE + 1;
    }

impl<T:TypeInt> TypeInt for Negative<T> 
    {
        type Previous = T::Next;
        type Next = T::Previous;
        const VALUE:i64 = -T::VALUE;
    }
mod more_tests {
     use super::*;
    #[test]
    fn values() {
        assert_eq!(Zero::VALUE, 0);
        assert_eq!(Succ::<Zero>::VALUE, 1);
        assert_eq!(Succ::<Succ<Zero>>::VALUE, 2);
        assert_eq!(Negative::<Succ<Zero>>::VALUE, -1);
    }
}