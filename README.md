# `yes` in Rust

## Why
I coded this to prove that simple programs can have simple code. If you look at the source code for GNU's version of `yes` ([link](https://github.com/coreutils/coreutils/blob/master/src/yes.c)), you can see that it is full of complicated and mostly useless code (not shitting on GNU, I have a lot of respect for the GNU project). For example, Suckless has also reprogrammed `yes` ([link](http://git.suckless.org/sbase/file/yes.c.html)), and their version is simple and bloat-free. The difference between mine and Suckless's, is that mine requires no external libraries (they use their own library which is pretty large), and mine is all in one file (once again they use a .h file that they coded for most of the program).

## Why Rust?
I see Rust as the future of systems programming, and I feel that as the language becomes more optimized, it will eventually be faster than C. I want to code such tools in Rust to encourage others to do the same.

## License
Obviously this project is small and should be used as a resource for research, so I gave it the Unlicense.
