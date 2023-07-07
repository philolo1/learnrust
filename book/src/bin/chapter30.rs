use std::sync::{Arc, Mutex};
use std::thread;

// part 1
// fn main() {
//     let m = Mutex::new(5);
//
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//
//     println!("m = {:?}", m);
// }
//
// part 2

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}
