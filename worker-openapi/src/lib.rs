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

async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello, Cloudflare Workers!",
    })
}

#[ohkami::worker]
async fn my_worker(b: Bindings) -> Ohkami {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ohkami::new((
        "/".GET(hello),
    ))
}
