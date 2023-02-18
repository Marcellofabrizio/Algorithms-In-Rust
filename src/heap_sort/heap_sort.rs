pub fn heapsort(arr: &mut [i32]) {
    build_max_heap(arr);
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn build_max_heap(arr: &mut [i32]) {
    for i in (0..arr.len()).rev() {
        heapify(arr, arr.len(), i);
    }
}

fn heapify(arr: &mut [i32], n: usize, i: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = i;

    if l < n && arr[l] > arr[largest] {
        largest = l;
    }

    if r < n && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn left(i: usize) -> usize {
    return 2 * i + 1;
}

fn right(i: usize) -> usize {
    return 2 * i + 2;
}
