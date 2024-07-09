# Design Document

## Basic information
- This design document was last updated: 6th of March 2022
- Application title: schedule-tasks
- Author: Sietse van der Bom
- Reviewers: DK, RG
- Programming language: Rust 2018 (rustc 1.59.0)

## Description
The application helps to schedule computer resources for a specific set of tasks. The aggregate of these tasks is a job.

The application takes an input file with a job description consisting of following task details:
- Tasks to be run,
- For each task: the time duration of running the task,
- For each task: its dependencies, i.e. tasks that need to be completed prior to the task at hand.

The application validates the input file. When valid, it runs and creates an output file which contains:
- Critical path of the job,
- The minimum duration of the job,
- The maximum number of tasks that will run in parallel during the job.

## Assumptions
- A computer can only handle one task at a time, so one computer can not handle multiple tasks simultaneously.
- There are no cyclical references for the dependencies in the input file.
- The application will check the validity of the input file and will only create output files for valid input files.
- Input file errors will be reported via stdout with reference to the line and column number of the input file, and the kind of error.
- An unlimited number of computers is available.
- Tasks will be scheduled at the earliest possible time given its dependencies. So tasks will not be postponed to have fewer computers run in parallel.
- The critical path consists of the longest stretch of dependent tasks measuring the total duration to complete them. When multiple critical paths exist, i.e. paths with the same total duration, an arbitrary one of these is selected and presented as 'the critical path' in the output file.

## Input file format and validation
The application parses the input file. The parser should be able to deal with the rather loose input file format. Example of input file:
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

### Validation rules
1. Starts with the `task-name`: an alphanumeric field of one or more characters.
2. Followed by the `duration`: between parentheses, a non-negative integer.
3. Eventually followed by its `dependencies`: between brackets after the keyword 'after', the task-name or the task-names separated by commas.
4. A task-name (see rule 1) should always start on a new line and be directly followed by its duration without whitespace.
5. Except from the previous rule, extra whitespace and newlines are allowed.
6. Job consistency: there should at least be one task without dependencies.
7. Job consistency: tasks listed as dependencies should exist as tasks elsewhere in the input file.
8. Job consistency: tasks should have a unique task-name.

### Parsing
For parsing of the input file the nom parser combinators library will be used: https://github.com/Geal/nom

## Algorithms, data structures and performance
For the scheduling we will use:
- A `Task` struct with properties: `name` (string), `duration` (usize), `start-time` (usize), `end-time` (usize), `dependencies` (a Vector with task-names, i.e. strings).
- A `Scheduler` struct with properties: `unscheduled-tasks `(Vector of tasks), `scheduled-tasks` (Vector of tasks), `critical-path` (Vector of tasks), `minimum-job-duration` (usize), `maximum-parrallellism` (usize).

After parsing the `unscheduled-tasks` Vector will contain all tasks als elements. The processing will be as follows:
- We loop [1] over the `unscheduled-tasks` vector to identify tasks without dependencies and store their indices. Loop over the stored indices to transfer tasks from `unscheduled-tasks` to `scheduled-tasks`. The start-time (is 0) and end-time (is start-time plus duration) property is set.
- In the same loop [1] we take a task from the `unscheduled-tasks` vector and loop [2] over its dependencies while comparing these in a loop [3] to the names of the tasks in the `scheduled-tasks` vector. When all dependencies are present in `scheduled-tasks` (dependencies fulfilled) we break out of loop [2] and [3] and we set the properties of the task: a start-time equal to the maximum end-time of its dependencies (this maximum is tracked in a variable while looping over the `scheduled tasks`) and its end-time (start-time plus duration).
- We continue looping over the `unscheduled-tasks` (possibly multiple times) until there are no more tasks in the vector and all tasks are scheduled. Now the schedule is finished.
- Output: *The minimum duration of the job* is tracked in a variable during looping (so no extra loop), i.e. the maximum end-time given as a property to a task.
- Output: *The maximum parallelism* is determined by looping (or iterating) over the finished `scheduled-tasks` vector for each time-interval between 0 and *the minimum duration of the job*. For each interval can be determined how many tasks are active (i.e. the interval is within start- and end-time of the task) and the maximum of all these intervals gives the max parallelism.
- Output: *Critical path of the job* is determined by looping (or a filter) over the finished `scheduled-tasks` vector. Start with the (last performed) task with highest end-time, then find the previous task by searching on the dependency that has and end-time which matches this last task's start-time, and repeat till start-time 0 is reached.   

To make the schedule while fulfilling the dependencies we have chosen for looping-structures (mainly) over 2 vectors (the `unscheduled-task` and the `scheduled-task` vectors): 

Advantages:
- The vectors are easier to manipulate with indices without mutability issues.
- One could also work with one vector and give the tasks a boolean property `scheduled` but that would give more elements to the loop and thus be less performant.

Disadvantages:
- In case of many tasks the performance has O(N^2) or worse (multiple loops) characteristics, so performance degrades at high load.
- From a programming perspective working with iterators could be more pleasant/ easier to read the code.
 
Relatively easy performance wins could be obtained by:
- Sorting the `unscheduled-tasks` vector to have (1) tasks without dependencies at the front and (2) have 'dependees' in front of dependencies (by comparing depencies to task-name).
- Using HashMaps for `unscheduled-tasks` and `scheduled-tasks` which give O(N), i.e. linear, lookup performance. 
- Using HashSets or HashTreeSets for the names of `scheduled-tasks` (to see quickly whether dependencies are already scheduled). These lookups are also lineair and can replace a part of the looping.

I think some or all of these improvements would be appropriate additions to make the app faster in case of use with really large input-files.

Self-referencing  
The Task type has a dependencies property. We chose to reference to task-names (i.e. strings) and assemble these in a vector. This requires us to loop/ find for the relating task instances. One might also consider to reference to the actual memory locations of the instances of the Task type instead. Self-referencing is in itself an 'unsafe' operation, but using more complex Rust techniques like pin and unpin this might be feasible.

Advantages:
- Extreme performance increase as start- and end-times are 'within reach'.
- Cyclical references can be determined.

Disadvantages:
- Significant increase in complexity of code and its maintainability.

## User interface
The user runs the application via a terminal command-line. The "schedule-tasks" command takes a single file as an argument and prints output to stdout. For example, following command (from the directory in which the schedule-task executable is):
```
./schedule-tasks test/3-independent-tasks.tasks.in
```
The CLI help information can be found using:
```
./schedule-tasks --help
```

## Building, running and testing
Install Rust. The project can be build in the standard Rust way using cargo:
- `cargo run` compiles into the `target/debug` directory and runs the executable (for development).
- `cargo build --release` compiles an optimized production build into the `target/release` directory.

At the root of the project is a `test` directory containing input files:
- Test input files are named: `<my_descriptive_name>.tasks.in`
- Expected outputs are named: `<my_descriptive_name>.sched.out`

To run and view results of unit- and integration-tests use `cargo test` in the root directory of the project.

## Milestones
1. Concise Design Document PR reviewed and accepted.
2. Final PR of code.

Approximate planning:
- Milestone 1 deadline: 21st February 2022
- Milestone 2: the week starting 21st February 2022


