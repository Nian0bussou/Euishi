# Euishi - えういし
A program to move images based on their dimensions and pixel count

this is a fucking hot mess, proceed at your own risks

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
is not necessary to be provided when using -c (--choose-dirs) 

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
