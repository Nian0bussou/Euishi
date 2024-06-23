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

A program to move images based on their dimensions and pixel count

# install

cloning the repo : 
```
git clone https://github.com/Nian0bussou/rustmove.git && cd rustmove
```

### using :
 - using `make` along the provided `Makefile` 
 make a binary at `./target/debug/rustmove` then copies it to `./rustmove`

simply run the binary `./rustmove` afterward

- using `cmake`

# Usage : 
 - use `./rustmove -h` for help


##### removetmps

is a function that will
walk through the directories to remove 
abandonned tmp files 
(most likely to happen if this program is stop midway while sorting,
or if there was already tmp files made by other programs)

***avoid using removetmps when a program is using a file in the directories provided,***

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

#### Sort 

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


# scramble

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


