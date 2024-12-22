use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let inp: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut a = vec![];
    let mut b = vec![];
    for line in inp {
        a.push(line[0]);
        b.push(line[1]);
    }

    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn part2(input: &str) -> u32 {
    let inp: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut a = vec![];
    let mut b = vec![];
    for line in inp {
        a.push(line[0]);
        b.push(line[1]);
    }

    let mut occur = HashMap::new();
    for &num in &b {
        *occur.entry(num).or_insert(0) += 1;
    }

    a.into_iter()
        .map(|num| num * occur.get(&num).unwrap_or(&0))
        .sum()
}

#[test]
fn examples() {
    let ex1 = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(part1(ex1), 11);
    assert_eq!(part2(ex1), 31);
}
