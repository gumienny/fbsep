// If several elements are equally maximum, the last element index is returned.
// If the `iter` is empty, None is returned.
pub fn argmax<T, Iter>(iter: Iter) -> Option<usize>
where
    T: Copy + Ord,
    Iter: Iterator<Item = T>,
{
    let mut argmax: Option<(usize, T)> = None;

    for (i, h) in iter.enumerate() {
        match argmax {
            None => {
                argmax = Some((i, h));
            }
            Some((_, y)) => {
                if y < h {
                    argmax = Some((i, h));
                }
            }
        }
    }

    argmax.map(|(i, _)| i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argmax_test() {
        let nums = &[0u8; 0];

        assert_eq!(None, argmax(nums.iter()));

        assert_eq!(Some(2), argmax([-3i64, -6, -1, -4].iter()));
        assert_eq!(Some(3), argmax([1, 2, 3, 7, 4].iter()));
    }
}
