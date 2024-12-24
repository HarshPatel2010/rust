
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
    }
    main()
}

fn is_vec(v: &Vec<u8>) {}