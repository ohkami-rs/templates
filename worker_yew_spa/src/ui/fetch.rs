use ohkami::serde::Serialize;

pub use reqwest::Error;


pub struct Client(reqwest::Client);

impl Client {
    const ORIGIN: &'static str = {
        #[cfg(feature="DEBUG")] {"http://localhost:8787"}
        #[cfg(not(feature="DEBUG"))] {todo!()}
    };
}

macro_rules! call {
    ( $( $method:ident & $with_body_method:ident ),* ) => {
        #[allow(non_snake_case, unused)]
        impl Client {$(
            pub async fn $method(&self,
                path: impl AsRef<str>
            ) -> Result<reqwest::Response, Error> {
                self.0.request(
                    reqwest::Method::$method,
                    format!("{}{}", Self::ORIGIN, path.as_ref())
                ).send().await
            }

            pub async fn $with_body_method<Body: Serialize>(&self,
                body: Body,
                path: impl AsRef<str>
            ) -> Result<reqwest::Response, Error> {
                self.0.request(
                    reqwest::Method::$method,
                    format!("{}{}", Self::ORIGIN, path.as_ref())
                ).json(&body).send().await
            }
        )*}
    };
} call! {
    GET & GETwith,
    PUT & PUTwith,
    POST & POSTwith,
    PATCH & PATCHwith,
    DELETE & DELETEwith
}
