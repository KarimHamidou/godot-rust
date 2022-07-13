use std::iter::{Extend, FromIterator};

use crate::private::get_api;
use crate::sys;

use crate::RefCounted;
use crate::ToVariant;
use crate::Variant;

use std::fmt;

/// A reference-counted `Variant` vector. Godot's generic array data type.
/// Negative indices can be used to count from the right.
pub struct VariantArray(pub(crate) sys::godot_array);

impl VariantArray {
    /// Creates an empty `VariantArray`.
    #[inline]
    pub fn new() -> Self {
        VariantArray::default()
    }

    /// Sets the value of the element at the given offset.
    #[inline]
    pub fn set(&mut self, idx: i32, val: &Variant) {
        unsafe { (get_api().godot_array_set)(&mut self.0, idx, &val.0) }
    }

    /// Returns a copy of the element at the given offset.
    #[inline]
    pub fn get_val(&mut self, idx: i32) -> Variant {
        unsafe { Variant((get_api().godot_array_get)(&self.0, idx)) }
    }

    /// Returns a reference to the element at the given offset.
    #[inline]
    pub fn get_ref(&self, idx: i32) -> &Variant {
        unsafe { Variant::cast_ref((get_api().godot_array_operator_index_const)(&self.0, idx)) }
    }

    /// Returns a mutable reference to the element at the given offset.
    #[inline]
    pub fn get_mut_ref(&mut self, idx: i32) -> &mut Variant {
        unsafe { Variant::cast_mut_ref((get_api().godot_array_operator_index)(&mut self.0, idx)) }
    }

    #[inline]
    pub fn count(&mut self, val: &Variant) -> i32 {
        unsafe { (get_api().godot_array_count)(&mut self.0, &val.0) }
    }

    /// Clears the array, resizing to 0.
    #[inline]
    pub fn clear(&mut self) {
        unsafe {
            (get_api().godot_array_clear)(&mut self.0);
        }
    }

    #[inline]
    pub fn remove(&mut self, idx: i32) {
        unsafe { (get_api().godot_array_remove)(&mut self.0, idx) }
    }

    #[inline]
    pub fn erase(&mut self, val: &Variant) {
        unsafe { (get_api().godot_array_erase)(&mut self.0, &val.0) }
    }

    /// Returns `true` if the `VariantArray` contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        unsafe { (get_api().godot_array_empty)(&self.0) }
    }

    /// Returns the number of elements in the array.
    #[inline]
    pub fn len(&self) -> i32 {
        unsafe { (get_api().godot_array_size)(&self.0) }
    }

    /// Appends an element at the end of the array.
    #[inline]
    pub fn push(&mut self, val: &Variant) {
        unsafe {
            (get_api().godot_array_push_back)(&mut self.0, &val.0);
        }
    }

    /// Removes an element at the end of the array.
    #[inline]
    pub fn pop(&mut self) -> Variant {
        unsafe { Variant((get_api().godot_array_pop_back)(&mut self.0)) }
    }

    /// Appends an element to the front of the array.
    #[inline]
    pub fn push_front(&mut self, val: &Variant) {
        unsafe {
            (get_api().godot_array_push_front)(&mut self.0, &val.0);
        }
    }

    /// Removes an element at the front of the array.
    #[inline]
    pub fn pop_front(&mut self) -> Variant {
        unsafe { Variant((get_api().godot_array_pop_front)(&mut self.0)) }
    }

    /// Insert a new int at a given position in the array.
    #[inline]
    pub fn insert(&mut self, at: i32, val: &Variant) {
        unsafe { (get_api().godot_array_insert)(&mut self.0, at, &val.0) }
    }

    /// Searches the array for a value and returns its index.
    /// Pass an initial search index as the second argument.
    /// Returns `-1` if value is not found.
    #[inline]
    pub fn find(&self, what: &Variant, from: i32) -> i32 {
        unsafe { (get_api().godot_array_find)(&self.0, &what.0, from) }
    }

    /// Returns true if the `VariantArray` contains the specified value.
    #[inline]
    pub fn contains(&self, what: &Variant) -> bool {
        unsafe { (get_api().godot_array_has)(&self.0, &what.0) }
    }

    #[inline]
    pub fn resize(&mut self, size: i32) {
        unsafe { (get_api().godot_array_resize)(&mut self.0, size) }
    }

    /// Searches the array in reverse order.
    /// Pass an initial search index as the second argument.
    /// If negative, the start index is considered relative to the end of the array.
    #[inline]
    pub fn rfind(&self, what: &Variant, from: i32) -> i32 {
        unsafe { (get_api().godot_array_rfind)(&self.0, &what.0, from) }
    }

    /// Searches the array in reverse order for a value.
    /// Returns its index or `-1` if not found.
    #[inline]
    pub fn find_last(&self, what: &Variant) -> i32 {
        unsafe { (get_api().godot_array_find_last)(&self.0, &what.0) }
    }

    /// Inverts the order of the elements in the array.
    #[inline]
    pub fn invert(&mut self) {
        unsafe { (get_api().godot_array_invert)(&mut self.0) }
    }

    /// Return a hashed i32 value representing the array contents.
    #[inline]
    pub fn hash(&self) -> i32 {
        unsafe { (get_api().godot_array_hash)(&self.0) }
    }

    #[inline]
    pub fn sort(&mut self) {
        unsafe { (get_api().godot_array_sort)(&mut self.0) }
    }

    // TODO
    // pub fn sort_custom(&mut self, obj: ?, s: ?) {
    //     unimplemented!()
    // }

    // pub fn bsearch(&mut self, val: (), before: bool) -> i32 {
    //     unsafe {
    //         (get_api().godot_array_bsearch)(&mut self.0, val, before)
    //     }
    // }

    // pub fn bsearch_custom(&mut self, val: ?, obj: ?, s: ?, before: bool) -> i32 {
    //     unimplemented!();
    // }

    #[inline]
    pub fn iter(&self) -> Iter {
        Iter {
            arr: self,
            range: 0..self.len(),
        }
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut {
        let len = self.len();
        IterMut {
            arr: self,
            range: 0..len,
        }
    }

    #[doc(hidden)]
    #[inline]
    pub fn sys(&self) -> *const sys::godot_array {
        &self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_sys(sys: sys::godot_array) -> Self {
        VariantArray(sys)
    }
}

impl RefCounted for VariantArray {
    impl_common_methods! {
        #[inline]
        fn new_ref(&self) -> VariantArray : godot_array_new_copy;
    }
}

impl_basic_traits!(
    for VariantArray as godot_array {
        Drop => godot_array_destroy;
        Default => godot_array_new;
    }
);

impl fmt::Debug for VariantArray {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

pub struct Iter<'a> {
    arr: &'a VariantArray,
    range: std::ops::Range<i32>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Variant;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|idx| self.arr.get_ref(idx))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

pub struct IterMut<'a> {
    arr: &'a mut VariantArray,
    range: std::ops::Range<i32>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Variant;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|idx| {
            let short_ref: &'_ mut Variant = self.arr.get_mut_ref(idx);
            unsafe { std::mem::transmute::<_, &'a mut Variant>(short_ref) }
        })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T: ToVariant> FromIterator<T> for VariantArray {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut arr = Self::new();
        arr.extend(iter);
        arr
    }
}

