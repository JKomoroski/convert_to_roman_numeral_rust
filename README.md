# Roman Numeral Conversion In Rust

### Intro
In order to dig a bit deeper into Rust, I've written a little app that can convert numbers into roman numerals. This builds a binary that accepts a standard in parameter of any positive whole number and converts it. Does not use bar notation. Technically works for any number up to 32 bit max value (4294967295), but that's a lot of 'M' characters to return to your terminal.

### Build
Built using `rustc 1.29.0`.
Clone the repo, navigate to the directory and run:
```
rustc convert_to_roman_numeral.rs
```
This outputs the `convert_to_roman_numeral` binary.

### Useage
```
convert_to_roman_numeral 10
```

### To-Do
- Tests
- Github Release Binary
- Perhaps impliment Bar Notation
- Fix the things I don't like about the code


Rules [here](https://www.mathsisfun.com/roman-numerals.html).
