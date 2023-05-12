use salvo::prealude::*

#[handler]
async fn hello() -> &'static str {
    "Hola, Mundo!"
}