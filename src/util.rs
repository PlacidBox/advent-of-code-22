// I genuinely can't find a crate that implements this. Maybe I should make one.
pub fn next_permutation<T: Ord>(input: &mut [T]) -> bool {
    if input.len() < 2 {
        // len 0 or 1 input, there are no permutations
        return false;
    }
    // From wikipedia:
    //     The following algorithm generates the next permutation lexicographically after a
    // given permutation. It changes the given permutation in-place.
    //     Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the
    //      permutation is the last permutation.
    //     Find the largest index l greater than k such that a[k] < a[l].
    //     Swap the value of a[k] with that of a[l].
    //     Reverse the sequence from a[k + 1] up to and including the final element a[n].

    // Find the largest index k such that a[k] < a[k + 1]. If no such index exists, the permutation
    // is the last permutation.
    let mut k = None;
    for i in 0..input.len() - 1 {
        if input[i] < input[i + 1] {
            k = Some(i);
        }
    }

    let Some(k) = k else {
        // Last permutation was reached, the input is in reverse order lexographically so reverse it
        // to get it back to sorted, and return `false` to indicate that we're done.
        input.reverse();
        return false;
    };

    // Find the largest index l greater than k such that a[k] < a[l].
    let mut l = None;
    for i in k + 1..input.len() {
        if input[k] < input[i] {
            l = Some(i);
        }
    }
    // this should be guaranteed to be non-None, since we already checked when deciding k.
    // It's something to clean up when publishing as a crate, though.
    let l = l.unwrap();

    // Swap the value of a[k] with that of a[l].
    input.swap(k, l);

    // Reverse the sequence from a[k + 1] up to and including the final element a[n].
    input[k + 1..].reverse();

    true
}

#[cfg(test)]
mod tests {
    use super::next_permutation;
    #[test]
    fn test_next_perm() {
        let mut list = [0, 1, 2];

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [0, 2, 1]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [1, 0, 2]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [1, 2, 0]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [2, 0, 1]);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, [2, 1, 0]);

        assert_eq!(next_permutation(&mut list), false);
        assert_eq!(list, [0, 1, 2]);
    }

    #[test]
    fn test_next_perm_dupes() {
        let mut list = ['a', 'a', 'b'];

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, ['a', 'b', 'a']);

        assert_eq!(next_permutation(&mut list), true);
        assert_eq!(list, ['b', 'a', 'a']);

        assert_eq!(next_permutation(&mut list), false);
        assert_eq!(list, ['a', 'a', 'b']);
    }
}
