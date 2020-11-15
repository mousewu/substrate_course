fn main() {
    let a =vec! [];
    let sum_  = sum_of_vec(&a);
    println!("{:?}",sum_);
}

fn sum_of_vec(vec: &[u32]) -> Option<u32>{
    if vec.len()>0{
        let mut sum_: u32 = 0;
        for value in vec.iter() {
            match sum_.checked_add(* value){
                Some(_v) => sum_ = sum_ + value,
                None => return None
            }
    }
    Some(sum_)
    }
    else{
        None
    }
}
