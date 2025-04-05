pub mod function_traits{
    #[macro_export]
    macro_rules! evaluate{
        ($type:ty)=>{<$type>::VALUE};
    }

    #[macro_export]
    macro_rules! apply_function_trait {
        ($function_trait:tt, $param:ty) => { <$param as $function_trait>::Type};
    }


    #[cfg(test)]
    mod tests {
       struct Foo();
       struct Bar();
       
       trait Barred{
        type Type;
       }
       
       impl Barred for Foo{
        type Type = Bar;
       }
       trait Truthy{
        const VALUE : bool;
       }

       impl Truthy for Bar {
           const VALUE : bool = true;
       }

       #[test]
       fn apply() {
           assert!(evaluate!(apply_function_trait!(Barred, Foo)));
       }

    }
    
}