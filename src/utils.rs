/// Returns the next power of 2 that is greater than or equal to `size`.
///
/// # Arguments
///
/// * `size` - The size to calculate the next power of 2 from.
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

/// Returns the width of the node at the given position.
/// Using recurrence relation,
/// ```x = 2*y + 6```
///
/// # Arguments
///
/// * `n` - The position of the node.
pub fn calculate_width(n: u32) -> u32 {
    6 * ((2u32).pow(n) - 1)
}