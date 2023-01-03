#[macro_use]
extern crate derive_new;

use std::fs;

const TEST_PATH: &'static str = "./src/test_input.txt";
const REAL_PATH: &'static str = "./src/real_input.txt";

#[derive(Debug, Clone, new, PartialEq, Eq, Hash)]
struct A {
    a: i64,
    i: usize,
}
#[derive(Debug, Clone, new, PartialEq, Eq, Hash)]
struct B {
    a: i64,
    i: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum C {
    ChildA(A),
    ChildB(B),
}

fn move_n(order: &Vec<C>, value: &C, i: usize) -> Vec<C> {
    match value {
        C::ChildA(_value) => return order.to_vec(),
        C::ChildB(value) => {
            let mut k = order.clone();

            k.rotate_left(i);
            k = k[1..].to_vec();
            if value.a > 0 {
                k.rotate_left(value.a as usize % (order.len() - 1))
            } else {
                k.rotate_right(value.a.abs() as usize % (order.len() - 1))
            }

            k.insert(0, C::ChildA(A::new(value.a, value.i)));
            return k;
        }
    }
}

fn part1(text_input: String) -> i64 {
    let data: Vec<C> = text_input
        .lines()
        .enumerate()
        .map(|x| C::ChildB(B::new(x.1.parse().unwrap(), x.0)))
        .collect();
    let initial_order = data.clone();
    let mut order = data.clone();
    initial_order.iter().for_each(|n| {
        order = move_n(&order, n, order.iter().position(|x| *x == *n).unwrap());
    });
    let k: Vec<i64> = order
        .iter()
        .map(|n| match n {
            C::ChildA(n) => n.a,
            C::ChildB(n) => n.a,
        })
        .collect();

    let z_p = k.iter().position(|x| *x == 0).unwrap();
    let x: i64 = vec![
        k[(z_p + 1000) % k.len()],
        k[(z_p + 2000) % k.len()],
        k[(z_p + 3000) % k.len()],
    ]
    .iter()
    .sum();

    return x;
}
fn part2(text_input: String) -> i64 {
    let data: Vec<i64> = text_input.lines().map(|x| x.parse().unwrap()).collect();
    let encryption_key: i64 = 811589153 as i64;
    let initial_order: Vec<C> = data
        .iter()
        .enumerate()
        .map(|x| C::ChildB(B::new(x.1 * encryption_key, x.0)))
        .collect();
    let mut order = initial_order.clone();
    for _ in 0..10 {
        initial_order.iter().for_each(|n| {
            order = move_n(&order, n, order.iter().position(|x| *x == *n).unwrap());
        });
        order = order
            .iter()
            .map(|n| match n {
                C::ChildA(n) => C::ChildB(B::new(n.a, n.i)),
                C::ChildB(n) => C::ChildB(B::new(n.a, n.i)),
            })
            .collect();
    }
    let k: Vec<i64> = order
        .iter()
        .map(|n| match n {
            C::ChildA(n) => n.a,
            C::ChildB(n) => n.a,
        })
        .collect();
    let z_p = k.iter().position(|x| *x == 0).unwrap();
    let x: Vec<i64> = vec![
        k[(z_p + 1000) % k.len()] as i64,
        k[(z_p + 2000) % k.len()] as i64,
        k[(z_p + 3000) % k.len()] as i64,
    ];

    let s: i64 = x.iter().sum();

    return s;
}

fn main() {
    let test_input = fs::read_to_string(TEST_PATH).expect("Should have been able to read the file");
    let real_input = fs::read_to_string(REAL_PATH).expect("Should have been able to read the file");

    println!("PART 1");
    use std::time::Instant;
    let now = Instant::now();
    println!("Test: {}", part1(test_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part1(real_input.clone()));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);
    let now = Instant::now();

    println!("PART 2");
    println!("Test: {:?}", part2(test_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let now = Instant::now();
    println!("Real: {}", part2(real_input));
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
