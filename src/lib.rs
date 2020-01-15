/// Generate permutations in increasing order.
pub struct PermuteSorted <T> {
    next_val: Option<Vec<T>>,
}

impl <T: Clone + PartialOrd> PermuteSorted  <T> {
    /// Must pass next_val presorted in non-decreasing order
    pub fn new (next_val: Vec<T>) -> Self {
        PermuteSorted  { next_val: Some(next_val) }
    }

}

impl<T: Clone + PartialOrd> Iterator for PermuteSorted <T> {
    type Item = Vec<T>;

    fn next (&mut self) -> Option<Vec<T>> {
        self.next_val.take().map(|val| {
            let mut copy = val.clone();
            let len = copy.len();
            let mut found = false;
            for i in (0..len - 1).rev() {
                if copy[i] < copy[i+1] {
                    found = true;
                    let mut swap = i+1;
                    for j in swap..len {
                        if copy[i] < copy[j] {
                            swap = j
                        }
                    }
                    copy.swap(i, swap);
                    copy[i+1..len].reverse();
                    break;
                }
            }
            self.next_val = if found {
                Some(copy)
            } else {
                None
            };
            val
        })
    }
}

#[cfg(test)]
mod lib_test  {
    use super::PermuteSorted ;
    #[test]
    fn basic() {
        let mut perm = PermuteSorted ::new(vec![1.1, 2.2, 3.3]);
        assert_eq!(perm.next(), Some(vec![1.1, 2.2, 3.3]));
        assert_eq!(perm.next(), Some(vec![1.1, 3.3, 2.2]));

        let input = vec![1, 2, 3, 4, 5, 6, 7];
        let mut perm = PermuteSorted ::new(input);
        assert_eq!(perm.next(), Some(vec![1, 2, 3, 4, 5, 6, 7]));
        assert_eq!(perm.next(), Some(vec![1, 2, 3, 4, 5, 7, 6]));
        assert_eq!(perm.next(), Some(vec![1, 2, 3, 4, 6, 5, 7]));

        let mut perm = PermuteSorted ::new(vec![1, 2, 2]);
        assert_eq!(perm.next(), Some(vec![1, 2, 2]));
        assert_eq!(perm.next(), Some(vec![2, 1, 2]));
        assert_eq!(perm.next(), Some(vec![2, 2, 1]));
        assert_eq!(perm.next(), None);

        let mut perm = PermuteSorted ::new(vec![std::f64::NAN, std::f64::NAN, 1.1, 2.2]);
        assert!(vec_compare(&perm.next().unwrap(), &vec![ std::f64::NAN, std::f64::NAN, 1.1, 2.2]));
        assert!(vec_compare(&perm.next().unwrap(), &vec![ std::f64::NAN, std::f64::NAN, 2.2, 1.1]));
        assert_eq!(perm.next(), None);

        let mut perm = PermuteSorted ::new(vec!["a", "b", "c"]);
        assert_eq!(perm.next(), Some(vec!["a", "b", "c"]));
        assert_eq!(perm.next(), Some(vec!["a", "c", "b"]));
        assert_eq!(perm.next(), Some(vec!["b", "a", "c"]));
        assert_eq!(perm.next(), Some(vec!["b", "c", "a"]));
        assert_eq!(perm.next(), Some(vec!["c", "a", "b"]));
        assert_eq!(perm.next(), Some(vec!["c", "b", "a"]));
        assert_eq!(perm.next(), None);
        assert_eq!(perm.next(), None);
    }

    // NaN vec_compare from https://stackoverflow.com/a/40768104
    fn eq_with_nan_eq(a: f64, b: f64) -> bool {
        (a.is_nan() && b.is_nan()) || (a == b)
    }

    fn vec_compare(va: &[f64], vb: &[f64]) -> bool {
        (va.len() == vb.len()) &&  // zip stops at the shortest
         va.iter()
           .zip(vb)
           .all(|(a,b)| eq_with_nan_eq(*a,*b))
    }
}
