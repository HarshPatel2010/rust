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
    println!("success!!");
}

pub struct Array<T,const N:usize>{
    data:[T;N]
}