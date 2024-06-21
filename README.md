```
 /$$$$$$$                        /$$
| $$__  $$                      | $$
| $$  \ $$ /$$   /$$  /$$$$$$$ /$$$$$$
| $$$$$$$/| $$  | $$ /$$_____/|_  $$_/
| $$__  $$| $$  | $$|  $$$$$$   | $$
| $$  \ $$| $$  | $$ \____  $$  | $$ /$$
| $$  | $$|  $$$$$$/ /$$$$$$$/  |  $$$$/
|__/  |__/ \______/ |_______/    \___/

 /$$      /$$
| $$$    /$$$
| $$$$  /$$$$  /$$$$$$  /$$    /$$  /$$$$$$
| $$ $$/$$ $$ /$$__  $$|  $$  /$$/ /$$__  $$
| $$  $$$| $$| $$  \ $$ \  $$/$$/ | $$$$$$$$
| $$\  $ | $$| $$  | $$  \  $$$/  | $$_____/
| $$ \/  | $$|  $$$$$$/   \  $/   |  $$$$$$$
|__/     |__/ \______/     \_/     \_______/
```

# RustMove

Program to move images based on their dimensions and pixel count

# install

clone the repo : 
```
git clone https://github.com/Nian0bussou/rustmove.git
```

can be run using `cargo run`

binary file can be found at `./target/debug/rustmove` after compiling with cargo,
the file itself has no dependency, so it can be moved wherever you like or symlink'd.

# Usage 
`rustmove <choice1> <choice2> <path>`
(or `cargo run <choice1> <choice2> <path>`, (will not work on Windows however))

### defaults (no args provided)
uses the defaults
- wont call remove_tmps
- sort the files
- default path `"/mnt/d/grapper/"` on *nix based system or `"D:/grapper/` on windows

## Choices
each choice will be processed by the order, 
so choice2 requires choice1 to be made before,
so path requires choice1 & choice2 to be made before.

### choice1
- 0 => sort the files
- 1 => scramble the files into their respective root subdir

### choice2
- 1 => call remove_tmps
- anything else => doesnt call remove_tmps

remove_tmps is a function that will
walk through the directories to remove 
abandonned tmp files 
(most likely to happen if the program is stop midway while sorting,
or if there was already tmp files)

***avoid using remove_tmps when a program is using a file in the directories provided,***

***could cause data loss***

remove_tmps can output each file it is passing 


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


