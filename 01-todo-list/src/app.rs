use wasm_timer::{UNIX_EPOCH, SystemTime};

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{models::Todo, components::TodoView};

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Filter {
    fn test(&self, todo: &Todo) -> bool {
        match self {
            Self::All => true,
            Self::Active => !todo.completed,
            Self::Completed => todo.completed,
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let input_value = use_state(String::default);
    let filter_value = use_state_eq(Filter::default);
    let todos = use_state(Vec::new);

    {
        let todos = todos.clone();
        use_effect_with_deps(move |_| {
            let mut v = todos.to_vec();
            v.push(Todo{
                id: 0,
                name: String::from("Eat"),
                last_mod: None,
                completed: true,
            });
            v.push(Todo{
                id: 1,
                name: String::from("Drink"),
                last_mod: None,
                completed: false,
            });
            v.push(Todo{
                id: 2,
                name: String::from("Sleep"),
                last_mod: None,
                completed: false,
            });
            todos.set(v);
        }, ());
    }

    let onchange = {
        let input_value = input_value.clone();
        Callback::from(move |e: Event| {
            if let Some(element) = e.target_dyn_into::<HtmlInputElement>() {
                input_value.set(element.value());
            }
        })
    };

    let onsubmit = {
        let input_value = input_value.clone();
        let todos = todos.clone();
        Callback::from(move |e: SubmitEvent| {
            if !input_value.is_empty() {
                let mut v = todos.to_vec();
                v.push(Todo{
                    id: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                    name: input_value.to_string(),
                    last_mod: None,
                    completed: false,
                });
                todos.set(v);
                input_value.set(String::default());
            }
            e.prevent_default();
        })
    };
    
    let onfilter = {
        let filter_value = filter_value.clone();
        Callback::from(move |filter: Filter| {
            filter_value.set(filter);
        })
    };

    let oncomplete = {
        let todos = todos.clone();
        Callback::from(move |id: u64| {
            let mut v = todos.to_vec();
            if let Some(found) = v.iter_mut().find(|todo| todo.id == id) {
                found.completed = !found.completed;
            }
            todos.set(v);
        })
    };

    let onedit = {
        let todos = todos.clone();
        Callback::from(move |id: u64| {
            let mut v = todos.to_vec();
            if let Some(found) = v.iter_mut().find(|todo| todo.id == id) {
                found.last_mod = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());
            }
            todos.set(v);
        })
    };

    let ondelete = {
        let todos = todos.clone();
        Callback::from(move |id: u64| {
            let v = todos.to_vec();
            let v = v.iter().filter_map(|todo| if todo.id != id {Some(todo.clone())} else {None}).collect::<Vec<Todo>>();
            todos.set(v);
        })
    };

    html! {
        <div class="todoapp stack-large">
            <h1>{"TodoMatic"}</h1>
            <form {onsubmit}>
                <h2 class="label-wrapper">
                    <label for="new-todo-input" class="label__lg">
                        {"What needs to be done?"}
                    </label>
                </h2>
                <input
                    type="text"
                    id="new-todo-input"
                    class="input input__lg"
                    name="text"
                    autoComplete="off"
                    value={input_value.to_string()}
                    {onchange}
                    />
                {" "}
                <button type="submit" class="btn btn__primary btn__lg">{"Add"}</button>
            </form>
            <div class="filters btn-group stack-exception">
                <button type="button" class="btn toggle-btn" aria-pressed="true" onclick={onfilter.reform(move |_| Filter::All)}>
                    <span class="visually-hidden">{"Show "}</span>
                    <span>{"all"}</span>
                    <span class="visually-hidden">{" tasks"}</span>
                </button>
                <button type="button" class="btn toggle-btn" aria-pressed="false" onclick={onfilter.reform(move |_| Filter::Active)}>
                    <span class="visually-hidden">{"Show "}</span>
                    <span>{"active"}</span>
                    <span class="visually-hidden">{" tasks"}</span>
                </button>
                <button type="button" class="btn toggle-btn" aria-pressed="false" onclick={onfilter.reform(move |_| Filter::Completed)}>
                    <span class="visually-hidden">{"Show "}</span>
                    <span>{"completed"}</span>
                    <span class="visually-hidden">{" tasks"}</span>
                </button>
            </div>
            <h2 id="list-heading">{todos.iter().filter(|todo| !todo.completed).count()}{" tasks remaining"}</h2>
            <ul role="list"
                class="todo-list stack-large stack-exception"
                aria-labelledby="list-heading">
                { todos.iter()
                    .filter(|todo| filter_value.test(todo))
                    .map(|todo| {
                        let todo = todo.clone();
                        let id = todo.id;
                        html! {
                            <TodoView
                                todo={todo}
                                oncomplete={oncomplete.reform(move |_| id)}
                                onedit={onedit.reform(move |_| id)}
                                ondelete={ondelete.reform(move |_| id)}
                            />
                        }
                    })
                    .collect::<Html>() }
            </ul>
        </div>
    }
}
