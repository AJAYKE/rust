
pub fn getting_srarted_with_iterators() {
    let nums = vec![1,2,3,4,4,5,5,6,6];

    let iter_2 = nums.iter()
    .filter(|x| *x%2==1).map(|x| x*2);

    let vectoru: Vec<i32> = iter_2.collect();

    for val in vectoru.iter() {
        println!("{}",val);
}

    println!("{:?}", vectoru);
}