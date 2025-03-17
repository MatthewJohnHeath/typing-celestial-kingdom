use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Default)]
pub struct False();

#[derive(Debug, PartialEq, Eq, Default)]
pub struct True();


pub trait TypeBool{
    type Not;
    const VALUE: bool;
}

impl TypeBool for False {
    type Not = True;
    const VALUE: bool = false;
}

impl TypeBool for  True {
    type Not = False;
    const VALUE: bool = true;
}

pub struct TypePair<T,S>(PhantomData<T>, PhantomData<S>);

trait And{
    type BoolType;
}

impl<T> And for TypePair<True, T>{
    type BoolType = T;
}

impl<T> And for TypePair<False, T>{
    type BoolType = False;
}

trait Or{
    type BoolType;
}

impl<T> Or for TypePair<True, T>{
    type BoolType = True;
}

impl<T> Or for TypePair<False, T>{
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
    fn and(){
        assert!(<TypePair<True, True> as And>::BoolType::VALUE);
        assert!(!<TypePair<False, True> as And>::BoolType::VALUE);
        assert!(!<TypePair<True, False> as And>::BoolType::VALUE);
        assert!(!<TypePair<False, False> as And>::BoolType::VALUE);
    }
    #[test]
    fn or(){
        assert!(<TypePair<True, True> as Or>::BoolType::VALUE);
        assert!(<TypePair<False, True> as Or>::BoolType::VALUE);
        assert!(<TypePair<True, False> as Or>::BoolType::VALUE);
        assert!(!<TypePair<False, False> as And>::BoolType::VALUE);
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
