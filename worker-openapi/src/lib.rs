use ohkami::prelude::*;
#[cfg(feature = "openapi")]
use ohkami::openapi;

#[ohkami::bindings]
struct Bindings;

#[derive(Serialize)]
#[cfg_attr(feature = "openapi", derive(openapi::Schema))]
struct Message {
    message: &'static str,
}

async fn hello() -> JSON<Message> {
    JSON(Message {
        message: "Hello, Workers!",
    })
}

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ohkami::new((
        "/".GET(hello),
    ))
}
