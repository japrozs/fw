# Fw

`Fw` is a filewatcher for all programming languages and has a very easy and simple to use interface.

## Usage

To get started with Fw, you can use it as follows

```bash
fw "CMD" [FILES...]
```

`CMD` - Command is the command that you want to run when any of the file(s) change. **It has to be in quotes** otherwise it will not work.

`FILES` - Files are the relative path to hte files that you want to watch.

An example command would go as follows :

```bash
fw "echo file was changed" foo.txt bar.txt baz.txt
```

To get all the commands you use the `-h` flafg as follows:

```bash
fw -h # Alternatively, you can also use --help
```

## Install

You can install `fw` via Homebrew

```bash
brew install fw
```

## License : `MIT`
