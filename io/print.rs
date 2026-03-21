fn print_output<T: std::fmt::Display>(vector: &Vec<T>) {
    let n: usize = vector.len();
    for index in 0..n {
        if index == 0 {
            print!("{}", vector[index]);
        } else {
            print!(" {}", vector[index]);
        }
    }
    println!("");
}
fn print_2d<T: std::fmt::Display>(graph: &Vec<Vec<T>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for i in 0..h {
        for j in 0..w {
            print!("{} ", graph[i][j]);
        }
        println!("");
    }
}

fn main() {
    // format output: 000001(桁数を指定する)
    println!("{:0>6}", num);
    // format float value(小数点以下3ケタ)
    println!("{:.3}", float_num);
    // print { or }
    print!("{} : {{", i);
    // print vec
    let n: usize = vector.len();
    for i in 0..n {
        if i == 0 {
            print!("{}", vector[i]);
        } else {
            print!(" {}", vector[i]);
        }
    }
    println!("");
    // print graph: v: {next0, next1}..
    for i in 0..n {
        print!("{}: {{", i+1);
        for j in 0..graph[i].len() {
            if j == 0 {
                print!("{}", graph[i][j]);
            } else {
                print!(", {}", graph[i][j]);
            }
        }
        println!("}}");
    }
    // print with whitespace
    println!("{}", answers.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" "));
}