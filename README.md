# Euishi - えういし
A program to move images based on their dimensions and pixel count

this is a fucking hot mess, func names makes no sense whatsoever, proceed at your own risks

this is intended mainly for personal use, if it doesn't work as you expect it to, i don't fucking care.

# Makefile
provides 'shortcuts' to commands i frequently do

# install
cloning the repo : 
```
git clone https://github.com/Nian0bussou/Euishi.git && cd Euishi
```

### building :
--- 
```
cargo build --release
```
binaray will be at `./target/release/euishi`

###### Compile to other name;
You can change the name it will compile to in the `Cargo.toml` file

# Usage : 
 - use `./euishi -h` for help

Commands: 

    move      sort the files 

    scramble  scramble the files 

    remove    remove tmp files

    help      Print this message or the help of the given subcommand(s)


Options (only in move & scramble):

    -p, --path <PATH>                provide the path
    
    -c, --choose-dirs <CHOOSE_DIRS>  specify which dirs to act upon using a json file
    
    -h, --help                       Print help

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
"-p" (--path)
is not necessary to be provided when using -c (--choose-dirs) 

Options of remove:

    -p, --path <PATH>  provide the path
    
    -v, --verbose
    
    -h, --help         Print help

 

