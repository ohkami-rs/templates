mod api;
mod models;

use ohkami::prelude::*;


#[ohkami::bindings]
struct Bindings;

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let fangs = {
        #[cfg(debug_assertions)]
        ohkami::fang::CORS::new("http://127.0.0.1:8080")
    };

    Ohkami::with(fangs, (
        /* in production, `./dist` contents are served by `--assets dist` of `deploy` script in package.json */

        "/hello".GET(api::hello),
    ))
}
