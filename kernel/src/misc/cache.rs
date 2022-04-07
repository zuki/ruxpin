
use core::ops::Deref;
use core::fmt::Debug;
use core::ptr::NonNull;
use core::marker::PhantomData;
use core::sync::atomic::{self, AtomicUsize, Ordering};

use alloc::vec::Vec;

use crate::printkln;
use crate::misc::linkedlist::{UnownedLinkedList, UnownedLinkedListNode};


pub struct Cache<T> {
    max_size: usize,
    items: Vec<UnownedLinkedListNode<CacheArcInner<T>>>,
    order: UnownedLinkedList<CacheArcInner<T>>,
}

pub struct CacheArc<T> {
    ptr: NonNull<CacheArcInner<T>>,
    _marker: PhantomData<T>,
}

pub struct CacheArcInner<T> {
    refcount: AtomicUsize,
    data: T,
}

impl<T> Cache<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            items: Vec::with_capacity(max_size),
            order: UnownedLinkedList::new(),
        }
    }

    pub fn clear(&mut self) -> Result<(), ()> {
        for i in 0..self.items.len() {
            if self.items[i].refcount.load(Ordering::Relaxed) != 0 {
                return Err(());
            }
        }

        *self = Cache::new(self.max_size);
        Ok(())
    }

    pub fn get<C, F, E>(&mut self, compare: C, fetch: F) -> Result<CacheArc<T>, E>
    where
        C: Fn(&T) -> bool,
        F: FnOnce() -> Result<T, E>
    {
        // Search the list for the matching object
        let mut iter = self.order.iter();
        while let Some(ptr) = iter.next() {
            let item = unsafe { &mut (*ptr.as_ptr()) };
            if compare(&item.data) {
                unsafe {
                    self.order.remove_node(ptr);
                    self.order.insert_head(ptr);
                }
                printkln!("cache: returning existing");
                return Ok(item.wrap_inner());
            }
        }

        // If not every cache entry is in use, then allocate a new one and fetch the object
        if self.items.len() < self.max_size {
            self.items.push(UnownedLinkedListNode::new(CacheArcInner::new(fetch()?)));
            let i = self.items.len() - 1;
            unsafe {
                self.order.insert_head(self.items[i].wrap_non_null());
            }
                printkln!("cache: returning new");
            return Ok(self.items[i].wrap_inner());
        }

        // If the cache is full, then find the last entry in the list that has no references and recycle it
        let mut iter = self.order.iter_rev();
        while let Some(ptr) = iter.next() {
            let item = unsafe { &mut (*ptr.as_ptr()) };
            if item.refcount.load(Ordering::Relaxed) == 0 {
                item.data = fetch()?;
                unsafe {
                    self.order.remove_node(ptr);
                    self.order.insert_head(ptr);
                }
                printkln!("cache: recycling old");
                return Ok(item.wrap_inner());
            }
        }

        panic!("Out of Cache");
    }
}

impl<T: Debug> Cache<T> {
    pub fn print(&mut self) {
        let mut i = 0;
        let mut iter = self.order.iter();
        printkln!("Cache contents:");
        while let Some(ptr) = iter.next() {
            let item = unsafe { &mut (*ptr.as_ptr()) };
            printkln!("{}: {:?}", i, item.data);
            i += 1;
        }
    }
}


impl<T> CacheArc<T> {
    fn from_inner(inner: NonNull<CacheArcInner<T>>) -> Self {
        let inner_data = unsafe { inner.as_ref() };
        let count = inner_data.refcount.fetch_add(1, Ordering::Relaxed);

        if count == isize::MAX as usize {
            panic!("Too many references");
        }

        Self {
            ptr: inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for CacheArc<T> {
    fn clone(&self) -> Self {
        CacheArc::from_inner(self.ptr)
    }
}

impl<T> Deref for CacheArc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Drop for CacheArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        // TODO I have no idea if this is right.  I don't want to decrement the count if it's already 0
        if inner.refcount.load(Ordering::Acquire) != 0 {
            inner.refcount.fetch_sub(1, Ordering::Acquire);
        }
        atomic::fence(Ordering::Release);
        // Don't need to drop inner because it's stored in the Vec in Cache<T>
    }
}

unsafe impl<T: Sync + Send> Send for CacheArc<T> {}
unsafe impl<T: Sync + Send> Sync for CacheArc<T> {}


impl<T> CacheArcInner<T> {
    fn new(data: T) -> Self {
        Self {
            refcount: AtomicUsize::new(0),
            data,
        }
    }

    fn wrap_inner(&mut self) -> CacheArc<T> {
        CacheArc::from_inner(NonNull::new(self.as_ptr()).unwrap())
    }

    fn as_ptr(&mut self) -> *mut CacheArcInner<T> {
        self as *mut CacheArcInner<T>
    }
}

