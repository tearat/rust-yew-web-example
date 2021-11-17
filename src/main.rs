use yew::prelude::*;
use fake::{Fake};
use fake::faker::name::raw::*;
use fake::locales::*;
use yew::{Callback, function_component, html, use_state};
use web_sys::HtmlInputElement as InputElement;
use log::Level;
use log::info;

#[derive(Clone, PartialEq)]
struct User {
    id: usize,
    full_name: String,
}

#[derive(Properties, PartialEq)]
struct UsersListProps {
    users: Vec<User>,
}

#[function_component(App)]
fn app() -> Html {
    let users = users_fake_generator(3);
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <hr/>
            <Counter />
            <hr/>
            <UserList users={users} />
        </>
    }
}

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_: MouseEvent| counter.set(*counter + 1))
    };
    html! {
        <>
            <h2>{ "Counter:" }</h2>
            <button {onclick}>{ "Increment value" }</button>
            <p>{ "Current value: " } { *counter }</p>
        </>
    }
}


#[function_component(UserList)]
fn user_list(UsersListProps { users }: &UsersListProps) -> Html {
    let filter = use_state(|| "".to_string());
    let onkeypress = {
        let filter = filter.clone();
        Callback::from(move |event: KeyboardEvent| {
            if &event.key() == "Enter" {
                let input: InputElement = event.target_unchecked_into();
                let value = input.value();
                info!("set filter value to {}", value);
                filter.set(value)
            }
        })
    };

    let users_filtered: Vec<&User> = users
        .iter()
        .filter(|user| user.full_name.to_lowercase().contains(&filter.to_lowercase()[..]))
        .collect();

    let user_list = match users_filtered.len() {
        0 => {
            html! { <p>{ "Not found" }</p> }
        }
        _ => {
            users_filtered
            .iter()
            .map(|user| html! {
                <p>{format!("{}: {}", user.id, user.full_name)}</p>
            })
            .collect::<Html>() 
        }
    };

    html! {
        <>
        <h2>{ "Users list:" }</h2>
        <input type="text" placeholder="Find user..." onkeypress={onkeypress} />
        { user_list }
        </>
    }
}

fn users_fake_generator(count: usize) -> Vec<User> {
    let mut users: Vec<User> = Vec::new();
    for id in 0..count {
        users.push(User { id: id+1, full_name: Name(EN).fake() });
    };
    users
}

fn main() {
    console_log::init_with_level(Level::Debug).expect("Log init failed");
    yew::start_app::<App>();
}
