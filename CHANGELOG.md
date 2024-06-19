# Changelog

## [0.1.0] 2024-06-16 <lincketheo@gmail.com>

### Added

- Parses a very strict Changelog syntax with not very clean code
- Prints the output to the terminal with some special logic
    - if head == 1, no list 
    - if only one object is added (body, version, date), only prints the data
- Parses command line arguments `./chlgp get <filename> --head n body version date json`
    - And nothing else. 

