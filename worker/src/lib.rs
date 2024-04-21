use ohkami::prelude::*;

#[ohkami::worker]
async fn my_worker() -> Ohkami {
    #[cfg(feature = "DEBUG")]
    console_error_panic_hook::set_once();

    Ohkami::new((
        "/".GET(|| async {"Hello, world!"}),
    ))
}
