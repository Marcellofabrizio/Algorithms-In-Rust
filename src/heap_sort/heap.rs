pub struct HeapMax {
    heap_array: [i32; 1000],
}

impl HeapMax {
    pub fn new(heap_array: [i32; 1000]) -> HeapMax {
        HeapMax {
            heap_array: (heap_array),
        }
    }
}
