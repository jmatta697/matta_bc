Author: Joe Matta
Copyright 2018 - Joe Matta. All rights reserved.

This program aims to imitate bc which is a command line calculator. Typing bc at the command line of most linux systems
will bring up a copyright notice then the program waits for you to type in a bit of math, it will then evaluate that math
and print an answer, then wait for the user to type more.

This implementation of bc does the following:

supports arithmetic (+ - * /)
supports parenthesis including nested parentheses
supports variables -
    variables that are referenced but not yet assigned automatically get a value of zero.
    allows assignment to variables using =
supports comparisons < =< etc. (return 1 for true and 0 for false)
supports passing arithmetic expressions on the command line as command line parameters when starting matta_bc
eg:
entering the following in the command line: matta_bc 5+3
    will invoke matta_bc for the single computation, print the answer of 8 and then exit the program.
help (this document) is available through the help flag: matta_bc --help
    - this will bring up this document explaining how to use matta_bc

To run the program from the command line:
Navigate to the folder that holds the executable matta_bc (file with no extension) and type the following:
$ ./matta_bc
Calling the program with no command line arguments will allow the user to enter an expression and obtain a result as
many times as they would like until 'quit' is entered. Entering 'quit' ends the program.
The user may pass a command line argument:
$ ./matta_bc 5+3
If more that one argument is passed to the program call, an error will occur.
$ ./matta_bc 5+3 7-2  ---> error
If the user includes spaces, parentheses, '<', or '>', single quotes are required around the argument. Some other cases
may exist that necessitate the use of quotes. Thus, the user is advised to always use quotes when passing a command line
argument.
$ ./matta_bc '(3+3)/2'

notes:
- Multiple digit numbers cannot have spaces between digits (ex. 32 cannot be expressed as 3 2) when passing an expression
    as a command line argument without quotes
- Multiplication cannot be done by (2)(3) or 2(3), must be (2)*(3) or 2*(3).
- Variable names must start with a lowercase letter. They may only contain lowercase letters and numbers.
- Variable names must end in numbers, if they include numbers. Varibale names cannot have numbers inside name
    (ie. 'var3iable' is not a valid variable name, but 'variable33' is)
- When passing a command line argument without quotes around it, argument cannot contain spaces or parentheses
- Variables can NOT be assigned to other variables (ie. 'variable0 = variable1' is not valid)
- If assigning a value to a variable, user input must start with a lowercase letter followed by any number of any
    combination of lowercase letters and then followed by numbers and then '='. The assigned value may be a
    mathematical expression or a comparison of expressions.
- Dividing by 0 explicitly (ie. entering '3/0') will cause the an error and the user will be prompted to enter a valid
    expression. Dividing by 0 implicitly (ie. entering '3/(2-2)' will cause a fatal error; a message will be printed
    and the program will exit.
- When entering an expression as a command line argument, the user is required to surround the expression with
    single quotes in some cases, specifically in the case of comparisons using '<' or '>'. Single quotes must also be
    used when the expression contains parentheses. For most other cases, the expression can be entered without being
    surrounded by quotes. This is the result of this notation coinciding with bash commands.
    Example: 34<=100 must be entered as '34<=100' as a command line argument
    Example: (3+3)/2 must be entered as '(3+3)/2' as a command line argument

source for expression evaluation algorithm:
https://www.geeksforgeeks.org/expression-evaluation/