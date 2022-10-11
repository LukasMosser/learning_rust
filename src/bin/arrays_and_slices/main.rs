use std::mem;

fn analyze_slice(slice: &[i32]){
    println!("first element of the slice: {}", slice[0]);
    println!("Length of the slice: {}", slice.len());
}

fn main(){
    let xs: [i32; 5] = [1, 2, 3,4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("Length of the array: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section as a slice");
    analyze_slice(&ys[1 .. 4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 2 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    //println!("{}", xs[5]);
}