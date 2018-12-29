use std::fmt::Debug;

pub fn quicksort<T: PartialOrd + Debug>(slice: &mut [T]) {
    if slice.len() > 2 {
        let mut left= 1;
        let mut right = slice.len() - 1;

        while left < right {
            while left < slice.len() && slice[left] <= slice[0] {
                left += 1;
            }

            while right > 0 && slice[right] >= slice[0] {
                right -= 1;
            }

            if left < right {
                slice.swap(left, right);
            }
        }

        slice.swap( 0, right);

        let length = slice.len();
        quicksort(&mut slice[0..right]);
        quicksort(&mut slice[right+1..length])
    } else if slice.len() == 2 {
        if slice[0] > slice[1] {
            slice.swap(0, 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;
    use rand::Rng;

    fn assert_sorted<T: PartialOrd + Display>(slice: &[T]) {
        for i in 0..slice.len()-1 {
            assert!(slice[i] <= slice[i+1], "Element {} at index {} is not smaller than element {} at index {}", slice[i], i, slice[i+1], i+1);
        }
    }

    #[test]
    fn test_sort_single_element() {
        let mut input = vec![1];
        quicksort(&mut input);

        assert_sorted(&input);
    }

    #[test]
    fn test_sort_two_elements() {
        let mut input = vec![2, 1];
        quicksort(&mut input);

        assert_sorted(&input);
    }

    #[test]
    fn test_sort_random_vector() {
        let mut rng = rand::thread_rng();

        let mut input: Vec<u64> = (0..100).map(|_| rng.gen_range(0, 20)).collect();

        println!("Input {:?}", input);

        quicksort(&mut input);

        println!("Output {:?}", input);

        assert_sorted(&input);
    }

}
