# RustMove

Program to move images based on their dimensions and pixel count

# install
can be run using `cargo run`

binary file can be found at `./target/debug/rustmove` after compiling with cargo

feel free to move this file wherever you want

# Usage 
`rustmove <choice1> <choice2> <path>`

### defaults (no args provided)
uses the defaults
- wont call remove_tmps
- sort the files
- default path `"/mnt/d/grapper/"`

### choice1
- 1 => call remove_tmps
- anything else => doesnt call remove_tmps

remove_tmps is a function that will walk through the directories to remove abandonned tmp files

### choice2 
- 0 => sort the files
- 1 => scramble the files into their respective root subdir


### path
choice is required to be specified since it have to be in order

if the path is specified, it will use it as the root dir (where all the subdirs are located)



however **NO VERIFICATION IS DONE** 
(and by that I mean that it will just exit if it cant find any available directories),
make sure the path is correct

### Sort 

The program will sort the subdirectories in the path provided (in this case 'your-path')

your-path/

├── subdir1

├── subdir2

└── subdir3

it will sort using this pattern 

subdir1/

├── bad_quality

│   ├── l

│   ├── p

│   └── s

├── other

├── square

├── video

└── wall

subdir2/
...
etc.


