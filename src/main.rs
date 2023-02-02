use itertools::Itertools;

mod bitset;

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

    // Let's assume operations don't have order
    // Then all operations would be of the form  "a+b+c+d+e"
    // Variables can be in any permutation.
    // Choose with repetition 4 operators and permutate them in any order.
    // Some expressions will be equivalent, we'll use sympy to check equivalent expressions.
    // Keep only expressions which are not equivalent to anything else.
    // From those expressions group them by the starting variable.
    // From there continue grouping by the expression and variable.
    // The result is 5 trees with initial variables a, b, c, d or e.
    // Each node in the tree is an expression and a variable.
    // We DFS through the tree and generate code.
    // 
    // Largest possible integer value?
    // 100 * 100 * 9 * 9 * 8 * 8 * 7 * 7
    // = 2,540,160,000
    // signed int limit = 2,147,483,647
    // unsigned int limit = 4,294,967,295
    // We'll have to use unsigned int carefully (32 bit)
    // 
    // We need set of numbers with entries from 100 to 999 (1000 total)
    // can be represented with a 1024 bit number
    // or 8 - 1024 bit numbers
    // Done! :D (the bit set part) I literally made a class on it yesterday lol
    //
    // 
    

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
