#[allow(non_snake_case)]
use std::error::Error;
use std::sync::{ Arc, Mutex };

pub type ResultOf< T > = Result< T, Box::<dyn Error> >;

pub type ArcMut< T > = Arc<Mutex< T >>;

pub fn new_arcmut<T>( t: T ) -> ArcMut< T > {
    Arc::new(
            Mutex::new(t)
        )
}

