//! Object Pool.
//!
//! The goal of an object pool is to reuse expensive to allocate objects or frequently allocated objects.
//!
use std::iter::Iterator;

/// Forward Only Object Pool.
///
/// # Examples
///
/// ```no_run
/// use nuki::object_pool::*;
///
/// #[repr(C)]
/// #[derive(Debug, Default)]
/// struct ObjectFoo {
///     base: PoolObjectBase,
///     val: u64,
/// }
///
/// impl PoolObjectTypeId for ObjectFoo {
///     fn pool_object_type_id(&self) -> usize {
///         12345678
///     }
/// }
///
/// impl ObjectFoo {
///     fn base(&self) -> &PoolObjectBase {
///         &self.base
///     }
///
///     fn base_mut(&mut self) -> &mut PoolObjectBase {
///         &mut self.base
///     }
/// }
///
/// let mut pool = ForwardPool::with_capacity(1024);
/// if let Some(foo) = pool.alloc::<ObjectFoo>(None) {
///     assert_eq!(foo.base().pool_object_type(), 12345678);
///     assert_eq!(
///         foo.base().pool_object_size(),
///         std::mem::size_of::<ObjectFoo>()
///     );
/// }
/// ```
#[derive(Debug)]
pub struct ForwardPool {
    buf: Vec<u8>,
    offset: usize,
}

impl ForwardPool {
    /// Create the poll with capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            offset: 0,
        }
    }

    /// Returns a raw pointer to the vector's buffer.
    /// # Safety
    pub unsafe fn as_ptr(&self) -> *const u8 {
        self.buf.as_ptr()
    }

    /// Returns an unsafe mutable pointer to the vector's buffer.
    /// # Safety
    pub unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    /// Return available bytes.
    pub fn space(&self) -> usize {
        self.buf.capacity() - self.offset
    }

    /// Return used bytes.
    pub fn used(&self) -> usize {
        self.offset
    }

    /// Allocate an object and return the mutable reference.
    pub fn alloc<T: PoolObjectTypeId + Default>(&mut self, def_val: Option<T>) -> Option<&mut T> {
        unsafe {
            if self.buf.capacity() - self.offset < std::mem::size_of::<T>() {
                None
            } else {
                let size = std::mem::size_of::<T>();
                let ptr = self.buf.as_mut_ptr().add(self.offset);
                let base = &mut *(ptr as *mut PoolObjectBase);
                let obj = &mut *(ptr as *mut T);
                *obj = def_val.unwrap_or_default();
                base.set_pool_object_type(obj.pool_object_type_id());
                base.set_pool_object_size(size);
                self.offset += size;
                Some(obj)
            }
        }
    }

    /// Clear all objects in the pool.
    pub fn clear(&mut self) {
        self.offset = 0;
    }

    /// Returns an iterator over the objects.
    pub fn iter(&self) -> ForwardPoolIter<'_> {
        ForwardPoolIter::new(self)
    }

    /// Returns an iterator that allows modifying each object.
    pub fn iter_mut(&mut self) -> ForwardPoolIterMut<'_> {
        ForwardPoolIterMut::new(self)
    }
}

/// Pooled Object Iterator.
#[derive(Debug)]
pub struct ForwardPoolIter<'pool> {
    pool: &'pool ForwardPool,
    cursor: usize,
}

impl<'pool> Iterator for ForwardPoolIter<'pool> {
    type Item = &'pool dyn PoolObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.pool.used() {
            None
        } else {
            let base = unsafe { &*(self.pool.as_ptr().add(self.cursor) as *const PoolObjectBase) };
            self.cursor += base.pool_object_size();
            Some(base)
        }
    }
}

impl<'pool> ForwardPoolIter<'pool> {
    pub fn new(pool: &'pool ForwardPool) -> Self {
        Self { pool, cursor: 0 }
    }
}

/// Mutable Pooled Object Iterator.
#[derive(Debug)]
pub struct ForwardPoolIterMut<'pool> {
    pool: &'pool mut ForwardPool,
    cursor: usize,
}

impl<'pool> Iterator for ForwardPoolIterMut<'pool> {
    type Item = &'pool mut dyn PoolObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.pool.used() {
            None
        } else {
            let base =
                unsafe { &mut *(self.pool.as_mut_ptr().add(self.cursor) as *mut PoolObjectBase) };
            self.cursor += base.pool_object_size();
            Some(base)
        }
    }
}

impl<'pool> ForwardPoolIterMut<'pool> {
    pub fn new(pool: &'pool mut ForwardPool) -> Self {
        Self { pool, cursor: 0 }
    }
}

