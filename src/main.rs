use rand::Rng;
mod heap_sort;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<i32> = (0..10).map(|x| rng.gen_range(1..21)).collect();
    heap_sort::heap_sort::heapsort(&mut vec);
    println!("{:?}", vec);
}
