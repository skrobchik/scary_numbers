use std::fmt::Display;
use itertools::Itertools;

struct BitSet {
    banks: [u128; 8] // 128 bits
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet { banks: [0_u128; 8] }
    }

    pub fn contains(&self, value: u32) -> bool {
        // If value is not in [100, 999] ignore it
        if !( 100 <= value && value <= 999 ) {
            return false;
        }
        
        let value = value - 100;
        let i = value / 128;
        let value = value - (128 * i);
        let set_bank = self.banks[i as usize];

        set_bank & (1 << value) != 0
    }

    pub fn insert(&mut self, value: u32) {
        // If value is not in [100, 999] ignore it
        if !( 100 <= value && value <= 999 ) {
            return;
        }

        let value = value - 100;
        let i = value / 128;
        let value = value - (128 * i);
        let set_bank = &mut self.banks[i as usize];

        *set_bank = *set_bank | (1 << value); 
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            (100_u32..=999_u32).into_iter()
            .filter(|x| { self.contains(*x) })
            .map(|x| format!("{}", x))
            .intersperse(", ".to_string())
            .collect::<String>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::BitSet;

    #[test]
    fn basic_functionality() {
        let mut bitset = BitSet::new();
        assert_eq!(false, bitset.contains(240));
        bitset.insert(240);
        println!("{}", bitset);
        assert_eq!(true, bitset.contains(240));
    }

    #[test]
    fn limit_test() {
        let mut bitset = BitSet::new();
        for x in 100..=999 {
            assert_eq!(false, bitset.contains(x));
        }
        for x in 100..=500 {
            bitset.insert(x);
        }
        for x in 100..=500 {
            assert_eq!(true, bitset.contains(x));
        }
        for x in 501..=999 {
            assert_eq!(false, bitset.contains(x));
        }
    }

    #[test]
    fn ignore_out_of_bounds() {
        let mut bitset = BitSet::new();
        let ignored_nums = [0, 42, 98, 99, 1000, 1001, 1023, 1024, 1025, 1026, 1123, 1124, 1125, 1126, 9999];
        for x in ignored_nums {
            assert_eq!(false, bitset.contains(x));
        }
        for x in ignored_nums {
            bitset.insert(x);
        }
        for x in ignored_nums {
            assert_eq!(false, bitset.contains(x));
        }
    }
}