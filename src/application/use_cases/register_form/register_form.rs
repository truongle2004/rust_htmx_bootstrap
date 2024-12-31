use leptos::{ev::SubmitEvent, html::Input, prelude::*};
use leptos_router::hooks::use_navigate;

#[component]
pub fn RegisterFormUseCase() -> impl IntoView {
    //let (email, set_email) = signal("".to_string());
    //
    //let (password, set_password) = signal("".to_string());

    let email_input: NodeRef<Input> = NodeRef::new();
    let username_input: NodeRef<Input> = NodeRef::new();
    let password_input: NodeRef<Input> = NodeRef::new();
    let repeat_password_input: NodeRef<Input> = NodeRef::new();

    let navigate = use_navigate();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        //let email_value = email_input.get().expect("email input").value();
        //let password_value = password_input.get().expect("password input").value();
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
                            id="exampleInputEmail1"
                            name="username"
                            aria-describedby="emailHelp"
                            placeholder="enter your username"
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
                            placeholder="Enter your email"
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
                            placeholder="Enter your password"
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
                            placeholder="Enter password again"
                            required
                        />
                    </div>
                    <button type="submit" class="btn btn-primary w-100 mb-3">"Submit"</button>
                    <p class="text-center">
                        "Don’t have an account?"
                        <span class="text-primary fw-semibold ms-2" style="cursor: pointer;">"Sign Up"</span>
                    </p>
                </form>
            </div>
        </div>
    <div id="response"></div>
    }
}
