# Google Hash Code 2020


<a title="AmrAtrash / CC BY-SA (https://creativecommons.org/licenses/by-sa/4.0)"><img width="512" src="https://upload.wikimedia.org/wikipedia/commons/7/7e/Event_google-hash-code_491696.jpg"></a>

This is a [Google Hash Code 2020](https://codingcompetitions.withgoogle.com/hashcode) solution (Rank 824 out of 10724): [Problem statement](https://storage.googleapis.com/coding-competitions.appspot.com/HC/2020/hashcode_2020_online_qualification_round.pdf).

## Build & Run

```bash
git clone https://github.com/baurst/hc20
cd hc20
cargo run --release 0 1 2 3 4 5
```

Each digit represents a specific problem (check folder datasets):

0. a_example.txt
1. b_read_on.txt
2. c_incuabula.txt
3. d_tough_choices.txt
4. e_so_many_books.txt
5. f_libraries_of_the_world.txt

If you provide multiple digits, the problems will be solved in parallel.
The tool will create a directory called out and save the solutions with their respective score.

## Background
Our [original solution](https://github.com/ltriess/hc20) was done in Python.
Python is great when you need to get things done quickly and don't want to be bothered about types. Plus, as long as you can express your problem in terms of matrices and matrix-operations (numpy), it's very fast. 
I reimplemented our solution to get a feeling about whether I could have done the competition in Rust, which offers a nicer development experience in my opinion. 
Rust has a steeper learning curve, but it is fast out of the box:
Instead of having to focus on a solution that can be expressed using numpy matrix operations I can come up with any idea, implement it in the most intuitive way and the compiler will take care of the speed, which is liberating.


## Takeaways for doing Coding Competitions with Rust
* with good preparation (stubs for data loading and writing...) coding competitions can absolutely be done in Rust
* unsurprisingly, coding in Python is still faster
* with Rust there is much less time spent debugging: at least for me, if the code compiles it's correct most of the time
* don't worry about optimizations, the compiler will take care of speed: focus on your actual solution


## Team Members for the [original solution](https://github.com/ltriess/hc20)

* [Larissa Triess](https://github.com/ltriess)
* [Christoph Rist](https://github.com/risteon)
* [David Schmidt](https://github.com/DavidS3141)
* [Stefan Baur](https://github.com/baurst)

