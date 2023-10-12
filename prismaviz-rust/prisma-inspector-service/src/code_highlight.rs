use prismaviz::{
    attributes::{PslArgument, PslAttribute},
    field_type::PslField,
};
use psl_core::{diagnostics::Diagnostics, parser_database::ParserDatabase};
use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct WeakSpan {
    pub start: usize,
    pub end: usize,
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct CodeHighlightInput {
    span: WeakSpan,
    schema: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CodeHighlightOutput {
    code: HtmlLayout,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct HtmlLayout {
    pub html: String,
    pub span: WeakSpan,
}
impl HtmlLayout {
    fn new() -> HtmlLayout {
        HtmlLayout {
            html: String::from(""),
            span: WeakSpan { start: 0, end: 0 },
        }
    }
}

/**
 * Highlights the model snippet using ast representation and
 */
#[post("/api/v1/code_highlight", data = "<input>")]
pub fn code_highlight(input: Json<CodeHighlightInput>) -> Option<Json<CodeHighlightOutput>> {
    let mut diagnostics = Diagnostics::default();
    let parser_db = ParserDatabase::new(input.schema.clone().into(), &mut diagnostics);
    let mut code: Vec<HtmlLayout> = vec![];
    for model in parser_db.walk_models() {
        let mut html_layout = HtmlLayout::new();

        let model_open = format!(
            r#"<div class="model-name-open">
                 <span class="keyword model-keyword">model</span>
                 <span class="model-name">{}</span>
                 <span class="open-curly">{{</span>
               </div>
            "#,
            model.name()
        );
        let mut fields_markup: Vec<String> = vec![];
        for field in model.fields() {
            let mut psl_field = PslField::new(field.name().to_string());
            psl_field.resolve_field(field);

            let field_name_markup = psl_field.resolve_field_name_markup();
            let field_type_markup = psl_field.resolve_field_type_markup();
            let attributes_markup = psl_field.resolve_attributes_markup();
            fields_markup.push(format!(
                r#"
                      <div class="field-name-container">
                        {}
                      </div>
                      <div class="field-attributes-container">
                        {}
                        {}
                      </div>
                "#,
                field_name_markup,
                field_type_markup,
                attributes_markup.join("")
            ));
        }
        // End of fields wrapper
        let model_attributes_markup = model
            .ast_model()
            .attributes
            .iter()
            .map(|attr| {
                let mut attribute =
                    PslAttribute::new("@".to_string() + &attr.name.name.clone(), false, true);
                attribute.resolve_arguments(attr.arguments.clone());
                attribute.attribute_markup()
            })
            .collect::<Vec<String>>();

        html_layout.span = WeakSpan {
            start: model.ast_model().span.start,
            end: model.ast_model().span.end,
        };
        html_layout.html = format!(
            r#"
            <div class="models-wrapper">
                {}
                <div class="fields-container">
                    {}
                </div>
                <div class="model-attributes">
                    {}
                </div>
                <div> 
                    <span class="close-curly">}}</span>
                </div>
            </div>
        "#,
            model_open,
            fields_markup.join("\n"),
            model_attributes_markup.join("\n")
        );
        code.push(html_layout);
    }
    let active_model_code = code
        .into_iter()
        .find(|c| c.span.start == input.span.start && c.span.end == input.span.end);
    match active_model_code {
        Some(code) => Some(Json(CodeHighlightOutput { code })),
        None => {
            panic!("Model not found")
        }
    }
}
