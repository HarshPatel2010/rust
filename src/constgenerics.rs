pub fn main() {
    let arrays : [Array<i32,3>;3] = [
        Array{
            data:[1,2,3]
        },
        Array{
            data:[13,23,33]
        },
        Array{
            data:[1,2,11]
        }
    ];
 let floats : [Array<f64,2>;3]=[
     Array{data:[1.0,2.0]},
     Array{data:[1.0,2.0]},
     Array{data:[1.0,2.0]},

 ];
    println!("success!!");

    // Fill in the blanks to make it work.
    fn print_array<T :std::fmt::Debug,const N:usize>(arr:[T;N]){
        println!("{:?}", arr);
    }
    fn main() {
        let arr = [1, 2, 3];
        print_array(arr);

        let arr = ["hello", "world"];
        print_array(arr);
    }
    main()
}

pub struct Array<T,const N:usize>{
    data:[T;N]
}