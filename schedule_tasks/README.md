# Task scheduling

Many applications are made up of lots of little tasks which need
to be scheduled on the available compute resources. In this assignment
you will create a program called "schedule-tasks" that takes an input
set of tasks and schedules them assuming infinite computers are available
to execute the tasks. You are limited only by the dependencies between
tasks - but you need to warn us how many computers you're going to take
in the worst case, and how long you need them for!

## Inputs and Outputs

Input: a file containing the set of tasks, their durations and
dependencies. You may assume that there are no cycles in the task
dependencies. Input files may contain mistakes.

Output: For correct input files, please output the following items

- a critical path of your schedule (print a longest chain of tasks in your run) 
- minimum duration of the job (minimum time to run all the tasks assuming infinite resources)
- maximum parallelism of the job (maximum number of tasks you schedule simultaneously)

## Example

Given this set of tasks in `test/example.tasks.in`:

```
A(1)
B(1) after [A]
C(1) 
  after [A]
D(1) after [B]
F(1) after 
  [B, 
   C]
G(1) after [C]
H(1) after [D, F]
I(1) after 
  [F, G]
```

when invoked as `./schedule-tasks test/example.tasks.in` we would output

```
Critical: A->B->D->H
Minimum: 4
Parallelism: 3
```

In the input file, "A(1)" means a task called "A" which takes 1
unit of time to execute. You may assume all tasks take non-negative
integer time units. You may assume no task has circular dependencies.

For incorrect input files you should print the line and column for the
first error you discover and exit.

## Error examples

An invalid task length:

```
A(-1)
```

should output

```
Error: line 1, column 3
```

Invalid syntax:

```
A(1)
B(1) afer [A]
```

should output

```
Error: line 2, column 6
```

## Testing

You must name test input files as `<name>.tasks.in` and expected outputs
as `<name>.sched.out` where `<name>` is some descriptive name of your choice.

Your "schedule-tasks" command must take a single file as an argument 
and print output to stdout.

That is, invocations look like this:

```bash
./schedule-tasks test/3-independent-tasks.tasks.in
```

## Requested materials

Please submit the following materials in two steps.

First, send us a PR on this repo with a concise design document written
in markdown. Strive for clarity rather than length! The design should 
include a description of how your solution works and any trade-offs you
have made. While not typical for a design doc, in this case, please include
a short description of the tools you expect to use. For example, a Rust 
installation or other build / interpreters / compilers.  

We will review the design and give any guidance or suggestions we can
before you write the code. Once the PR is merged, you can assume we
agree with your approach and that correctly coding it will be a full
and complete solution to the problem.

Next, send us a PR with the code and any tests for your solution. Please
include instructions for compiling and running any tests you have included.

You must have a top-level "test" directory containing input files. We will
run the submitted "schedule-tasks" command on each input file and your outputs
to the associated output file. We will also add our own test files (you do not
need to worry about name clashes with these files). 

## Questions?

If you have questions, please create your design or implementation PR early
and use the discussion feature on the PR. This will ensure a prompt response
from our engineering team.
