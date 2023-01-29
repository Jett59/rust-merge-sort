fn merge<T: Ord + Clone>(array1: &[T], array2: &[T], result: &mut [T]) {
    assert!(array1.len() + array2.len() == result.len());
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

fn merge_sort<T: Ord + Default + Clone>(array: &mut [T]) {
    if array.len() < 2 {
        return;
    }
    let middle = array.len() / 2;
    merge_sort(&mut array[..middle]);
    merge_sort(&mut array[middle..]);
    let mut temp_array = Vec::with_capacity(array.len());
    temp_array.resize(array.len(), Default::default());
    merge(
        &array[..middle],
        &array[middle..],
        temp_array.as_mut_slice(),
    );
    array.clone_from_slice(temp_array.as_slice());
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
    println!("{:?}", array);
}
