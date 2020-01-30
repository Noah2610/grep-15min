# grep in 15 minutes
## description
Returns each line from the file
if any given pattern matches a sub-slice of the line.

## usage
```
grep15min FILE [PATTERN...]

ARGUMENTS
    FILE
        input file.
        every line is returned that matches against any pattern.
    PATTERN
        a string. matches anywhere inside each line.
        multiple patterns can be given.
        each pattern is checked against every line.

OUTPUT
    the output is every line from the input file,
    which matched any of the given patterns.

    each output line is prefixed with its line number.
```
