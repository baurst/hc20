/*
def load(ds_name):
    filename = osp.join(osp.dirname(__file__), "..", "in", ds_name + ".txt")
    values = readvalues(filename)
    data = {}
    # Todo unpack values into structured dict data

    data["B"] = values[0][0]
    data["L"] = values[0][1]
    data["D"] = values[0][2]

    data["S"] = np.asarray(values[1])

    data["ids"] = []

    data["N"] = []
    data["T"] = []
    data["M"] = []

    for l in range(data["L"]):
        n, t, m = values[2 * l + 2]
        data["ids"].append(values[2 * l + 2 + 1])
        data["N"].append(n)
        data["T"].append(t)
        data["M"].append(m)

    data["libs"] = [
        {
            "index": i,
            "n": data["N"][i],
            "t": data["T"][i],
            "m": data["M"][i],
            "ids": set(data["ids"][i]),
        }
        for i in range(data["L"])
    ]

    del data["ids"]
    del data["N"]
    del data["T"]
    del data["M"]

    return data

*/

use std::env;

#[derive(Debug)]
struct Book {
    value: i64,
    index: usize,
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    index: usize,
    signup_time: usize,
    max_scannable_books_per_day: usize,
}

#[derive(Debug)]
struct Problem {
    days_left: i64,
    book_scores: Vec<i64>,
    libraries: Vec<Library>,
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

    let mut  libraries = vec![]; 
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
    };

    return problem;
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
        println!("{:?}", problem);
    }

}