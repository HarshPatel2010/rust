pub fn main() {
    assert_eq!(sum(1, 2), 3);
    println!("{}", sum(5, 5));


    // FIX the errors.
    #[derive(Debug)]
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {:?}", self.x);
            } else {
                println!("The largest member is y = {:?}", self.y);
            }
        }
    }
#[derive(Debug,PartialEq,PartialOrd)]
    struct Unit(i32);

    fn main() {
        let pair:Pair<Unit> = Pair::new(Unit(11),Unit(2));


        pair.cmp_display();

        struct Container(i32, i32);

        // USING associated types to re-implement trait Contains.
        // trait Contains {
        //    type A;
        //    type B;

        trait Contains<A, B> {
            fn contains(&self, _: &A, _: &B) -> bool;
            fn first(&self) -> i32;
            fn last(&self) -> i32;
        }

        impl Contains<i32, i32> for Container {
            fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
                (&self.0 == number_1) && (&self.1 == number_2)
            }
            // Grab the first number.
            fn first(&self) -> i32 { self.0 }

            // Grab the last number.
            fn last(&self) -> i32 { self.1 }
        }

        fn difference<A, B, C: Contains<A, B>>(container: &C) -> i32 {
            container.last() - container.first()
        }

        fn main() {
            let number_1 = 3;
            let number_2 = 10;

            let container = Container(number_1, number_2);

            println!("Does container contain {} and {}: {}",
                     &number_1, &number_2,
                     container.contains(&number_1, &number_2));
            println!("First number: {}", container.first());
            println!("Last number: {}", container.last());

            println!("The difference is: {}", difference(&container));
        }
        main()
    }
    main()

}

// Implement `fn sum` with trait bound in two ways.
pub fn sum<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
    x + y // x.add(y)
}