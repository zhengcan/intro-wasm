use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Todo {
    pub id: u64,
    pub name: String,
    pub last_mod: Option<u64>,
    pub completed: bool,
}
