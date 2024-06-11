use yew::UseStateHandle;


pub fn set_state<S: Clone>(state: &UseStateHandle<S>, f: impl FnOnce(&mut S)) {
    state.set({
        let mut state = (&**state).clone();
        f(&mut state);
        state
    })
}

pub fn report_error(message: impl Into<String>) {
    web_sys::window().unwrap().alert_with_message(&{
        let mut message = message.into();
        message.push_str("\n\
            ( Maybe the server is sleeping now. Please try later! )");
        message
    }).unwrap();
}
