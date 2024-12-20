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
    }
    main()
}

// Implement `fn sum` with trait bound in two ways.
pub fn sum<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
    x + y // x.add(y)
}