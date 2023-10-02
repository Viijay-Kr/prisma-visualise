use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CodeHighlightInput<'r> {
    code: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CodeHighlightOutput<'r> {
    html: &'r str,
}
/**
 * Highlights the model snippet using ast representation and 
 */
#[post("/api/v1/code_highlight", data = "<code>")]
pub async fn code_highlight(
    code: Json<CodeHighlightInput<'_>>,
) -> Option<Json<CodeHighlightOutput>> {
    todo!()
}
