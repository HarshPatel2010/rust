pub fn main() {
    let v = vec![1, 2, 3];
    for x in v {
        println!("{}",x)

    }
    /* Fill the blanks and fix the errors.
Using two ways if possible */
    fn main() {
        let mut v1 = vec![1, 2].into_iter();

        assert_eq!(v1.next(),Some(1));
        assert_eq!(v1.next(), Some(2));
        assert_eq!(v1.next(), None);
        /* Make it work */
        fn main() {
            let arr:Vec<i32> = vec![0; 10];
            for i in arr.iter() {
                println!("{}", i);
            }

            println!("{:?}",arr);
            /* Fill in the blank */
            fn main() {
                let mut names :Vec<&str>= vec!["Bob", "Frank", "Ferris"];

                for name in names.iter_mut() {
                    *name = match name {
                        &mut "Ferris" => "There is a rustacean among us!",
                        _ => "Hello",
                    }
                }

                println!("names: {:?}", names);
                /* Fill in the blank */
                fn main() {
                    let mut values:Vec<i32> = vec![1, 2, 3];
                    let mut values_iter = values.iter_mut();

                    if let Some(v) = values_iter.next(){
                        *v=0
                    }

                    assert_eq!(values, vec![0, 2, 3]);
                    for e in values  {
                        println!("{}",e);
                    }
                    /* Fill in the blanks */
                    fn main() {
                        let v1: Vec<i32> = vec![1, 2, 3];

                        let v2: Vec<i32> = v1.iter().map(|n| n+1).collect();

                        assert_eq!(v2, vec![2, 3, 4]);
                        /* Make it work */
                        use std::collections::HashMap;
                        fn main() {
                            let names = [("sunface",18), ("sunfei",18)];
                            let folks: HashMap<&str,i32> = names.into_iter().collect();

                            println!("{:?}",folks);

                            let v1: Vec<i32> = vec![1, 2, 3];

                            let v2:Vec<i32> = v1.into_iter().collect();

                            assert_eq!(v2, vec![1, 2, 3]);
                        }
                        main()
                    }
                    main()
                }
                main()
            }
            main()
        }
        main()
    }
    main()
}