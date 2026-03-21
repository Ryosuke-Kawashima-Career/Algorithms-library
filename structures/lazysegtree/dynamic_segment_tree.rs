use proconio::input;
// ABC435E
// Q. lr continues to come, paint [l, r) to black. After each query, output how many white cells remain.
/* Answer
1. Nの制約が 1≤N≤2×10^5 とかの場合は遅延セグメント木の典型問題です。（黒マスを0、白マスを1で表した数列を管理。区間更新と区間和取得ができるようにする）
2. セグメント木のサイズが大きすぎる場合、必要なところだけ作るセグ木、またの名をDynamic Segment Tree (動的セグメント木)を用いて解きます。
 */
#[derive(Clone)]
struct Node<T, E> {
    sum: T,
    lazy: E,
    left: Option<Box<Node<T, E>>>,
    right: Option<Box<Node<T, E>>>,
}

impl<T: Clone, E: Clone> Node<T, E> {
    fn new(sum: T, lazy: E) -> Self {
        Node {
            sum,
            lazy,
            left: None,
            right: None,
        }
    }
}

// Separate configuration to allow splitting borrows
// This struct holds all the immutable 'behavior' logic (closures)
// Added PhantomData to hold T and E type parameters
struct SegTreeConfig<T, E, F, G, H, TI, EI, INIT> {
    n: usize,
    op: F,          // fn(T, T) -> T
    mapping: G,     // fn(T, E) -> T
    composition: H, // fn(E, E) -> E
    identity_t: TI, // fn() -> T
    identity_e: EI, // fn() -> E
    init: INIT,     // fn(l, r) -> T
    _marker: std::marker::PhantomData<(T, E)>,
}

impl<T, E, F, G, H, TI, EI, INIT> SegTreeConfig<T, E, F, G, H, TI, EI, INIT>
where
    T: Clone + Copy,
    E: Clone + Copy + PartialEq,
    F: Fn(T, T) -> T,
    G: Fn(T, E) -> T,
    H: Fn(E, E) -> E,
    TI: Fn() -> T,
    EI: Fn() -> E,
    INIT: Fn(usize, usize) -> T,
{
    fn new_node(&self, l: usize, r: usize) -> Box<Node<T, E>> {
        Box::new(Node::new((self.init)(l, r), (self.identity_e)()))
    }

    fn push(&self, node: &mut Box<Node<T, E>>, l: usize, r: usize) {
        if node.lazy == (self.identity_e)() {
            return;
        }

        let mid = (l + r) / 2;

        // Ensure left child exists
        if node.left.is_none() {
            node.left = Some(self.new_node(l, mid));
        }
        let left = node.left.as_mut().unwrap();
        left.sum = (self.mapping)(left.sum, node.lazy);
        left.lazy = (self.composition)(left.lazy, node.lazy);

        // Ensure right child exists
        if node.right.is_none() {
            node.right = Some(self.new_node(mid, r));
        }
        let right = node.right.as_mut().unwrap();
        right.sum = (self.mapping)(right.sum, node.lazy);
        right.lazy = (self.composition)(right.lazy, node.lazy);

        node.lazy = (self.identity_e)();
    }

    fn update_with_range(&self, node: &mut Box<Node<T, E>>, l: usize, r: usize) {
        let mid = (l + r) / 2;
        let left_val = match &node.left {
            Some(child) => child.sum,
            None => (self.init)(l, mid),
        };
        let right_val = match &node.right {
            Some(child) => child.sum,
            None => (self.init)(mid, r),
        };
        node.sum = (self.op)(left_val, right_val);
    }

    fn apply_recursive(
        &self,
        node: &mut Box<Node<T, E>>,
        nl: usize,
        nr: usize,
        l: usize,
        r: usize,
        e: E,
    ) {
        if l <= nl && nr <= r {
            node.sum = (self.mapping)(node.sum, e);
            node.lazy = (self.composition)(node.lazy, e);
            return;
        }

        self.push(node, nl, nr);
        let mid = (nl + nr) / 2;

        if l < mid {
            if node.left.is_none() {
                node.left = Some(self.new_node(nl, mid));
            }
            self.apply_recursive(node.left.as_mut().unwrap(), nl, mid, l, r, e);
        }
        if mid < r {
            if node.right.is_none() {
                node.right = Some(self.new_node(mid, nr));
            }
            self.apply_recursive(node.right.as_mut().unwrap(), mid, nr, l, r, e);
        }

        self.update_with_range(node, nl, nr);
    }

    fn fold_recursive(
        &self,
        node: &mut Box<Node<T, E>>,
        nl: usize,
        nr: usize,
        l: usize,
        r: usize,
    ) -> T {
        if l <= nl && nr <= r {
            return node.sum;
        }

        self.push(node, nl, nr);
        let mid = (nl + nr) / 2;
        let mut res = (self.identity_t)();

        if l < mid {
            let val = match &mut node.left {
                Some(child) => self.fold_recursive(child, nl, mid, l, r),
                None => {
                    let ql = std::cmp::max(nl, l);
                    let qr = std::cmp::min(mid, r);
                    if ql < qr {
                        (self.init)(ql, qr)
                    } else {
                        (self.identity_t)()
                    }
                }
            };
            res = (self.op)(res, val);
        }

        if mid < r {
            let val = match &mut node.right {
                Some(child) => self.fold_recursive(child, mid, nr, l, r),
                None => {
                    let ql = std::cmp::max(mid, l);
                    let qr = std::cmp::min(nr, r);
                    if ql < qr {
                        (self.init)(ql, qr)
                    } else {
                        (self.identity_t)()
                    }
                }
            };
            res = (self.op)(res, val);
        }
        res
    }
}

