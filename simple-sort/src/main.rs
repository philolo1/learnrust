use rand::Rng;

fn simple_sort(arr: &mut Vec<i32>) {
    for a in 0..arr.len() {
        for b in (a + 1)..arr.len() {
            if arr[b] < arr[a] {
                let res = arr[a];
                arr[a] = arr[b];
                arr[b] = res;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut arr : Vec<i32> = vec![];
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(0..=10);
        arr.push(number)
    }

    println!("vec: {:?}", arr);
    simple_sort(&mut arr);

    println!("vec sorted: {:?}", arr);
    println!("vec length: {}", arr.len());

    for a in 0..9 {
        assert!(arr[a] <= arr[a+1], "Index {} and {} are not the same", a, a+1)
    }



}
