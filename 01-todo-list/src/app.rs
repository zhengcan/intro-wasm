use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct Todo {}

#[function_component]
pub fn App() -> Html {
    let todos = use_state(|| Vec::<Todo>::new());

    {
        let todos = todos.clone();
        use_effect_with_deps(move |_| {
            todos.to_vec().push(Todo{});
        }, ());
    }

    html! {
        <div className="todoapp stack-large">
            <h1>{"TodoMatic"}</h1>
            <form>
                <h2 className="label-wrapper">
                    <label htmlFor="new-todo-input" className="label__lg">
                        {"What needs to be done?"}
                    </label>
                </h2>
                <input
                    type="text"
                    id="new-todo-input"
                    className="input input__lg"
                    name="text"
                    autoComplete="off"
                    />
                <button type="submit" className="btn btn__primary btn__lg">
                    {"Add"}
                </button>
            </form>
            <div className="filters btn-group stack-exception">
                <button type="button" className="btn toggle-btn" aria-pressed="true">
                    <span className="visually-hidden">{"Show "}</span>
                    <span>{"all"}</span>
                    <span className="visually-hidden">{" tasks"}</span>
                </button>
                <button type="button" className="btn toggle-btn" aria-pressed="false">
                    <span className="visually-hidden">{"Show "}</span>
                    <span>{"Active"}</span>
                    <span className="visually-hidden">{" tasks"}</span>
                </button>
                <button type="button" className="btn toggle-btn" aria-pressed="false">
                    <span className="visually-hidden">{"Show "}</span>
                    <span>{"Completed"}</span>
                    <span className="visually-hidden">{" tasks"}</span>
                </button>
            </div>
            <h2 id="list-heading">{"3 tasks remaining"}</h2>
            <ul role="list"
                className="todo-list stack-large stack-exception"
                aria-labelledby="list-heading">
                { todos.iter()
                    .map(|todo| {
                        html! {
                            <li className="todo stack-small">
                                <div className="c-cb">
                                    <input id="todo-0" type="checkbox" defaultChecked={"true"} />
                                    <label className="todo-label" htmlFor="todo-0">
                                        {"Eat"}
                                    </label>
                                </div>
                                <div className="btn-group">
                                    <button type="button" className="btn">
                                        {"Edit "}<span className="visually-hidden">{"Eat"}</span>
                                    </button>
                                    <button type="button" className="btn btn__danger">
                                        {"Delete "}<span className="visually-hidden">{"Eat"}</span>
                                    </button>
                                </div>
                            </li>
                        }
                    })
                    .collect::<Html>() }
            </ul>
        </div>
    }
}
