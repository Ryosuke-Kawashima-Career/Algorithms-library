fn main() {
    input!{n: usize}
    for perm in (0..n).permutation() {
    }
    // nPr
    for comb in (0..n).combinations(r) {
        for perm in (0..r).permutation() {
            for i in 0..r {
                vector[comb[perm[i]]];
            }
        }
    }
}

pub struct PermutationIterator<T: Ord + Clone> {
    li: Vec<T>,
    is_finished: bool,
}

impl <T: Ord + Clone> PermutationIterator<T> {
    pub fn new(mut li: Vec<T>) -> PermutationIterator<T> {
        let is_finished = li.is_empty();
        li.sort();
        PermutationIterator {
            li,
            is_finished,
        }
    }
}

impl <T: Ord + Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;

    // C++ の next_permutation 実装をもとに
    // ref. https://cpprefjp.github.io/reference/algorithm/next_permutation.html
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if self.li.len() == 1 {
            self.is_finished = true;
            return Some(self.li.clone());
        }

        let mut i = self.li.len() - 1;
        let res = self.li.clone();

        loop {
            let ii = i;
            i -= 1;
            if self.li[i] < self.li[ii] {
                let mut j = self.li.len() - 1;
                while self.li[i] >= self.li[j] { j -= 1; }

                self.li.swap(i, j);
                self.li[ii..].reverse();
                return Some(res);
            }
            if i == 0 {
                self.li.reverse();
                self.is_finished = true;
                return Some(res);
            }
        }
    }
}

pub trait Permutation<T: Ord + Clone> {
    fn permutation(self) -> PermutationIterator<T>;
}

// Vec<T> に対してのみの実装する
// impl <T: Ord + Clone> Permutation<T> for Vec<T> {
//     fn permutation(self) -> PermutationIterator<T> {
//         PermutationIterator::new(self)
//     }
// }

// IntoIterator を実装するものに対して Permutation を実装する
impl <T: Ord + Clone, I: IntoIterator<Item=T>> Permutation<T> for I {
    fn permutation(self) -> PermutationIterator<T> {
        PermutationIterator::new(self.into_iter().collect())
    }
}