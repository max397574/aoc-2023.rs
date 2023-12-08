use num_traits::PrimInt;

pub fn grid<T>(text_block: &str, callback: impl Fn(char) -> T) -> Vec<Vec<T>> {
    text_block
        .lines()
        .map(|line| line.chars().map(&callback).collect())
        .collect()
}

pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    let mut a = a;
    let mut b = b;
    let mut c;
    while b != T::from(0).unwrap() {
        c = a;
        a = b;
        b = c % b;
    }
    a
}

pub fn single_lcm<T: PrimInt>(a: T, b: T) -> T {
    (a * b) / gcd(a, b)
}

pub fn get_lcm<T: PrimInt>(numbers: Vec<T>) -> T {
    let mut tmp_lcm = single_lcm(*numbers.first().unwrap(), *numbers.get(1).unwrap());
    for num in numbers.iter().skip(2) {
        tmp_lcm = single_lcm(tmp_lcm, *num);
    }
    tmp_lcm
}
