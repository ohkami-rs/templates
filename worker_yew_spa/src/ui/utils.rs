use yew::UseStateHandle;


pub fn set_state<S: Clone>(cards: &UseStateHandle<S>, f: impl FnOnce(&mut S)) {
    cards.set({
        let mut cards = (&**cards).clone();
        f(&mut cards);
        cards
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
