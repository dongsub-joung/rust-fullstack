use leptos::prelude::RwSignal;
use crate::models::Todo;

pub mod app;
pub mod todo_modal;
pub mod todo_list_itme;

pub type TodoListSignal = RwSignal<Vec<Todo>>;
pub type ShowTodoModalSignal = RwSignal<bool>;
pub type EditTodoSignal = RwSignal<Option<Todo>>;
pub type TodoSignal = RwSignal<Todo>;