#[allow(unused_variables)]
fn turbofish() {
    
    #[derive(Debug)]
    struct Pet<T> {
        cats: T,
        dogs: T,
    }
    
    
    // impl Pet<i32> {
    //     // notice new() is a static method
    //     fn new() -> Self {
    //         Pet {
    //             cats: 5,
    //             dogs: 10,
    //         }
    //     }
    // }

    // so here a couple problems, by naive assumption
    // we could continue like, but it defeats the whole purpose of generics
    // what to do?

    // impl Pet<i64> {
    //     // notice new() is a static method
    //     fn new() -> Self {
    //         Pet {
    //             cats: 5,
    //             dogs: 10,
    //         }
    //     }
    // }

    // let pets = Pet::new();
    // println!("{:?}",pets);

    impl<T> Pet<T> {
        // notice new() is a static method
        fn new(a:T,b:T) -> Self {
            Pet {
                cats: a,
                dogs: b,
            }
        }
    }
    
    let pets = Pet::<i64>::new(5_i64,10_i64);
    println!("Cats: {}, Dogs: {}",pets.cats,pets.dogs);

    let pets = Pet::<String>::new("Million".into(),"Billion".into());
    println!("Cats: {}, Dogs: {}",pets.cats,pets.dogs);
    
}
fn main() {
    turbofish();
}