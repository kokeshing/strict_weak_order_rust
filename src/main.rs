// 狭義の弱順序を満たさないような型を定義してsortしてみる
//狭義の弱順序は 非反射律 かつ 推移律 かつ 反対称律 かつ 比較不能性の推移律(同値性の推移律) であること
// 反対称律をやぶる

enum P {
    A,
    B,
    C,
}

use std::cmp::Ordering;
impl PartialOrd for P {
    fn partial_cmp(&self, other: &P) -> Option<Ordering> {
        match self {
            P::A => match other {
                P::A => Some(Ordering::Equal),
                P::B => Some(Ordering::Greater),
                P::C => Some(Ordering::Greater),
            },
            P::B => match other {
                P::A => Some(Ordering::Greater),
                P::B => Some(Ordering::Equal),
                P::C => Some(Ordering::Greater),
            },
            P::C => match other {
                P::A => Some(Ordering::Less),
                P::B => Some(Ordering::Greater),
                P::C => Some(Ordering::Equal),
            }
        }
    }
}

impl Ord for P {
    fn cmp(&self, other: &P) -> Ordering {
        match self {
            P::A => match other {
                P::A => Ordering::Equal,
                P::B => Ordering::Greater,
                P::C => Ordering::Greater,
            },
            P::B => match other {
                P::A => Ordering::Greater,
                P::B => Ordering::Equal,
                P::C => Ordering::Greater,
            },
            P::C => match other {
                P::A => Ordering::Less,
                P::B => Ordering::Greater,
                P::C => Ordering::Equal,
            }
        }
    }
}

impl PartialEq for P {
    fn eq(&self, other: &P) -> bool {
        match self {
            P::A => match other {
                P::A => true,
                P::B => false,
                P::C => false,
            },
            P::B => match other {
                P::A => false,
                P::B => true,
                P::C => false,
            },
            P::C => match other {
                P::A => false,
                P::B => false,
                P::C => true,
            }
        }
    }
}

impl Eq for P {}

extern crate is_sorted;
use is_sorted::IsSorted;
fn main() {
    println!("非反射律");
    println!("A < A => {:?}", P::A < P::A);
    println!("B < B => {:?}", P::B < P::B);
    println!("C < C => {:?}", P::C < P::C);
    println!("");
    println!("");

    println!("推移律");
    println!("A < B => {:?}", P::A < P::B);
    println!("B < A => {:?}", P::B < P::A);
    println!("");
    println!("B < C => {:?}", P::B < P::C);
    println!("C < B => {:?}", P::C < P::B);
    println!("");
    println!("C < A => {:?}", P::C < P::A);
    println!("A < C => {:?}", P::A < P::C);
    println!("");
    println!("");

    println!("反対称律");
    println!("A < B => {:?}", P::A < P::B);
    println!("B < A => {:?}", P::B < P::A);
    println!("A = B => {:?}", P::A == P::B);
    println!("");
    println!("B < C => {:?}", P::B < P::C);
    println!("C < B => {:?}", P::C < P::B);
    println!("B = C => {:?}", P::B == P::C);
    println!("");
    println!("C < A => {:?}", P::C < P::A);
    println!("A < C => {:?}", P::A < P::C);
    println!("C = A => {:?}", P::C == P::A);
    println!("");
    println!("");

    println!("比較不能性の推移律");
    println!("B < A => {:?}", P::B < P::A);
    println!("A < B => {:?}", P::A < P::B);
    println!("");
    println!("C < B => {:?}", P::C < P::B);
    println!("B < C => {:?}", P::B < P::C);
    println!("");
    println!("C < A => {:?}", P::C < P::A);
    println!("A < C => {:?}", P::A < P::C);
    println!("");

    let mut vector = vec![P::A, P::B, P::C, P::B, P::C, P::A, P::B, P::A];

    vector.sort();

    print!("[");
    for x in &vector {
        match &x {
            P::A => print!("{} ", "A"),
            P::B => print!("{} ", "B"),
            P::C => print!("{} ", "C"),
        }
    }
    println!("\u{8}]");
    println!("soted : {:?}", vector.iter().is_sorted());
}
