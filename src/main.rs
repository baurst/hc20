
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


struct Book {
    value: i64,
    index: usize,
}

struct Library {
    books: Vec<Book>,
    index: usize,
    signup_time: f64,
    max_scannable_books_per_day: usize,
}

struct Problem{
    days_left: i64,
    book_scores: Vec<i64>,
    libraries: Vec<Library>,
}


fn load(ds_name: &str) -> Problem {
    use std::path::Path;
    use std::fs;

 //   let filename = osp.join(osp.dirname(__file__), "..", "in", ds_name + ".txt")
    let filename = env::current_dir().unwrap().join("datasets").join(ds_name.to_owned() + ".txt");
    println!("Loading file: {:?}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let problem = Problem{days_left: 0, book_scores: vec![0;10], libraries: vec![]};
    return problem;
}
fn main() {
    let dsets = vec!["a_example", "b_read_on", "c_incunabula", "d_tough_choices", "e_so_many_books", "f_libraries_of_the_world"];


    for argument in env::args().skip(1) {
        println!("{}", argument);
        load(dsets[argument.parse::<usize>().unwrap()]);
    
    }

    println!("Hello, world!");
}