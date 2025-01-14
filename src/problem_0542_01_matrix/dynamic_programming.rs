pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

trait MakeIter<'a> {
    type Iter: Iterator<Item = &'a mut i32>;

    fn make_iter(&self, slice: &'a mut [i32]) -> Self::Iter;
}

impl<'a, F, R> MakeIter<'a> for F
where
    F: Fn(&'a mut [i32]) -> R,
    R: Iterator<Item = &'a mut i32>,
{
    type Iter = R;

    fn make_iter(&self, slice: &'a mut [i32]) -> Self::Iter {
        self(slice)
    }
}

impl Solution {
    #[allow(single_use_lifetimes)] // False positive.
    fn update(mut rows_iter: impl Iterator<Item = impl AsMut<[i32]>>, make_iter: impl for<'b> MakeIter<'b>) {
        let mut prev_row = rows_iter.next().unwrap();
        let mut first_row_iter = make_iter.make_iter(prev_row.as_mut());
        let mut left = *first_row_iter.next().unwrap();

        for value in first_row_iter {
            *value = (*value).min(left + 1);
            left = *value;
        }

        for mut row in rows_iter {
            let mut iter = make_iter
                .make_iter(prev_row.as_mut())
                .zip(make_iter.make_iter(row.as_mut()));

            let (&mut top, left) = iter.next().unwrap();

            *left = (*left).min(top + 1);

            let mut left = *left;

            for (&mut top, current) in iter {
                *current = (*current).min(top.min(left) + 1);
                left = *current;
            }

            prev_row = row;
        }
    }

    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn iter_forward(slice: &mut [i32]) -> impl Iterator<Item = &mut i32> {
            slice.iter_mut()
        }

        fn iter_backward(slice: &mut [i32]) -> impl Iterator<Item = &mut i32> {
            slice.iter_mut().rev()
        }

        let mut mat = mat;

        for row in &mut mat {
            for cell in row {
                if *cell != 0 {
                    *cell = i32::MAX - 1;
                }
            }
        }

        Self::update(mat.iter_mut().map(Vec::as_mut_slice), iter_forward);
        Self::update(mat.iter_mut().map(Vec::as_mut_slice).rev(), iter_backward);

        mat
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::update_matrix(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
