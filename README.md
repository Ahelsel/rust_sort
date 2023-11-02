# MergeSort / QuickSort (Single-Threaded)

### Running the program

Use ```cargo build --release``` to build the project

Use ```./target/release/my-project [-i filename.txt] [-o filename.txt] [--alg=<merge,quick>]``` 

The input file should contain integers (one per line). 

The sorted numbers will be printed, line by line, into the output file


### Generating Test File to be Sorted

You can generate a input file full of numbers to be sorted by running

```./generate_numbers.sh [amount to be generated] [lower bound] [upper bound]```

The numbers will be outputted into the file ```inp.txt```


### File Organization

## main.rs
This contains the main logic for the program and is responsible for most of the error handling

## sort.rs
This file/module contains the sorting functions (merge_sort, merge(helper), and quick_sort)

## parse.rs
This file/module contains the functions responsible for reading from the input file, writing to the output file, and  