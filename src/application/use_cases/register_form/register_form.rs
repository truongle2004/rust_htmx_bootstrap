use leptos::{
    ev::{Event, SubmitEvent},
    html::Input,
    prelude::*,
    tachys::view,
};
use leptos_router::hooks::use_navigate;

#[component]
pub fn RegisterFormUseCase() -> impl IntoView {
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (repeat_password, set_repeat_password) = signal("".to_string());
    let (user_name, set_user_name) = signal("".to_string());
    let email_test = RwSignal::new("".to_string());

    let error = RwSignal::new("".to_string());
    let is_error = RwSignal::new(false);
    let is_error_password = RwSignal::new(false);

    let email_input: NodeRef<Input> = NodeRef::new();
    let username_input: NodeRef<Input> = NodeRef::new();
    let password_input: NodeRef<Input> = NodeRef::new();
    let repeat_password_input: NodeRef<Input> = NodeRef::new();

    let navigate = use_navigate();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        set_email.set(email_input.get().expect("email input").value());
        set_password.set(password_input.get().expect("password input").value());
        set_repeat_password.set(
            repeat_password_input
                .get()
                .expect("repeat password input")
                .value(),
        );
        set_user_name.set(username_input.get().expect("username input").value());

        // Check if the password is match, it will return true else return false
        if Some(password.get()) != Some(repeat_password.get()) {
            error.set("Passwords don't match".to_string());
            is_error.set(true);
            is_error_password.set(true)

        // If the password is match, we start call login use case
        } else {
            // error.set("Passwords match".to_string());
            // TODO: call the login use case here 
            is_error.set(false);
        }
    };

    let on_click_login = move |_| {
        navigate("/login", Default::default());
    };

    view! {
         <div class="container d-flex align-items-center justify-content-center vh-100">
            <div class="card p-4 shadow-lg" style="width: 30rem;">
                <h3 class="text-center mb-4">"Register"</h3>
                <form on:submit=on_submit>
                    <div class="mb-3">
                        <label for="exampleInputEmail1" class="form-label">"User name"</label>
                        <input
                            type="text"
                            class="form-control"
                            value=user_name
                            id="exampleInputEmail1"
                            name="username"
                            aria-describedby="usernameHelp"
                            placeholder="enter your username"
                            node_ref=username_input
                            required
                        />
                        <div id="emailHelp" class="form-text"></div>
                    </div>
                    <div class="mb-3">
                        <label for="exampleInputEmail1" class="form-label">"Email address"</label>
                        <input
                            type="email"
                            class="form-control"
                            id="exampleInputEmail1"
                            name="email"
                            aria-describedby="emailHelp"
                            value=email_test
                            placeholder="Enter your email"
                            node_ref=email_input
                            required
                        />
                        <div id="emailHelp" class="form-text"></div>
                    </div>
                    <div class="mb-3">
                        <label for="exampleInputPassword1" class="form-label">"Password"</label>
                        <input
                            type="password"
                            class="form-control"
                            id="password_1"
                            name="password"
                            value=password
                            placeholder="Enter your password"
                            node_ref=password_input
                            required
                        />
                    </div>
                    <div class="mb-3">
                        <label for="exampleInputPassword1" class="form-label">"Repeat Password"</label>
                        <input
                            type="password"
                            class="form-control"
                            id="exampleInputPassword1"
                            name="password_2"
                            value=repeat_password
                            placeholder="Enter password again"
                            node_ref=repeat_password_input
                            required
                        />
                    </div>
                    <button type="submit" class="btn btn-primary w-100 mb-3">"Submit"</button>
                    <p class="text-center">
                        "Already have an account?"
                        <span class="text-primary fw-semibold ms-2" style="cursor: pointer;" on:click=on_click_login>"Login"</span>
                    </p>
                    {
                        view! {
                            {
                                move || match is_error.get() {
                                    true if error.get().len() > 0 => {
                                        view! {
                                            <div class="alert alert-danger" role="alert">
                                                {error.get()}
                                            </div>
                                        }.into_any()
                                    },
                                    _ => view! {}.into_any()
                                }
                            }
                        }
                    }
                </form>
            </div>
        </div>
    <div id="response"></div>
    }
}
