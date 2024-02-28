# Rokgrep
A light and simple grep clone made with Rust.

If you're looking for something a little more useful, check out [ripgrep](https://github.com/BurntSushi/ripgrep).

## Usage
```
rokgrep
  -p, --path <file/directory path>       
  -t, --text <text to look for>       
  -r, --recursive         
  -c, --case-insensitive  
  -h, --help
```
The `path` and `text` arguments are required.
The `recursive` argument will have rokgrep look in subdirectories, if the path you entered is a directory.
The `case-insensitive` argument makes it so that capital or lowercase letters do not affect the search result.
