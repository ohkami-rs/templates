use ohkami::prelude::*;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let o = Ohkami::new((
        "/".GET(|| async {"Hello, AWS Lambda!"}),
    ));

    lambda_runtime::run(o).await
}
