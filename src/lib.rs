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
pub struct Succ<T>( PhantomData<T>);
pub struct Negative<T>(PhantomData<T>);


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
        type Previous = Negative<T::Next>;
        type Next = Negative<T::Previous>;
        const VALUE:i64 = -T::VALUE;
    }
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
    fn next(){
        assert_eq!(<Zero as TypeInt>::Next::VALUE, 1);
        assert_eq!(<One as TypeInt>::Next::VALUE, 2);
        assert_eq!(<MinusOne as TypeInt>::Next::VALUE, 0);
        assert_eq!(<MinusTwo as TypeInt>::Next::VALUE, -1);
    }

    #[test]
    fn previous(){
        assert_eq!(<Zero as TypeInt>::Previous::VALUE, -1);
        assert_eq!(<One as TypeInt>::Previous::VALUE, 0);
        assert_eq!(<Two as TypeInt>::Previous::VALUE, 1);
        assert_eq!(<MinusOne as TypeInt>::Previous::VALUE, -2);
    }
}