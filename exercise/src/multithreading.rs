use std::{sync::mpsc, thread};


pub fn calculate_huge_sum(){
    let (tx,rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut res: u128 = 0;
            for j in i*10000000..(i+1)*10000000 {
                res += j;
            };
            let _ = producer.send(res);
        });
    }

    drop(tx);
    let mut ans: u128 = 0;

    for val in rx {
        ans +=  val;
    }

    println!("final res {}", ans);
}