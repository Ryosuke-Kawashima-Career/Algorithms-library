use proconio::input;
// 鉄則B59
fn main() {
    input!{n: usize, a: [i64; n]}
    let (invs, _) = sort_inversion(&a);
    println!("{}", invs);
}

fn sort_inversion<T: Clone + Ord>(a: &[T]) -> (usize, Vec<T>) {
	// 長さが 1 以下のときは、ソートする必要がない
	if a.len() <= 1 {
	  return (0, a.to_vec());
	}
	
	// 約半分に分ける（分割統治法）
	let half = a.len() / 2;
	// 半分に分けた配列について、それぞれソートしつつ転倒数を求める
	let (inv1, vec1) = sort_inversion(&a[.. half]);
	let (inv2, vec2) = sort_inversion(&a[half ..]);
	// 配列をマージしつつ、最終的な転倒数を求める
	let mut inv = inv1 + inv2;
	let mut result = Vec::with_capacity(a.len());
	let mut i1 = 0;
	let mut i2 = 0;
	while i1 < vec1.len() && i2 < vec2.len() {
	  if vec1[i1] <= vec2[i2] {
	    // 交換する必要がないため転倒数は増えない
	    result.push(vec1[i1].clone());
	    i1 += 1;
	  } else {
	    result.push(vec2[i2].clone());
	    i2 += 1;
	    // vec2 の要素が vec1 の要素より前にくるとき、交換が起きている
	    // ここで vec1 の残りの要素はすべて vec2[i2] より大きいため、転倒数はその大きさ分増える
	    inv += vec1.len() - i1;
	  }
	}
	// 余ったものを result に追加
	result.append(&mut vec1[i1 ..].to_vec());
	result.append(&mut vec2[i2 ..].to_vec());
	(inv, result)
}