fn main() {
    let mut num = 5;

    // no need for unsafe keyword here because
    // we can create raw pointers in safe code;
    // we can't dereference raw pointers outside an unsafe block
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // in order to dereference raw pointers outside an unsafe block
    // use the unsafe keyword
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is {}", *r2);
    }

    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {}

use std::slice;
// sample implementation of split_at_mut
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!( mid <= len );
    
    let ptr = values.as_mut_ptr();
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )

    }

}