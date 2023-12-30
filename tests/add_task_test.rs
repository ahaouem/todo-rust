#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use todo_lib::add_task;

    #[test]
    fn test_add_task() {
        let id = 1;
        let priorities = vec!["high".to_string(), "medium".to_string(), "low".to_string()];

        let _mock_read_input = |prompt: &str| match prompt {
            "Task title: " => "Test title".to_string(),
            "Task description: " => "Test description".to_string(),
            _ => "high".to_string(),
        };

        let mut tasks = Vec::new();

        add_task(&mut tasks, id, &priorities);
        assert_eq!(tasks[0].title, "Test title");
        assert_eq!(tasks[0].description, "Test description");
        assert_eq!(tasks[0].priority, "high");
        assert_eq!(
            tasks[0].due_date,
            NaiveDate::from_ymd_opt(2022, 12, 31).unwrap()
        );
    }
}