/// Base attributes of the Pooled Object.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PoolObjectBase {
    type_: u16,
    size: u16,
}

/// Pooled Object Abstract.
pub trait PoolObject {
    /// Returns the reference to the `base`.
    fn pool_object_base(&self) -> &PoolObjectBase;

    /// Returns the mutable reference to the `base`.
    fn pool_object_base_mut(&mut self) -> &mut PoolObjectBase;

    /// Returns the type of the pooled object.
    fn pool_object_type(&self) -> usize;

    /// Set the type of the pooled object.
    fn set_pool_object_type(&mut self, type_: usize);

    /// Returns the size of the pooled object.
    fn pool_object_size(&self) -> usize;

    /// Set the size of the pooled object.
    fn set_pool_object_size(&mut self, size: usize);
}

impl PoolObject for PoolObjectBase {
    fn pool_object_base(&self) -> &PoolObjectBase {
        self
    }

    fn pool_object_base_mut(&mut self) -> &mut PoolObjectBase {
        self
    }

    fn pool_object_type(&self) -> usize {
        self.type_ as usize
    }

    fn set_pool_object_type(&mut self, type_: usize) {
        self.type_ = type_ as u16
    }

    fn pool_object_size(&self) -> usize {
        self.size as usize
    }

    fn set_pool_object_size(&mut self, size: usize) {
        self.size = size as u16
    }
}

/// TypeId of the Pooled Object.
pub trait PoolObjectTypeId {
    /// Returns the TypeId of the target Type.
    fn pool_object_type_id(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    #[derive(Debug, Default)]
    struct ObjectFoo {
        base: PoolObjectBase,
        val: u64,
    }

    #[repr(C)]
    #[derive(Debug, Default)]
    struct ObjectBar {
        base: PoolObjectBase,
        vals: [u64; 8],
    }

    enum ObjectType {
        Unknown,
        Foo,
        Bar,
        FooBar,
    }

    impl From<usize> for ObjectType {
        fn from(val: usize) -> Self {
            match val {
                1 => ObjectType::Foo,
                2 => ObjectType::Bar,
                3 => ObjectType::FooBar,
                _ => ObjectType::Unknown,
            }
        }
    }

    impl PoolObjectTypeId for ObjectFoo {
        fn pool_object_type_id(&self) -> usize {
            ObjectType::Foo as usize
        }
    }

    impl ObjectFoo {
        fn base(&self) -> &PoolObjectBase {
            &self.base
        }

        #[allow(dead_code)]
        fn base_mut(&mut self) -> &mut PoolObjectBase {
            &mut self.base
        }
    }

    impl PoolObjectTypeId for ObjectBar {
        fn pool_object_type_id(&self) -> usize {
            ObjectType::Bar as usize
        }
    }

    impl ObjectBar {
        fn base(&self) -> &PoolObjectBase {
            &self.base
        }

        #[allow(dead_code)]
        fn base_mut(&mut self) -> &mut PoolObjectBase {
            &mut self.base
        }
    }

    #[test]
    fn test_forward_pool() {
        let mut pool = ForwardPool::with_capacity(1024);

        loop {
            match pool.alloc::<ObjectFoo>(None) {
                Some(obj) => {
                    assert_eq!(obj.base().pool_object_type(), ObjectType::Foo as usize);
                    assert_eq!(
                        obj.base().pool_object_size(),
                        std::mem::size_of::<ObjectFoo>()
                    );
                }
                None => {
                    break;
                }
            }
            match pool.alloc::<ObjectBar>(None) {
                Some(bar) => {
                    assert_eq!(bar.base().pool_object_type(), ObjectType::Bar as usize);
                    assert_eq!(
                        bar.base().pool_object_size(),
                        std::mem::size_of::<ObjectBar>()
                    );
                }
                None => {
                    break;
                }
            }
        }

        for o in pool.iter() {
            assert!(o.pool_object_type() != ObjectType::Unknown as usize);
            assert!(o.pool_object_size() > 0);
            match ObjectType::from(o.pool_object_type()) {
                ObjectType::Foo => {
                    let t = unsafe {
                        std::mem::transmute::<*const PoolObjectBase, &ObjectFoo>(
                            o.pool_object_base(),
                        )
                    };
                    assert_eq!(o.pool_object_type(), t.base().pool_object_type());
                    assert_eq!(o.pool_object_size(), t.base().pool_object_size());
                }
                _ => {
                    println!("Unsupported {} yet", o.pool_object_type());
                }
            }
        }
    }
}
