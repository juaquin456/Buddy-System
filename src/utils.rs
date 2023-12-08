pub fn memory_to_allocate(size: usize) -> usize {
    if size == 0 {
        return 0;
    }

    let mut n = size;
    let mut longest_bit = 0;

    loop {
        n >>= 1;
        longest_bit += 1;

        if n == 0 {
            break;
        }
    }
    let potency_of_2: usize = 1 << (longest_bit - 1);
    if potency_of_2 >= size {
        potency_of_2
    }
    else {
        potency_of_2 << 1
    }
}