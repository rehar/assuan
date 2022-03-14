use std::ptr;

pub(crate) trait Ptr {
    type Inner;
}

impl<T> Ptr for *mut T {
    type Inner = T;
}

impl<T> Ptr for *const T {
    type Inner = T;
}

pub(crate) type NonNull<T> = ptr::NonNull<<T as Ptr>::Inner>;
