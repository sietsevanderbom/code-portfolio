use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::iter::once;
use crate::task::{
    Task,
    TimeMoment,
};


#[derive(Debug, Clone)]
pub struct Scheduler<'a> {
    pub number_of_unscheduled_tasks: usize,
    pub unscheduled_tasks: Vec<Task<'a>>,
    pub scheduled_tasks: Vec<Task<'a>>,
    pub critical_path: Vec<&'a str>,
    pub last_task: Task<'a>,
    pub scheduled_tasks_time_nodes: BTreeSet<TimeMoment>,
    pub max_parallelism: usize,
}

impl<'a> Scheduler<'a> {
    pub fn run(mut self) -> String {
        while self.number_of_unscheduled_tasks > 0 {
            self.process_unscheduled_tasks();
        }
        self.calculate_parallelism();
        self.assemble_critical_path_tasks();
        self.print_output()
    }

    fn process_unscheduled_tasks(&mut self) {
        let mut progress = false;
        let mut start_time: Option<TimeMoment> = Some(0);

        for i in 0..self.unscheduled_tasks.len() {
            if self.taskname_is_not_unique(&self.unscheduled_tasks[i]) {
                panic!("The taskname {} is not unique", &self.unscheduled_tasks[i].name);
            }

            if self.unscheduled_tasks[i].dependencies.is_empty() {
                self.set_start_and_end_times(i, &start_time);
                progress = true;
                break;
            } else {
                let available_start_time = self.start_time_to_be_scheduled_at(&self.unscheduled_tasks[i]);
                if let Some(time) = available_start_time {
                    start_time = Some(time);
                    self.set_start_and_end_times(i, &start_time);
                    progress = true;
                    break;
                }
            }
        };

        assert!(progress, "Progress stopped while scheduling: the input file contains logical \
        inconsistencies, like non-existent dependencies, or no tasks without dependencies, etc.");

        self.set_number_of_unscheduled_tasks();
    }

    fn start_time_to_be_scheduled_at(&self, unscheduled_task: &Task) -> Option<TimeMoment> {
        let mut latest_end_time_of_dependencies = 0;
        let mut number_of_dependencies: usize;
        let mut number_of_fulfilled_dependencies: usize = 0;
        let mut dependencies_fulfilled = false;
        'dependencies: for i in 0..unscheduled_task.dependencies.len() {
            number_of_dependencies = unscheduled_task.dependencies.len();
            for j in 0..self.scheduled_tasks.len() {
                if unscheduled_task.dependencies[i] == self.scheduled_tasks[j].name {
                    number_of_fulfilled_dependencies += 1;
                    if self.scheduled_tasks[j].end_time >= latest_end_time_of_dependencies {
                        latest_end_time_of_dependencies = self.scheduled_tasks[j].end_time;
                    };
                    if number_of_fulfilled_dependencies == number_of_dependencies {
                        dependencies_fulfilled = true;
                        break 'dependencies;
                    }
                }
            }
        }

