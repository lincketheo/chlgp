# chlgp

A Changelog Parser

## Usage:

Parses changelog into json - if no other arguments are supplied, parses the entire changelog
```
$ ./chlgp get <filename> 
```

Optionally supply the number of changelog entries:
```
$ ./chlgp get <filename> --head 5
```

Optionally supply which of body version date. If you don't include any, it includes all:
```
$ ./chlgp get <filename> body date --head 5 
```

Optionally specify the output format (pst... it only supports json for now :)

```
$ ./chlgp get <filename> body date json --head 5 
```

## Valid Changelog syntax

```
## [version] <date> 

<body>

## [next version] <date>

<body>
```

Note, this is still a weak parser so there are some rules:
1. No `[` or `]` anywhere other than the version.
2. One space before and after the date

These rules will be fixed.
