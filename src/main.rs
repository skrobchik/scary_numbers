use itertools::Itertools;
use pyo3::{prelude::*, types::IntoPyDict};
use indicatif::ProgressIterator;

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

fn expression_generator() {
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

    let variables: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];
    let operands: [char; 4] = ['+', '-', '*', '/'];

    println!("Generating all possible expressions...");
    let expressions = operands
        .iter()
        .combinations_with_replacement(variables.len() - 1)
        .map(|c| {
            c.iter()
                .permutations(c.len())
                .map(|ops| ops.iter().map(|c| ***c).collect_vec())
                .map(|ops| {
                    let expressions = variables
                        .iter()
                        .permutations(variables.len())
                        .map(|vars| vars.iter().map(|c| **c).collect_vec())
                        .map(|vars| {
                            format!(
                                "((((({}{}{}){}{}){}{}){}{}){}{})",
                                vars[0],
                                ops[0],
                                vars[1],
                                ops[1],
                                vars[2],
                                ops[2],
                                vars[3],
                                ops[3],
                                vars[4],
                                ops[4],
                                vars[5]
                            )
                        })
                        .collect_vec();
                    expressions
                })
                .flatten()
                .collect_vec()
        })
        .flatten()
        .collect_vec();
    println!("Done!");

    let mut simplified_expressions: Vec<(String, String)> = Vec::with_capacity(expressions.len());

    println!("Simplifying expressions...");
    Python::with_gil(|py| -> PyResult<()> {
        /*let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);*/

        let locals = [("sympy", py.import("sympy")?)].into_py_dict(py);
        let code = "a,b,c,d,e,f = sympy.symbols('a b c d e f')";
        py.run(&code, None, Some(&locals))?;
        for expression in expressions.into_iter().progress() {
            let code = format!("str(sympy.simplify({}))", expression);
            let simplified: String = py.eval(&code, None, Some(&locals))?.extract()?;
            simplified_expressions.push((expression, simplified));
        }
        Ok(())
    })
    .expect("Something went wrong with Python");
    println!("Done!");

    println!("Filtering equivalent expressions...");
    let expressions: Vec<String> = simplified_expressions
        .into_iter()
        .unique_by(|p| p.1.clone())
        .map(|p| p.0)
        .collect_vec();
    println!("Done!");

    println!("Expressions generated, writing to file...");
    std::fs::write("expressions.txt", expressions.join("\n")).expect("Failed to write to file!");
    println!("Done!");
}

fn main() {
    expression_generator();
}
