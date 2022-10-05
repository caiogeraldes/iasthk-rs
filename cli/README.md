# iastconvert

A rust CLI for Harvard-Kyoto to IAST conversion.

## Usage

The basic usage of the CLI is very straightforward:

```bash
$ iastconvert "asti nRpo nalo nAma|"
> asti nṛpo nalo nāma।
```

The output might be stored into a file by passing the `-o` or `--output` argument:

```bash
$ iastconvert "asti nRpo nalo nAma|" -o output.txt
$ echo output.txt
> asti nṛpo nalo nāma।
```

If needed, the `-f` or `--file` flag might be passed to assume the passed string as an input file name and not as a string to be converted.

```bash
$ echo "asti nRpo nalo nAma|" >> input.txt
$ iastconvert -f input.txt
> asti nṛpo nalo nāma।
```
