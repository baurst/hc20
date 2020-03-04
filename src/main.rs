use std::env;

#[derive(Debug, Clone, Copy)]
struct Book {
    value: i64,
    index: usize,
}

#[derive(Debug, Clone)]
struct Library {
    books: Vec<Book>,
    index: usize,
    signup_time: usize,
    max_scannable_books_per_day: usize,
}

#[derive(Debug, Clone)]
struct Problem {
    days_left: i64,
    book_scores: Vec<i64>,
    libraries: Vec<Library>,
    dataset_name: String,
}

struct Solution {
    libs_books: Vec<(Library, Vec<Book>)>,
}

fn score(problem: &Problem, solution: &Solution) -> i64 {
    let mut score = 0;
    let mut day_start: usize = 0;

    for (lib, books) in &solution.libs_books {
        day_start += lib.signup_time;
        let mut days_necessary = 0;
        if books.len() % lib.max_scannable_books_per_day == 0 {
            days_necessary = books.len() / lib.max_scannable_books_per_day;
        } else {
            days_necessary = books.len() / lib.max_scannable_books_per_day + 1
        }

        if days_necessary > problem.days_left as usize - day_start {
            panic!("too many books in library {}!", lib.index);
        }

        for book in books {
            score += book.value;
        }
    }

    return score;
}

fn load(ds_name: &str) -> Problem {
    use std::fs;
    use std::path::Path;

    //   let filename = osp.join(osp.dirname(__file__), "..", "in", ds_name + ".txt")
    let filename = env::current_dir()
        .unwrap()
        .join("datasets")
        .join(ds_name.to_owned() + ".txt");
    println!("Loading file: {:?}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.lines();
    let num_books = lines.next().unwrap();
    let first_line: Vec<i64> = num_books
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let num_books = first_line[0];
    let num_libs = first_line[1];
    let days_left = first_line[2];

    let book_scores: Vec<i64> = lines
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut libraries = vec![];
    for lib_idx in 0..num_libs {
        let lib_data: Vec<i64> = lines
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let num_books_in_lib = lib_data[0];
        let signup_time = lib_data[1];
        let max_shippable_books_per_day = lib_data[2];

        let books_in_lib: Vec<i64> = lines
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut books = vec![];

        for book in books_in_lib {
            books.push(Book {
                index: book as usize,
                value: book_scores.clone()[book as usize],
            });
        }

        let lib = Library {
            books: books,
            index: lib_idx as usize,
            signup_time: signup_time as usize,
            max_scannable_books_per_day: max_shippable_books_per_day as usize,
        };
        libraries.push(lib);
    }

    let problem = Problem {
        days_left: days_left,
        book_scores: book_scores,
        libraries: libraries,
        dataset_name: ds_name.to_owned(),
    };

    return problem;
}

fn solve(problem: &Problem) -> Solution {
    return Solution { libs_books: vec![] };
}

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
extern crate chrono;

fn save(solution: &Solution, problem_name: &str, score: i64) {
    let target_path = env::current_dir()
        .unwrap()
        .join("out")
        .join(problem_name.to_owned() + ".txt");

    let file = File::create(target_path.clone()).unwrap();
    let mut file = LineWriter::new(file);

    file.write_all(format!("{}\n", solution.libs_books.len()).as_bytes())
        .unwrap();

    for (lib, books) in &solution.libs_books {
        file.write_all(format!("{} {}", lib.index, books.len()).as_bytes())
            .unwrap();
        file.write_all(format!("{} {}", lib.index, books.len()).as_bytes())
            .unwrap();
        let books_str: String = books
            .into_iter()
            .map(|i| format!("{} ", i.index))
            .collect::<String>();
        file.write_all(books_str.as_bytes()).unwrap();
    }

    file.flush().unwrap();
    println!("Wrote file {:?}", target_path);
}

fn main() {
    let dsets = vec![
        "a_example",
        "b_read_on",
        "c_incunabula",
        "d_tough_choices",
        "e_so_many_books",
        "f_libraries_of_the_world",
    ];

    for argument in env::args().skip(1) {
        println!("{}", argument);
        let problem = load(dsets[argument.parse::<usize>().unwrap()]);

        let solution = solve(&problem);

        let score = score(&problem, &solution);

        save(&solution, &problem.dataset_name, score);
    }
}
