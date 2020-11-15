fn main() {
    let a =vec! [1,2,100,4,4294967295];
    let sum_  = sum_of_vec(&a);
    println!("{:?}",sum_);
}

fn sum_of_vec(vec: &[u32]) -> Option<u32>{
    if vec.len()>0{
        let mut total: u32 = 0;
        for value in vec.iter(){
            total.checked_add(*value);
            total += value;
        }
        Some(total)
    }
    else {
        None
    }
}
