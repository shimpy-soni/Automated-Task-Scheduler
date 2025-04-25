#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, log, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub scheduled_time: u64,
    pub completed: bool,
}

#[contracttype]
pub enum Taskbook {
    Task(u64),
}

const TASK_COUNT: Symbol = symbol_short!("TASK_CT");

#[contract]
pub struct TaskScheduler;

#[contractimpl]
impl TaskScheduler {
    pub fn create_task(env: Env, title: String, description: String, scheduled_time: u64) -> u64 {
        let mut count: u64 = env.storage().instance().get(&TASK_COUNT).unwrap_or(0);
        count += 1;

        let task = Task {
            id: count,
            title,
            description,
            scheduled_time,
            completed: false,
        };

        env.storage().instance().set(&Taskbook::Task(count), &task);
        env.storage().instance().set(&TASK_COUNT, &count);

        log!(&env, "Task created with ID: {}", count);
        count
    }

    pub fn mark_completed(env: Env, id: u64) {
        let key = Taskbook::Task(id);
        let mut task: Task = env.storage().instance().get(&key).expect("Task not found");

        if task.completed {
            panic!("Task is already completed");
        }

        task.completed = true;
        env.storage().instance().set(&key, &task);
        log!(&env, "Task {} marked as completed", id);
    }

    pub fn get_task(env: Env, id: u64) -> Task {
        let key = Taskbook::Task(id);
        env.storage().instance().get(&key).expect("Task not found")
    }
}