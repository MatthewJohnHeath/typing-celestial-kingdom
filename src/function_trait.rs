pub mod function_traits{
    #[macro_export]
    macro_rules! evaluate{
        ($type:ty)=>{<$type>::VALUE};
    }

    #[macro_export]
    macro_rules! apply_function_trait {
        ($function_trait:tt, $param:ty) => { <$param as $function_trait>::Type};
    }

    #[macro_export]
    macro_rules! declare_function_trait {
        {$trait_name:tt} => {        
            trait $trait_name{
            type Type;
           }};
    }

     #[macro_export]
    macro_rules! impl_function_trait {
        {
            $trait_name:tt
            {$in:ty => $out:ty}
        } => {
                impl $trait_name for $in{
                type Type = $out;
                }
            };
    }  


    #[cfg(test)]
    mod tests {
       struct Foo();
       struct Bar();
       
       declare_function_trait!{Barred}
       impl_function_trait!(Barred {Foo => Bar});
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