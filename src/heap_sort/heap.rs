pub struct HeapMax {
    heap_array: Vec<i32>,
    heap_size: usize,
}

impl HeapMax {
    pub fn new(array: Vec<i32>, array_size: usize) -> HeapMax {
        HeapMax {
            heap_array: (array),
            heap_size: array_size,
        }
    }

    pub fn print_array(&self) {
        println!("{:?}", self.heap_array);
    }

    pub fn heapsort(&self, arr: &[u32]) {
        build_max_heap(arr);
        for i in (1..arr.len()).rev() {
            arr.swap(0, i);
        }
    }

    pub fn build_max_heap(&self, arr: &[u32]) {
        for i in (0..arr.len()).rev() {
            heapify(arr, i);
        }
    }

    fn left(i: usize) {
        return 2 * i + 1;
    }

    fn right(i: usize) {
        return 2 * i + 2;
    }

    fn max_heapify(&mut self, i: i32) {
        let l: i32 = HeapMax::left(i);
        let r: i32 = HeapMax::right(i);

        println!("Index {}", i);
        println!("Left {}", l);
        println!("Right {}", r);
        println!("Heap-Size {}", self.heap_size);

        let mut largest = i;

        if l < self.heap_size as i32 && self.heap_array[l as usize] > self.heap_array[r as usize] {
            largest = l;
        }

        if r < self.heap_size as i32
            && self.heap_array[r as usize] > self.heap_array[largest as usize]
        {
            largest = r;
        }

        if largest != i {
            self.heap_array[i as usize] = self.heap_array[largest as usize];
            self.max_heapify(largest);
        }
    }

    fn parent(i: i32) -> i32 {
        return i32::abs(i / 2);
    }

    fn left(i: i32) -> i32 {
        return i * 2;
    }

    fn right(i: i32) -> i32 {
        return (i * 2) + 1;
    }
}
