use yew::prelude::*;

use crate::models::Todo;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub todo: Todo,
    pub oncomplete: Callback<Event>,
    pub onedit: Callback<MouseEvent>,
    pub ondelete: Callback<MouseEvent>,
}

#[function_component(TodoView)]
pub fn todo(props: &Props) -> Html {
    let todo = &props.todo;
    let full_name = format!("{}{}", todo.name, match todo.last_mod {
        Some(last_mod) => format!(" @{last_mod}"),
        _ => String::from("")
    });

    html! {
        <li key={todo.id} class="todo stack-small">
            <div class="c-cb">
                <input id={format!("todo-{}", todo.id)} type="checkbox" checked={todo.completed} onchange={&props.oncomplete} />
                {" "}
                <label class="todo-label" for={format!("todo-{}", todo.id)}>{&full_name}</label>
            </div>
            <div class="btn-group">
                <button type="button" class="btn" onclick={&props.onedit}>
                    {"Edit "}<span class="visually-hidden">{&todo.name}</span>
                </button>
                <button type="button" class="btn btn__danger" onclick={&props.ondelete}>
                    {"Delete "}<span class="visually-hidden">{&todo.name}</span>
                </button>
            </div>
        </li>
    }
}
