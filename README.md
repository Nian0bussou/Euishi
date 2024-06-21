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

### using :
 - with `cargo run` in the same directory
 - with `cargo build`, then running the binary at `./target/debug/rustmove`
 - using `make` along the provided `Makefile` producing a binary at `./rustmove`


# Usage 

- flags
    - path ; provides the path to be used ; defaults to `/mnt/d/grapper/`
    - move_ ; will sort the files
    - scramble ; will scramble the files
    - removeTmps ; will use remove_tmps
    - verbose ; will print which file it is checking when using removeTmps

if both move_ & scramble are flagged , scramble will run first


##### remove_tmps

!! is a function that will
walk through the directories to remove 
abandonned tmp files 
(most likely to happen if this program is stop midway while sorting,
or if there was already tmp files made by other programs)

***avoid using remove_tmps when a program is using a file in the directories provided,***

***could cause data loss***

##### path
if the path is specified, it will use it as the root dir (where all the subdirs are located)

however **NO VERIFICATION IS DONE** 
some directories are automatically made,
however the program will only move images 
(no extension verification yet,
requires the file to have width &
height which excludes pretty much everything except images)
make sure the path is correct

##### Sort 

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


##### scramble

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