impl<T: ToVariant> Extend<T> for VariantArray {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.push(&elem.to_variant());
        }
    }
}

godot_test!(test_array {
    let foo = Variant::from_str("foo");
    let bar = Variant::from_str("bar");
    let nope = Variant::from_str("nope");

    let mut array = VariantArray::new(); // []

    assert!(array.is_empty());
    assert_eq!(array.len(), 0);

    array.push(&foo); // [&foo]
    array.push(&bar); // [&foo, &bar]

    assert_eq!(array.len(), 2);

    assert!(array.contains(&foo));
    assert!(array.contains(&bar));
    assert!(!array.contains(&nope));

    array.set(0, &bar); // [&bar, &bar]
    array.set(1, &foo); // [&bar, &foo]

    assert_eq!(array.get_ref(0), &bar);
    assert_eq!(array.get_ref(1), &foo);

    array.pop(); // [&bar]
    array.pop(); // []

    let x = Variant::from_i64(42);
    let y = Variant::from_i64(1337);
    let z = Variant::from_i64(512);

    array.insert(0, &x); // [&x]
    array.insert(0, &y); // [&y, &x]
    array.push_front(&z); // [&y, &x]
    array.push_front(&z); // [&z, &z, &y, &x]

    assert_eq!(array.find(&y, 0), 2);
    assert_eq!(array.find_last(&z), 1);
    assert_eq!(array.find(&nope, 0), -1);

    array.invert(); // [&x, &y, &z, &z]

    assert_eq!(array.get_ref(0), &x);

    array.pop_front(); // [&y, &z, &z]
    array.pop_front(); // [&z, &z]

    assert_eq!(array.get_ref(0), &z);

    array.resize(0); // []
    assert!(array.is_empty());

    array.push(&foo); // [&foo]
    array.push(&bar); // [&foo, &bar]

    let array2 = array.new_ref();
    assert!(array2.contains(&foo));
    assert!(array2.contains(&bar));
    assert!(!array2.contains(&nope));

    let mut array3 = VariantArray::new(); // []

    array3.push(&Variant::from_i64(42));
    array3.push(&Variant::from_i64(1337));
    array3.push(&Variant::from_i64(512));

    assert_eq!(
        &[42, 1337, 512],
        array3.iter().map(|v| v.try_to_i64().unwrap()).collect::<Vec<_>>().as_slice(),
    );

    for v in array3.iter_mut() {
        *v = Variant::from_i64(54);
    }

    assert_eq!(
        &[54, 54, 54],
        array3.iter().map(|v| v.try_to_i64().unwrap()).collect::<Vec<_>>().as_slice(),
    );
});

godot_test!(
    test_array_debug {
        let mut arr = VariantArray::new(); // []
        arr.push(&Variant::from_str("hello world"));
        arr.push(&Variant::from_bool(true));
        arr.push(&Variant::from_i64(42));

        assert_eq!(format!("{:?}", arr), "[GodotString(hello world), Bool(True), I64(42)]");
    }
);

// TODO: clear arrays without affecting clones
//godot_test!(test_array_clone_clear {
//    let foo = Variant::from_str("foo");
//    let mut array = VariantArray::new();
//
//    array.push(&foo);
//    let array_clone = array.clone();
//    array.clear();
//
//    assert!(array.is_empty());
//    assert!(!array_clone.is_empty());
//});
