bc is a command line calculator, type bc at the command line of most linux systems and you get a copyright notice then
the program waits for you to type in a bit of math, then bc will evaluate that math and print an answer, then wait for
the user to type more.


Your subset of bc needs to


support arithmetic (+ - * /)
support parenthesis
support variables
variables that are referenced but not yet assigned automatically get a value of zero.
allow assignment to variables using =
Support comparisons < =< etc. (return 1 for true and 0 for false)
support passing arithmetic expressions on the command line as command line parameters when starting bc
eg:
bc 5+3
should invoke bc for the single computation, print the answer and then exit the program.
support using the help flag: bc --help should explain what your subset of bc can do.

source for evaluation algorithm:
https://www.geeksforgeeks.org/expression-evaluation/

notes:
-multiple digit numbers cannot have spaces between digits (ex. 32 cannot be expressed as 3 2)
-multiplication cant be done by (2)(3) or 2(3), must be (2)*(3) or 2*(3).
