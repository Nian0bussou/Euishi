# Euishi - えういし
A program to move images based on their dimensions and pixel count

# install
cloning the repo : 
```
git clone https://github.com/Nian0bussou/Euishi.git && cd Euishi
```

### building :
--- 

###### Linux : 
 - using `make` :
    use `make` in the rustmove directory
    `make` calls cargo build then moves `./target/debug/rustmove` to `./rustmove`
 - using `cargo`:
    using `cargo build`, binary will be in `./target/debug/`
    using `cargo build --release`, binary will be in `./target/release/`

###### Other OS : 
I have not tested on OS other than Linux, proceed at your own risk
 - can compile with `cargo build`, binary will be in `./target/debug/` by default

###### Compile to other name;
You can change the name it will compile to in the `Cargo.toml` file

# Usage : 
 - use `./rustmove -h` for help

Commands: 

    move      sort the files 

    scramble  scramble the files 

    remove    remove tmp files

    help      Print this message or the help of the given subcommand(s)


Options (only in move & scramble):

    -p, --path <PATH>                provide the path
    
    -c, --choose-dirs <CHOOSE_DIRS>  specify which dirs to act upon using a conf.json
    
    -h, --help                       Print help

"-p" (--path)
is not necessary to be provided using -c (--choose-dirs) when using `move`

Options of remove:

    -p, --path <PATH>  provide the path
    
    -v, --verbose
    
    -h, --help         Print help

 
a `json` file can be used to specify which dirs to sort using this pattern :

```json
{
  "paths": [
    "/your/path/1"
    "/your/path/2"
    "/your/path/3"
  ]
}
```


### removetmps

is a function that will
walk through the directories to remove 
abandonned tmp files 
(most likely to happen if this program is stop midway while sorting,
or if there was already tmp files made by other programs)

***avoid using removetmps when a program is using a file in the directories provided,***

***could cause data loss***

### path
if the path is specified, it will use it as the root dir (where all the subdirs are located)

however **NO VERIFICATION IS DONE** 
some directories are automatically made,
however the program will only move images 
(no extension verification yet,
requires the file to have width &
height which excludes pretty much everything except images)
make sure the path is correct

### Sort 

The program will sort the subdirectories in the path provided (in this case 'your-path')

IT **WILL NOT** sort the root directory ('your-path' in this example)

images will be classified by their aspect ratio and their quality

video are not sorted and only go to the `video` directory

bad_quality only refers to images under a threshold of less than 1080x1080 pixels *(can change)

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


### scramble

The program will scramble the subdirectories in the path provided (in this case 'your-path')

IT **WILL NOT** sort the root directory ('your-path' in this example)


it will scramble using the pattern below , it will move each file to the root dir (in this case `subdir1`):

your-path/

├── subdir1

├── subdir2

└ ── subdir3

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



