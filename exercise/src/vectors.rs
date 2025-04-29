
pub fn getting_started_with_vectors() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);

}

pub fn filter_evens(nums: &mut Vec<i32>) {
    let mut i = 0;
    while i < nums.len() {
        if nums[i]%2 ==1 {
            nums.remove(i);
        }
        i += 1;
    }
}