#![feature(test)]

extern crate test;

fn merge<T: Ord + Clone>(array1: &[T], array2: &[T], result: &mut [T]) {
    debug_assert!(array1.len() + array2.len() == result.len());
    let mut result_index = 0;
    let mut array1_index = 0;
    let mut array2_index = 0;
    while array1_index < array1.len() && array2_index < array2.len() {
        if array1[array1_index] < array2[array2_index] {
            result[result_index] = array1[array1_index].clone();
            array1_index += 1;
        } else {
            result[result_index] = array2[array2_index].clone();
            array2_index += 1;
        }
        result_index += 1;
    }
    if array1_index < array1.len() {
        for element in array1.iter().skip(array1_index) {
            result[result_index] = element.clone();
            result_index += 1;
        }
    } else {
        for element in array2.iter().skip(array2_index) {
            result[result_index] = element.clone();
            result_index += 1;
        }
    }
}

pub fn merge_sort_impl<T: Ord + Default + Clone>(array: &mut [T], temp_array: &mut [T]) {
    if array.len() < 2 {
        return;
    }
    if array.len() == 2 {
        if array[0] > array[1] {
            array.swap(0, 1);
        }
        return;
    }
    let middle = array.len() / 2;
    merge_sort_impl(&mut array[..middle], temp_array);
    merge_sort_impl(&mut array[middle..], temp_array);
    merge(
        &array[..middle],
        &array[middle..],
        &mut temp_array[..array.len()],
    );
    array.clone_from_slice(&temp_array[..array.len()]);
}

fn merge_sort<T: Ord + Default + Clone>(array: &mut [T]) {
    let mut temp_array = vec![T::default(); array.len()];
    merge_sort_impl(array, temp_array.as_mut_slice());
}

fn main() {
    let mut array = [
        9999,
        -1,
        0,
        999,
        -99999,
        7777,
        2222,
        1111,
        666,
        2323,
        121,
        454,
        88,
        1919191919,
        75000,
        3,
        -314,
        10,
        50000000000i64,
        -5000000000i64,
        73,
        73,
        84,
        91,
        -11,
        17,
        9,
        1,
    ];
    merge_sort(&mut array);
    println!("{array:?}");
}

pub struct SimpleRandom(u32);

impl SimpleRandom {
    pub fn next(&mut self, max: u32) -> u32 {
        let mut result = self.0;
        result ^= result << 13;
        result ^= result >> 17;
        result ^= result << 5;
        self.0 = result;
        result % max
    }
}

#[cfg(test)]
mod mergesort_test {
    use super::*;

    use test::Bencher;

    #[bench]
    fn many_elements(bencher: &mut Bencher) {
        let mut random = SimpleRandom(17);
        let element_count = 1000000;
        let mut array = Vec::with_capacity(element_count as usize);
        for _i in 0..element_count {
            array.push(random.next(1000000));
        }
        bencher.iter(|| {
            let mut array = array.clone();
            merge_sort(&mut array);
        });
    }

    #[bench]
    fn some_elements(bencher: &mut Bencher) {
        let mut random = SimpleRandom(31);
        let element_count = 50000;
        let mut array = Vec::with_capacity(element_count as usize);
        for _i in 0..element_count {
            array.push(random.next(1000000));
        }
        bencher.iter(|| {
            let mut array = array.clone();
            merge_sort(&mut array);
        });
    }

    #[bench]
    fn few_elements(bencher: &mut Bencher) {
        let mut random = SimpleRandom(127);
        let element_count = 500;
        let mut array = Vec::with_capacity(element_count as usize);
        for _i in 0..element_count {
            array.push(random.next(1000000));
        }
        bencher.iter(|| {
            let mut array = array.clone();
            merge_sort(&mut array);
        });
    }
}

#[cfg(test)]
mod builtin_sort {
    use super::*;

    use test::Bencher;

    #[bench]
    fn many_elements(bencher: &mut Bencher) {
        let mut random = SimpleRandom(17);
        let element_count = 1000000;
        let mut array = Vec::with_capacity(element_count as usize);
        for _i in 0..element_count {
            array.push(random.next(1000000));
        }
        bencher.iter(|| {
            let mut array = array.clone();
            array.sort();
        });
    }

    #[bench]
    fn some_elements(bencher: &mut Bencher) {
        let mut random = SimpleRandom(31);
        let element_count = 50000;
        let mut array = Vec::with_capacity(element_count as usize);
        for _i in 0..element_count {
            array.push(random.next(1000000));
        }
        bencher.iter(|| {
            let mut array = array.clone();
            array.sort();
        });
    }

    #[bench]
    fn few_elements(bencher: &mut Bencher) {
        let mut random = SimpleRandom(127);
        let element_count = 500;
        let mut array = Vec::with_capacity(element_count as usize);
        for _i in 0..element_count {
            array.push(random.next(1000000));
        }
        bencher.iter(|| {
            let mut array = array.clone();
            array.sort();
        });
    }
}