        if dependencies_fulfilled { Some(latest_end_time_of_dependencies) } else { None }
    }

    fn set_start_and_end_times(&mut self, index: usize, start_time: &Option<TimeMoment>) {
        let mut scheduled_task = self.unscheduled_tasks[index].clone();
        scheduled_task.start_time = start_time.unwrap_or(0);
        scheduled_task.end_time = scheduled_task.start_time + scheduled_task.duration;
        self.scheduled_tasks_time_nodes.insert(scheduled_task.end_time);

        self.set_last_task(scheduled_task.clone());

        self.scheduled_tasks.push(scheduled_task);
        self.unscheduled_tasks.remove(index);
    }

    fn initialize(&mut self) {
        self.set_number_of_unscheduled_tasks();
    }

    fn taskname_is_not_unique(&self, candidate: &Task) -> bool {
        if self.scheduled_tasks.iter().any(|task| task.name == candidate.name) {
            return true;
        }
        false
    }

    fn set_last_task(&mut self, task: Task<'a>) {
        if &task.end_time > self.last_task.end_time.borrow() {
            self.last_task = task;
        };
    }

    fn set_number_of_unscheduled_tasks(&mut self) {
        self.number_of_unscheduled_tasks = self.unscheduled_tasks.len();
    }

    fn calculate_parallelism(&mut self) {
        let scheduled_tasks_time_nodes: Vec<_> = self.scheduled_tasks_time_nodes.iter().rev().collect();

        for node in scheduled_tasks_time_nodes.windows(2) {
            let mut counter: usize = 0;

            // loop through reversed schedule: last tasks to first tasks
            for task in self.scheduled_tasks.iter().rev() {
                if self.do_overlap(*node[1], *node[0], task.start_time, task.end_time) {
                    counter += 1;
                };
                if counter > self.max_parallelism {
                    self.max_parallelism = counter;
                };
            }
        }
    }

    fn do_overlap(&self, a_start: TimeMoment, a_end: TimeMoment, b_start: TimeMoment, b_end: TimeMoment) -> bool {
        a_start < b_end && b_start < a_end
    }

    fn assemble_critical_path_tasks(&mut self) {
        self.critical_path.push(self.last_task.name);
        let mut dependencies: Vec<String> = self.last_task.dependencies.clone();
        let mut critical_path_task_start_time: TimeMoment = self.last_task.start_time;
        let mut critical_path_task: Task = Default::default();
        let mut finished = false;

        while !finished && !dependencies.is_empty() {
            'dependencies: for dependency in dependencies {

                // to track the critical path loop through reversed schedule: from last task to first task
                for task in self.scheduled_tasks.iter().rev() {
                    if task.name == dependency && task.end_time == critical_path_task_start_time {
                        self.critical_path.push(task.name);
                        critical_path_task_start_time = task.start_time;
                        critical_path_task = task.clone();
                        if task.start_time == 0 {
                            finished = true;
                        }
                        break 'dependencies;
                    }
                }
            }
            dependencies = critical_path_task.dependencies.clone();
        }
    }

    pub fn print_output(&self) -> String {
        format!(r#"Critical: {}
Minimum: {}
Parallelism: {}"#, self.print_critical_path(self.critical_path.clone()), self.last_task.end_time, self.max_parallelism)
    }

    fn print_critical_path(&self, input: Vec<&str>) -> String {
        let formatted: String = input
            .iter()
            .rev()
            .flat_map(|element| once("->").chain(once(*element)))
            .skip(1)
            .collect();
        formatted
    }
}

