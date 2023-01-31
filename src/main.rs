use itertools::Itertools;

fn count_solvable_targets(game: (i32, i32, i32, i32, i32)) -> i32 {
    // abcde++++
    // abcd+e+++
    // abc+de+++
    // ab+cde+++
    // abcd++e++
    // abcde
    todo!()
}


fn polish_generator() -> Vec<String> {
    let variables: [char; 5] = [
        'a',
        'b',
        'c',
        'd',
        'e',
    ];
    let operands: [char; 4] = [
        '+',
        '-',
        '*',
        '/'
    ];

    fn is_valid_polish(p: &Vec<char>) -> bool {
        let mut stack = 0;
        for c in p {
            if ['+','-','*','/'].iter().contains(c) {
                if stack < 2 {
                    return false;
                } else {
                    stack -= 1;
                }
            } else {
                stack += 1;
            }
        }
        true
    }
    

    let expressions = operands.iter()
        .combinations_with_replacement(4)
        .map(|ops| {
            let mut s: String = String::new();
            s.extend(ops);
            s.extend(variables);
            assert_eq!(s.len(), 9);
            let exprs = s.chars()
                .permutations(9)
                .unique()
                .filter(|p| {
                    is_valid_polish(p)
                })
                .map(|p| {
                    p.into_iter().collect::<String>()
                })
                .collect_vec();
            exprs
        })
        .flatten()
        .collect();

    expressions
}

fn main() {
    for expression in polish_generator() {
        println!("{}", expression);
    }
}
