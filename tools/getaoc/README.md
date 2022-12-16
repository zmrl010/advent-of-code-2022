<div align="center">
  <h1>getaoc</h1>
</div>

<div align="center">
  Command-line utility to fetch and save puzzle input for
  <a href="https://adventofcode.com">Advent of Code</a>
</div>

---

```shell
Usage: getaoc[.exe] [OPTIONS] --session <SESSION> [OUTDIR]

Arguments:
  [OUTDIR]
          Directory to save output files

          [default: ./]

Options:
  -y, --year <YEAR>
          Target puzzle year

          [default: $THIS_YEAR]

  -d, --day <DAY>
          Target puzzle day of month

          [default: $TODAY]

  -i, --input <INPUT_FILENAME>
          Name of file to save puzzle input

          [default: input]

  -s, --session <SESSION>
          Session id used for authentication.

          This can be collected by logging into https://adventofcode.com/
          and inspecting the session cookie.

          [env: SESSION]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
