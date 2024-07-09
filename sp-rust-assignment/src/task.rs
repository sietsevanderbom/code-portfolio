pub type Duration = usize;
pub type TimeMoment = usize;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Task<'a> {
    pub name: &'a str,
    pub duration: Duration,
    pub start_time: TimeMoment,
    pub end_time: TimeMoment,
    pub dependencies: Vec<String>,
}

impl<'a> Task<'a> {
    pub fn new(name: &'a str, duration: Duration, dependencies: Vec<String>) -> Self {
        Task {
            name,
            duration,
            start_time: 0,
            end_time: 0,
            dependencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let task = Task::new("G", 3, vec!["D".to_string(), "F".to_string()]);
        assert_eq!(task, Task { name: "G", duration: 3, start_time: 0, end_time: 0, dependencies: vec!["D".to_string(), "F".to_string()] })
    }
}