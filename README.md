# properties-file-parser
## Description
The goal of this Rust project is to parse .properties file (https://en.wikipedia.org/wiki/.properties)\
\
This project is uploaded on crates.io:\
https://crates.io/crates/properties-file-parser

## Grammar
This project is created with pest crate.
The grammar is following:
### comment
Parses comment, which can start with # or ! 
### spaces
One or more spaces or tabs
### property
property, which is:\
key=value, OR\
key: value, OR\
key value
### key
key in property
### value
value in property
### file
parses the whole file .properties
### silentEOI
used in order not to get "EOI" in output