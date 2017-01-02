# fnl

A tiny __awk__ replacement for distributed systems.

## Goals

1. Implement the most common use cases of awk
1. Avoid string based arguments.

### Goal 1
Serves to limit the scope of this tool. Awk itself is a fully featured programming language. __Fnl__ rather is appropriate when you need:

* To grab one or several columns of data.
* To perform simple calculations on one or more columns of data.

### Goal 2

Is the primary reason this tool exists in the first place. Dealing with distributed systems often involves running commands across multiple systems. Most existing unix tools take string based arguments. This means that you only get one or two levels of command nesting.

`$ parallel 'ssh {} "another-command argument1 argument2"'`

It is very easy to quickly run out of levels of nesting. In addition string based arguments require that you escape and unescape, etc. Not ideal.

__Fnl__ takes the approach used by [jt](http://) and passes arguments directly to the program.

`$ fnl %1 %2` or `$ fnl %1 [ %1 %2 add ]`

## Use

The most basic usage of __fnl__ is to extract columns from stdin.

```
$ echo 'a      b\t\tc' | fnl [ %2 ]
b
```

__Fnl__ breaks input lines into columns on any whitespace characters.

The argument __[__ begins a stack expression and __]__ ends it. Inside this expression __%2__ tells __fnl__ that we wish to extract the second column.

Since this usage is so common `[ %2 ]` can be rewritten `%2` for brevity. 

In the next example we extract all three columns in reverse using the shorter expression syntax.

```
$ echo 'a      b\t\tc' | fnl %3 %2 %1
c	b	a
```

Output is __tab__ delimited so that it is ready for interop with __cut__ by default.