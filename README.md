# SMP: A SiMPle Programming Language
An extremely simple interpreter written in Rust.
 
## Why
In Spring of 2021, I took [CSE 413](https://courses.cs.washington.edu/courses/cse413/) (Programming Languages & Implementation) at the University of Washington. This class covered the fundamentals of language internals. During this time I was also learning Rust on the side. After completing the class I decided it would be fun to try and implement a new language on my own.
 
## Language Documentation
The Grammar for the smp language was adapted from the article [The memory models that underlie programming languages](http://canonical.org/~kragen/memory-models/), I found this article while browsing Hacker News and thought it looked like a good candidate to try on my own. I did, however, make several adaptations and additions.
 
## The Basics
All smp programs consist of a number of top-level function definitions. For example:
```
def add(a, b) {
    return a + b;
}
 
def main() {
    return add(1, 1);
}
```
All programs must have a 0-argument function called main, this will be the entrypoint of the program. 
Main may optionally return a value, which will display at the end of the program.
All other functions MUST return a value, although this value can be ignored if it is just used for its side effects.
 
Smp statements can be standalone expressions, if their value is not returned or bound to a variable, they will be printed, and then discarded. You can use this to print the value of variables or expressions. For example, this program prints the values from 0 to 10.
```
def main() {
    lo := 0;
    hi := 10;
 
    while (lo < hi) {
        lo;
        lo := lo + 1;
    }
}
```
 
All variables are doubles, or arrays of doubles, however we can perform boolean operations by evaluating the values "truthiness". The truthiness rules are as follows: values within Epsilon (0.0000001) of 0 are considered false, all other values are considered true. 
 
## Language Features
 
### Conditionals And Loops
The following program prints even numbers from 0 to 20
```
def main() {
    i := 0;
    while (i <= 20) {
        if ((i % 2) == 0) {
            i;
        }
        i := i + 1;
    }
}
```

### Recursion
The following program computes 10!
```
def fact(n) {
    if (n == 1) {
        return n;
    }
 
    return n * fact(n-1);
}
 
def main() {
    return fact(10);
}
```

### Arrays
The following program initializes an array of 5 elements, sets their values in a loop, 
outputs the final value, and returns the entire array.
```
def main() {
    arr := [5];
    i := 0;
    while (i < 5) {
        arr[i] := i;
        i := i + 1;
    }
    arr[4];
    return arr;
}
```

### Built-In Functions
Supported functions: `sqrt`, `len`, `round`, `input`
```
def seq(arr) {
    i := 0;
    while (i < (len(arr))) {
        arr[i] := i;
        i := i + 1;
    }
    return arr;
}

def main() {
    arr := seq([round(input())]);
    return round(sqrt(arr[len(arr) - 1]));
}
```

### Comments
Comments are preceded by a `#`, the program will ignore anything after that until the end of the line.
```
# comment
def main() { # the main function is called main()!
    # returns 1; very cool!
    return 1; # we will now return 1!!
    # 1 was returned
}
# the program is over
```

## Should You Use This Language?
No! :)  
This was purely for fun and for learning. This language is most certainly full of bugs, and the code is not very extensible in its current state. However, I hope that anyone reading this can learn from my experiences and mistakes while implementing their own interpreters in Rust or any other language. 

## Planned Features
- Input: Currently all values are hardcoded, I am planning to add a built-in function that can parse a single float from stdin.  
- More Builtin functions
- Imports?
