#[derive(Deserialize, Debug)]
struct Input {
    name : String,
    email: String,
}

async fn accept_form(Form(input): Form<Input>) -> Html<&' static str> {
    tracing::debug!("form params {:?}", input);

    Html("<h3>Form posted</h3>")
}
