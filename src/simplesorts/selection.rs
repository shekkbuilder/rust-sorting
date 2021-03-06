//! Selection sort algorithm.
//!
//! The `selection` module contains the simple sorting algorithm "Selection Sort".
//!
//! Source: https://en.wikipedia.org/wiki/Selection_sort

extern crate libc;

use std::slice;

/// Simple sort: selection sort.
///
/// # Details
///
/// Selection sort is one of the simplest sorting algorithm. It sorts a vector in place by
/// looping over all elements and swapping the smallest element of the rest of the vector.
///
/// # Scaling
///
/// Selection sort in scale as O(N^2) since for all elements to sort, the algorithm will compare it
/// with every (remaining) elements.
///
/// Insertion sort is still useful as it is quite simple.
///
/// # Optimizations
///
/// None
///
/// # Notes
///
/// The type T of the vector elements to sort _must_ implement the `PartialOrd` trait so the
/// compiler knows how to compare the elements and sort them.
///
/// # Examples
///
/// ```
/// let mut data: Vec<i32> = vec![4, 2, 3, 1, 5];
/// sorting::simplesorts::selection::sort(&mut data);
/// assert_eq!(vec![1, 2, 3, 4, 5], data);
/// ```
///
pub fn sort<T: PartialOrd>(array: &mut [T]) {

    if !array.is_empty() {
        // Let's loop over the input array, skipping last value (important as we want to get a
        // slice of the following values).
        let n = array.len();
        for i in 0..n-1 {
            // Find index of the minimum value in the slice of values following the i^th element.
            let j = {
                let int_slice = &array[i+1..];
                // k now holds the index (of "array") of the minimum element of "int_slice", or the
                // value "i" of the current element of "array" (which should be the less than
                // all elements of "int_slice").
                // "k" is a tuple containing the smallest element of "int_slice" and its index.
                //     k.0: Index of "int_slice" of its smallest value.
                //     k.1: Smallest value of "int_slice".
                let k = int_slice.iter().enumerate().fold(
                    (0, int_slice.first().unwrap()), |acc, item| {
                        if acc.1 > item.1 { item } else { acc }
                });

                // Return from the block the smallest element between the one at index "i" of
                // the "array" and the smallest one from the "int_slice" array.
                // NOTE: Since "k" worked on the subarray "int_slice", it's first element is the
                //       i^th element of "array". If "k" is actually smaller than "array[i]", we
                //       need to adapt the index as stored in "k.0".
                if k.1 < &array[i] { i + k.0 + 1 } else { i }
            };

            array.swap(i, j);
        }
    }
}




#[no_mangle]
pub extern "C" fn ffi_selectionsort_i8(array_pointer: *const libc::int8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_selectionsort_i16(array_pointer: *const libc::int16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_selectionsort_i32(array_pointer: *const libc::int32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_selectionsort_i64(array_pointer: *const libc::int64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}


#[no_mangle]
pub extern "C" fn ffi_selectionsort_u8(array_pointer: *const libc::uint8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_selectionsort_u16(array_pointer: *const libc::uint16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_selectionsort_u32(array_pointer: *const libc::uint32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_selectionsort_u64(array_pointer: *const libc::uint64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u64, n as usize)
    };
    sort(&mut to_sort);
}

#[no_mangle]
pub extern "C" fn ffi_selectionsort_f32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_selectionsort_f64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}
