use proconio::input;
use std::collections::BTreeMap;
// ABC437E
// Q. Find the lexicographically smallest sequence of nodes. You need to add the element of Y to the Xth Array.
// A. DFS on the Trie Tree
/*A. A1,A2,ANを辞書順に並べ替えよという問題で，Trie （トライ木）を使って解けます．
A1,…,ANをキーとした Trie を作ります．ただし，同じ数列は同じ Trie のノードに対応するようにします．親ノードvから値xiの子ノードがすでにあればそれを再利用し，なければ新しいノードuを作って辺v⟶(xi)⟶uを追加するようにします．
子ノードの情報は連想配列（C++ の map，Python の dict）などで管理します．また，各ノードにはどの数列が対応するかも記録しておきます．
この Trie 上で DFS をすれば答えが求まります．あるノードから複数の辺が出る場合には，辺の値が小さい順に潜るようにすればよいです．DFS でノードにたどり着いたら，そのノードに対応する数列の添え字を答えの配列に昇順で追加します．
G[index: ノード番号][key: 数列の要素] = value(ノード番号)
temp: ノード番号の一時保存用変数
pos: 数列iがどのノードに属しているか
vs: ノード番号ごとに属している数列iを格納するリスト
*/
/* My Discussion:
数列の辞書順を求める。
親が強ければ子供も強いという辞書順の性質を利用する問題
もしかしてグラフを用いるのではないだろうか?
 */
#[allow(non_snake_case)]
fn main() {
    input! {n: usize, xy: [(usize, usize); n]}
    // trie[Node ID] = Map<Element of Arrays, Next Node ID>
    let mut trie: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); n + 1];
    // 一つのノードに複数の数列が割り振られる←要素が等しい数列が複数存在する。
    let mut nodeID_to_arrays: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut array_to_nodeID: Vec<usize> = vec![n + 1; n + 1];
    array_to_nodeID[0] = 0;
    let mut node_id: usize = 1;
    for array in 1..=n {
        let (parent_array, element) = xy[array - 1];
        // まだNodeが存在しない場合
        if !trie[array_to_nodeID[parent_array]].contains_key(&element) {
            trie[array_to_nodeID[parent_array]].insert(element, node_id);
            node_id += 1;
        }
        array_to_nodeID[array] =
            trie[array_to_nodeID[parent_array]][&element];
        nodeID_to_arrays[array_to_nodeID[array]].push(array);
    }
    let mut ans = Vec::new();
dfs(0, &trie, &nodeID_to_arrays, &mut ans);
for v in ans.iter() {
    print!("{} ", v);
}
println!();
}

#[allow(non_snake_case)]
fn dfs(
    node_id: usize,
    trie: &Vec<BTreeMap<usize, usize>>,
    nodeID_to_arrays: &Vec<Vec<usize>>,
    ans: &mut Vec<usize>,
) {
    let mut arrays = nodeID_to_arrays[node_id].clone();
    arrays.sort();
    ans.extend(arrays.iter().cloned());
    for (&_element, &next_node_id) in trie[node_id].iter() {
        dfs(next_node_id, trie, nodeID_to_arrays, ans);
    }
}
