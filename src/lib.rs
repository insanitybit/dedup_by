use std::mem;

// Shamelessly ripped from the standard library's implementation of dedup
/// Removes consecutive repeated elements in the vector based on whether the provided funciton 'f'
/// returns true (remove) or false (do not remove).
///
/// See the std documentation for dedup for more details:
/// https://doc.rust-lang.org/src/collections/up/src/libcollections/vec.rs.html#1004
/// # Examples
///
/// ```
/// use dedup_by::dedup_by;
/// let mut vec = vec![(1, 2), (2, 2), (3, 1)];
///
/// dedup_by(&mut vec, |a, b| { a.1 == b.1 });
///
/// assert_eq!(vec, [(1, 2), (3, 1)]);
/// ```
pub fn dedup_by<T, F>(v: &mut Vec<T>, mut f: F)
    where F: FnMut(&T, &T) -> bool
{
    unsafe {
        let ln = v.len();
        if ln <= 1 {
            return;
        }
        // Avoid bounds checks by using raw pointers.
        let p = v.as_mut_ptr();
        let mut r: usize = 1;
        let mut w: usize = 1;

        while r < ln {
            let p_r = p.offset(r as isize);
            let p_wm1 = p.offset((w - 1) as isize);
            if !f(&*p_r, &*p_wm1) {
                if r != w {
                    let p_w = p_wm1.offset(1);
                    mem::swap(&mut *p_r, &mut *p_w);
                }
                w += 1;
            }
            r += 1;
        }

        v.truncate(w);
    }
}

#[cfg(test)]
mod tests {
    use super::dedup_by;

    #[test]
    fn test_dedup_by() {
        let mut subs = vec!["a", "b", "c", "c", "b"];
        let d = vec!["a", "b", "c"];
        subs.sort();
        let mut subs_clone = subs.clone();

        dedup_by(&mut subs, |a, b| a == b);
        subs_clone.dedup();

        assert_eq!(subs, d);
        assert_eq!(subs, subs_clone);
    }
}
