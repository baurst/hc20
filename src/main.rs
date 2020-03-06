use std::collections::HashSet;
use std::env;

#[derive(Debug, Clone, Copy, Hash)]
struct Book {
    value: i64,
    index: usize,
}

use std::cmp::Ordering;

impl Ord for Book {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for Book {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        return self.index == other.index;
    }
}

impl Eq for Book {}

#[derive(Debug, Clone, Hash)]
struct Library {
    books: Vec<Book>,
    index: usize,
    signup_time: usize,
    max_scannable_books_per_day: usize,
}

impl PartialEq for Library {
    fn eq(&self, other: &Self) -> bool {
        return self.index == other.index;
    }
}

impl Eq for Library {}

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

    let mut books_taken: HashSet<Book> = vec![].iter().cloned().collect();
    let mut libs_taken: HashSet<usize> = vec![].iter().cloned().collect();

    for (lib, books) in &solution.libs_books {
        day_start += lib.signup_time;
        let days_necessary;
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
            if !books_taken.insert(book.clone()) {
                panic!("Lib {} Book {:?} was already taken!", lib.index, book);
            }
        }

        if !libs_taken.insert(lib.index) {
            panic!("Lib {} was already taken!", lib.index);
        }
    }

    return score;
}

fn load(ds_name: &str) -> Problem {
    use std::fs;

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

    assert_eq!(book_scores.len() as i64, num_books);

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

        assert_eq!(books_in_lib.len() as i64, num_books_in_lib);

        let mut books = vec![];

        for book in books_in_lib {
            books.push(Book {
                index: book as usize,
                value: book_scores[book as usize],
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
    let mut books_left = HashSet::new();
    let mut lib_books: Vec<(Library, Vec<Book>)> = vec![];
    let mut libs_left = HashSet::new();

    for lib in &problem.libraries {
        for book in &lib.books {
            books_left.insert(book);
        }
        libs_left.insert(lib);
    }

    let mut days_left = problem.days_left;
    while days_left > 0 && libs_left.len() > 0 {
        let mut max_possible_lib_score = -1;
        let mut best_lib: Option<&Library> = None;
        for lib in &libs_left {
            let books_at_lib: HashSet<_> = lib.books.iter().collect();
            let mut books_left_at_lib: Vec<_> = books_at_lib.intersection(&books_left).collect();
            if books_left_at_lib.len() <= 0 {
                continue;
            }
            books_left_at_lib.sort_unstable();
            let max_num_books_to_scan = days_left * lib.max_scannable_books_per_day as i64;
            let num_books_to_be_scanned = books_at_lib.len().min(max_num_books_to_scan as usize);
            let max_lib_score: i64 = books_left_at_lib
                .iter()
                .take(num_books_to_be_scanned)
                .map(|s| s.value)
                .sum();
            let max_lib_score = max_lib_score / lib.signup_time as i64;
            if max_lib_score > max_possible_lib_score {
                best_lib = Some(lib);
                max_possible_lib_score = max_lib_score;
            }
        }

        if best_lib.is_some() {
            let best_lib = best_lib.unwrap();

            if days_left > best_lib.signup_time as i64 {
                days_left -= best_lib.signup_time as i64;
                let books_at_lib: HashSet<_> = best_lib.books.iter().collect();
                let mut books_left_at_lib: Vec<_> =
                    books_at_lib.intersection(&books_left).collect();
                if books_left_at_lib.len() <= 0 {
                    continue;
                }
                books_left_at_lib.sort_unstable();
                let max_num_books_to_scan = days_left * best_lib.max_scannable_books_per_day as i64;
                let num_books_to_be_scanned =
                    books_at_lib.len().min(max_num_books_to_scan as usize);
                let books_to_scan: Vec<_> = books_left_at_lib
                    .iter()
                    .take(num_books_to_be_scanned)
                    .cloned()
                    .cloned()
                    .collect();
                let books_taken: HashSet<_> = books_to_scan.iter().cloned().collect();
                let diff: HashSet<_> = books_left.difference(&books_taken).cloned().collect();


                let books_to_scan : Vec<Book> = books_to_scan.iter().cloned().cloned().collect();
                lib_books.push((best_lib.clone(), books_to_scan.iter().cloned().collect()));
 
                //println!("Removing {} books from books_left", num_books - new_left);

                books_left = diff;
            }

            libs_left.remove(best_lib);
        //println!(
        //    "Choosing lib {} with score {}",
        //    best_lib.index, max_possible_lib_score
        //);
        } else {
            break;
        }
        println!("Days left: {} - Libs left: {}", days_left, libs_left.len());
    }
    return Solution {
        libs_books: lib_books,
    };
}

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;
extern crate chrono;

fn save(solution: &Solution, problem_name: &str, score: i64) {
    let target_path = env::current_dir().unwrap().join("out").join(
        problem_name.to_owned()
            + &format!("_score_{}_", score)
            + &chrono::Local::now().format("%Y_%m_%d_%H_%M_%S").to_string()
            + ".txt",
    );

    let file = File::create(target_path.clone()).unwrap();
    let mut file = LineWriter::new(file);

    file.write_all(format!("{}\n", solution.libs_books.len()).as_bytes())
        .unwrap();

    for (lib, books) in &solution.libs_books {
        file.write_all(format!("{} {}\n", lib.index, books.len()).as_bytes())
            .unwrap();
        let books_str: String = books
            .into_iter()
            .map(|i| format!("{} ", i.index))
            .collect::<String>();
        file.write_all((books_str + "\n").as_bytes()).unwrap();
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

    use rayon::prelude::*;
    let args: Vec<usize> = env::args()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let scores: Vec<i64> = args
        .par_iter()
        .map(|argument| {
            //println!("{}", argument);
            let problem = load(dsets[*argument]);

            let solution = solve(&problem);

            let score = score(&problem, &solution);
            //cum_score += score;
            save(&solution, &problem.dataset_name, score);
            return score;
        })
        .collect();
    let total_score: i64 = scores.iter().sum();
    println!("Total score: {} ", total_score);
}
