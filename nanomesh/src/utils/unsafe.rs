use std::ptr;

pub unsafe fn ptr_to_vec<T>(ptr: *const T, len: usize) -> Vec<T> {
    let mut vec = Vec::with_capacity(len);
    ptr::copy(ptr, vec.as_mut_ptr(), len);
    vec.set_len(len);
    return vec;
}

// Don't forget to free the memory after usage!
pub unsafe fn vec_to_ptr<T>(vec: &Vec<T>) -> *mut T {
    let len = vec.len();
    let ptr: *mut T = libc::malloc(std::mem::size_of::<T>() * len) as *mut T;
    ptr::copy(vec.as_ptr(), ptr, len);
    return ptr;
}

#[cfg(test)]
mod tests {

    #[test]
    fn vec_to_ptr() {
        let vec = vec![0, 1, 2, 3];
        
        unsafe {
            let ptr = super::vec_to_ptr(&vec);

            assert_eq!(*ptr.add(0), 0);
            assert_eq!(*ptr.add(1), 1);
            assert_eq!(*ptr.add(2), 2);
            assert_eq!(*ptr.add(3), 3);

            //let vec2 = ptr_to_vec(ptr, vec.len());
            libc::free(ptr as *mut libc::c_void);
        }
    }

    #[test]
    fn ptr_to_vec() {
        unsafe {
            let ptr: *mut i32 = libc::malloc(std::mem::size_of::<i32>() * 4) as *mut i32;
            *ptr.add(0) = 0;
            *ptr.add(1) = 1;
            *ptr.add(2) = 2;
            *ptr.add(3) = 3;

            let vec = super::ptr_to_vec(ptr, 4);

            libc::free(ptr as *mut libc::c_void);

            assert_eq!(vec[0], 0);
            assert_eq!(vec[1], 1);
            assert_eq!(vec[2], 2);
            assert_eq!(vec[3], 3);
        }
    }
}