pub struct DynamicLazySegmentTree<T, E, F, G, H, TI, EI, INIT> {
    config: SegTreeConfig<T, E, F, G, H, TI, EI, INIT>,
    root: Option<Box<Node<T, E>>>,
}

impl<T, E, F, G, H, TI, EI, INIT> DynamicLazySegmentTree<T, E, F, G, H, TI, EI, INIT>
where
    T: Clone + Copy,
    E: Clone + Copy + PartialEq,
    F: Fn(T, T) -> T,
    G: Fn(T, E) -> T,
    H: Fn(E, E) -> E,
    TI: Fn() -> T,
    EI: Fn() -> E,
    INIT: Fn(usize, usize) -> T,
{
    pub fn new(
        n: usize,
        op: F,
        mapping: G,
        composition: H,
        identity_t: TI,
        identity_e: EI,
        init: INIT,
    ) -> Self {
        DynamicLazySegmentTree {
            config: SegTreeConfig {
                n,
                op,
                mapping,
                composition,
                identity_t,
                identity_e,
                init,
                _marker: std::marker::PhantomData,
            },
            root: None,
        }
    }

    pub fn apply(&mut self, l: usize, r: usize, e: E) {
        if l >= r {
            return;
        }
        // Create root if missing
        if self.root.is_none() {
            self.root = Some(self.config.new_node(0, self.config.n));
        }

        let root = self.root.as_mut().unwrap();
        // Uses `self.config` (immutable) and `root` (mutable)
        self.config.apply_recursive(root, 0, self.config.n, l, r, e);
    }

    pub fn fold(&mut self, l: usize, r: usize) -> T {
        if l >= r {
            return (self.config.identity_t)();
        }
        // Create root if missing
        if self.root.is_none() {
            self.root = Some(self.config.new_node(0, self.config.n));
        }

        let root = self.root.as_mut().unwrap();
        // Uses `self.config` (immutable) and `root` (mutable)
        self.config.fold_recursive(root, 0, self.config.n, l, r)
    }

    pub fn fold_all(&mut self) -> T {
        let n = self.config.n;
        self.fold(0, n)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize); q],
    }

    type T = (i64, i64);
    type E = i64;

    // Config: op, mapping, composition, id_t, id_e, init
    let mut seg = DynamicLazySegmentTree::new(
        n,
        |a: T, b: T| (a.0 + b.0, a.1 + b.1),
        |a: T, f: E| if f == -1 { a } else { (f * a.1, a.1) },
        |f: E, g: E| if g == -1 { f } else { g },
        || (0, 0),
        || -1,
        |l: usize, r: usize| {
            let len = (r - l) as i64;
            (len, len)
        },
    );

    for (l, r) in queries {
        let l = l - 1;
        seg.apply(l, r, 0);
        let ans = seg.fold_all();
        println!("{}", ans.0);
    }
}
