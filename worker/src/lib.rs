use ohkami::prelude::*;

#[ohkami::bindings]
struct Bindings;

#[ohkami::worker]
async fn my_worker(b: Bindings) -> Ohkami {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ohkami::new((
        "/".GET(|| async {"Hello, Cloudflare Workers!"}),
    ))
}
