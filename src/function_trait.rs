
pub mod function_traits{
    use std::marker::PhantomData;
   
    pub struct Cons<T, S>(PhantomData<T>, PhantomData<S>);

    #[macro_export]
    macro_rules! type_list{
        () => {()};
        ($type:ty) => {$type};
        ($head:ty, $($tail:tt)+) => {Cons<$head, type_list($($tail:tt)+)>};
    }
   
    #[macro_export]
    macro_rules! evaluate{
        ($type:ty)=>{<$type>::VALUE};
    }

    #[macro_export]
    macro_rules! apply_function_trait {
        ($function_trait:tt, $params:tt) => { <type_list!($params) as $function_trait>::Type};
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
            {$in:tt => $out:ty}
        } => {
                impl $trait_name for type_list!($in){
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

       impl_function_trait!(Barred {(Foo, Bar) => Bar});
       trait Truthy{
        const VALUE : bool;
       }

       impl Truthy for Bar {
           const VALUE : bool = true;
       }

       #[test]
       fn apply() {
           assert!(evaluate!(apply_function_trait!(Barred, Foo)));
           assert!(evaluate!(apply_function_trait!(Barred, (Foo, Bar))));
       }

    }
    
}