pub fn build_scheduler(unscheduled_tasks: Vec<Task>) -> Scheduler {
    let mut scheduler = Scheduler {
        number_of_unscheduled_tasks: 0,
        unscheduled_tasks,
        scheduled_tasks: vec![],
        critical_path: vec![],
        last_task: Default::default(),
        scheduled_tasks_time_nodes: BTreeSet::from([0]),
        max_parallelism: 0,
    };

    scheduler.initialize();
    scheduler
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["A".to_string()]);
        let c = Task::new("C", 1, vec!["A".to_string()]);

        let tasks = vec![a, b, c];

        let scheduler = build_scheduler(tasks);
        let schedule_output = scheduler.run();

        assert_eq!(schedule_output, r#"Critical: A->B
Minimum: 2
Parallelism: 2"#)
    }

    #[test]
    #[should_panic(expected = "Progress stopped while scheduling: the input file contains logical inconsistencies, like non-existent dependencies, or no tasks without dependencies, etc.")]
    fn run_only_tasks_with_dependencies() {
        let a = Task::new("A", 1, vec!["B".to_string()]);
        let b = Task::new("B", 1, vec!["A".to_string()]);

        let tasks = vec![a, b];

        let scheduler = build_scheduler(tasks);
        scheduler.run();
    }

    #[test]
    #[should_panic(expected = "Progress stopped while scheduling: the input file contains logical inconsistencies, like non-existent dependencies, or no tasks without dependencies, etc.")]
    fn run_with_non_existing_dependencies() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["C".to_string()]);

        let tasks = vec![a, b];

        let scheduler = build_scheduler(tasks);
        scheduler.run();
    }

    #[test]
    fn process_unscheduled_tasks_one_loop() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["A".to_string()]);
        let c = Task::new("C", 1, vec!["A".to_string()]);

        let tasks = vec![a.clone(), b.clone(), c.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.process_unscheduled_tasks();
        assert_eq!(scheduler.number_of_unscheduled_tasks, 2);
        assert_eq!(scheduler.scheduled_tasks, [Task { name: "A", duration: 1, start_time: 0, end_time: 1, dependencies: vec![] }]);
        assert_eq!(scheduler.last_task, Task { name: "A", duration: 1, start_time: 0, end_time: 1, dependencies: vec![] });
        assert_eq!(scheduler.scheduled_tasks_time_nodes, BTreeSet::from([0, 1]));
    }

    #[test]
    fn process_unscheduled_tasks_two_loops() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["A".to_string()]);
        let c = Task::new("C", 1, vec!["A".to_string()]);

        let tasks = vec![a.clone(), b.clone(), c.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();
        assert_eq!(scheduler.number_of_unscheduled_tasks, 1);
        assert_eq!(scheduler.scheduled_tasks, [
            Task { name: "A", duration: 1, start_time: 0, end_time: 1, dependencies: vec![] },
            Task { name: "B", duration: 1, start_time: 1, end_time: 2, dependencies: vec!["A".to_string()] }]);
        assert_eq!(scheduler.last_task, Task { name: "B", duration: 1, start_time: 1, end_time: 2, dependencies: vec!["A".to_string()] });
        assert_eq!(scheduler.scheduled_tasks_time_nodes, BTreeSet::from([0, 1, 2]));
    }

    #[test]
    fn process_unscheduled_tasks_three_loops() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["A".to_string()]);
        let c = Task::new("C", 1, vec!["A".to_string()]);

        let tasks = vec![a.clone(), b.clone(), c.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();
        assert_eq!(scheduler.number_of_unscheduled_tasks, 0);
        assert_eq!(scheduler.scheduled_tasks, [
            Task { name: "A", duration: 1, start_time: 0, end_time: 1, dependencies: vec![] },
            Task { name: "B", duration: 1, start_time: 1, end_time: 2, dependencies: vec!["A".to_string()] },
            Task { name: "C", duration: 1, start_time: 1, end_time: 2, dependencies: vec!["A".to_string()] }]);
        assert_eq!(scheduler.last_task, Task { name: "B", duration: 1, start_time: 1, end_time: 2, dependencies: vec!["A".to_string()] });
        assert_eq!(scheduler.scheduled_tasks_time_nodes, BTreeSet::from([0, 1, 2]));
    }

    #[test]
    fn initialize() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["A".to_string()]);
        let c = Task::new("C", 1, vec!["A".to_string()]);

        let tasks = vec![a.clone(), b.clone(), c.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.initialize();
        assert_eq!(scheduler.number_of_unscheduled_tasks, 3);
    }

    #[test]
    #[should_panic(expected="The taskname A is not unique")]
    fn taskname_is_not_unique() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("A", 1, vec![]);

        let tasks = vec![a.clone(), b.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();
    }

    #[test]
    fn calculate_parallelism() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["A".to_string()]);
        let c = Task::new("C", 1, vec!["A".to_string()]);

        let tasks = vec![a.clone(), b.clone(), c.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.process_unscheduled_tasks();
        scheduler.calculate_parallelism();
        assert_eq!(scheduler.max_parallelism, 1);
        scheduler.process_unscheduled_tasks();
        scheduler.calculate_parallelism();
        assert_eq!(scheduler.max_parallelism, 1);
        scheduler.process_unscheduled_tasks();
        scheduler.calculate_parallelism();
        assert_eq!(scheduler.max_parallelism, 2);
    }

    #[test]
    fn assemble_critical_path_when_only_tasks_without_dependencies() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 8, vec![]);
        let c = Task::new("C", 3, vec![]);

        let tasks = vec![a.clone(), b.clone(), c.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();

        scheduler.assemble_critical_path_tasks();

        assert_eq!(scheduler.critical_path, vec!["B"]);
    }

    #[test]
    fn assemble_critical_path_tasks_final_schedule() {
        let a = Task::new("A", 1, vec![]);
        let b = Task::new("B", 1, vec!["A".to_string()]);
        let c = Task::new("C", 100, vec!["A".to_string()]);

        let tasks = vec![a.clone(), b.clone(), c.clone()];

        let mut scheduler = build_scheduler(tasks);
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();
        scheduler.process_unscheduled_tasks();

        scheduler.assemble_critical_path_tasks();

        assert_eq!(scheduler.critical_path, vec!["C", "A"]);
    }
}