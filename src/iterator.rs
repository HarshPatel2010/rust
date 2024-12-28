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
        }
        main()
    }
    main()
}