
pub fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v:Vec<u8> = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    // vec!(..) and vec![..] are same macros, so
    let v = vec!(1, 2, 3);
    is_vec(&v);

    // In code below, v is Vec<[u8; 3]> , not Vec<u8>
    // USE Vec::new and `for` to rewrite the below code
    let mut v1= Vec::new();
    is_vec(&v1);
    for i in &v{
        v1.push(*i);
    }

    assert_eq!(v, v1);

    println!("Success!2");

    // FILL in the blank
    fn main() {
        let mut v1 :Vec<i32>= Vec::from([1, 2, 4]);
        v1.pop();//[1,2]
        v1.push(3);//[1,2,3]

        let mut v2 = Vec::new();
        v2.extend(&v1);

        assert_eq!(v1, v2);

        println!("Success!");

        // FILL in the blanks
        fn main() {
            // Array -> Vec
            // impl From<[T; N]> for Vec
            let arr: [i32;3]= [1, 2, 3];
            let v1:Vec<i32> = Vec::from(arr);
            let v2: Vec<i32> = arr.into();

            assert_eq!(v1, v2);


            // String -> Vec
            // impl From<String> for Vec
            let s:String = "hello".to_string();
            let v1: Vec<u8> = s.into();

            let s = "hello".to_string();
            let v2 = s.into_bytes();
            assert_eq!(v1, v2);

            // impl<'_> From<&'_ str> for Vec
            let s:&str = "hello";
            let v3 = Vec::from(s);
            assert_eq!(v2, v3);

            // Iterators can be collected into vectors
            let v4: Vec<i32> = [0; 10].into_iter().collect();
            assert_eq!(v4, vec![0; 10]);

            println!("Success!");

            // FIX the error and IMPLEMENT the code
            fn main() {
                let mut v = Vec::from([1, 2, 3]);
                for i in 0..5 {
                    println!("{:?}", v.get(i));
                }

                for i in 0..5 {
                    // IMPLEMENT the code here...
                    match v.get(i){
                        Some(e)=>v[i]=e+1,
                        None => v.push(i+2)

                    }
                }

                assert_eq!(v, vec![2, 3, 4, 5, 6]);

                println!("Success!");
                // FIX the errors
                fn main() {
                    let mut vec :Vec<i32>= Vec::with_capacity(10);

                    // The vector contains no items, even though it has capacity for more
                    assert_eq!(vec.len(), 0);
                    assert_eq!(vec.capacity(), 10);

                    // These are all done without reallocating...
                    for i in 0..10 {
                        vec.push(i);
                    }
                    assert_eq!(vec.len(), 10);
                    assert_eq!(vec.capacity(), 10);

                    // ...but this may make the vector reallocate
                    vec.push(11);
                    assert_eq!(vec.len(), 11);
                    assert!(vec.capacity() >= 11);


                    // Fill in an appropriate value to make the `for` done without reallocating
                    let mut vec:Vec<i32> = Vec::with_capacity(100);
                    for i in 0..100 {
                        vec.push(i);
                    }

                    assert_eq!(vec.len(), 100);
                    assert_eq!(vec.capacity(), 100);

                    println!("Success!45");
                }
                main()
            }
            main()
        }
        main()
    }
    main()
}

fn is_vec(v: &Vec<u8>) {}