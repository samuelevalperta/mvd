# mvd
This project is made just for educational pourpose on learning Rust.

### Description
A CLI tool that helps you with the classic and annoying `mv /home/downloads/file .`

It moves the most recently added files from a folder (by default from the downloads one) to your working directory.

### Usage
```
  Usage: mvd [OPTIONS] [N] [NAMES]...

Arguments:
  [N]         Number of files to move [default: 1]
  [NAMES]...  A list of new names for the moved files

Options:
  -c, --copy             Copy files instead of move
  -a                     Include files whose names begin with a dot (‘.’)
  -s, --source <SOURCE>  Specify the source folder [env: DOWNLOAD_DIR=] [default: /home/user/Downloads]
  -h, --help             Print help
  -V, --version          Print version
  
```
