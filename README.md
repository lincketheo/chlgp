# chlgp

A Changelog Parser

## Usage:

Parses changelog into json - if no other arguments are supplied, parses the entire changelog
```
$ ./chlgp get CHANGELOG_example.md 
```

Optionally supply the number of changelog entries:
```
$ ./chlgp get CHANGELOG_example.md --head 5
```

Optionally supply any of `body` `version` or `date`. If you don't include any, it includes all:
```
$ ./chlgp get CHANGELOG_example.md body date --head 5 
```

Optionally specify the output format (pst... it only supports json for now :)

```
$ ./chlgp get CHANGELOG_example.md body date json --head 5 
```

## Valid Changelog syntax

```
## [version] <date> 

<body>

## [next version] <date>

<body>
```

This is still a pretty weak parser so there are some rules:
1. No `[` or `]` anywhere other than the version.
2. One space before and after the date (one space total in between version and date `[version] <date> ` is valid `[version]  <date> ` is not, notice the space after date).

These rules will be fixed in later iterations
