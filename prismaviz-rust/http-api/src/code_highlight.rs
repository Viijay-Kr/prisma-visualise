use psl_core::{
    diagnostics::Diagnostics,
    parser_database::{walkers::RefinedFieldWalker, ParserDatabase, ScalarType},
    schema_ast::ast::FieldArity,
};
use rocket::{
    serde::{self, json::Json, Deserialize, Serialize},
    time::format_description::modifier,
};
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
        let mut html = String::from(r#"<div class="model-container">"#);
        let model_name_span = format!(
            r#"<div class="model-name-open">
                 <span class="keyword model-keyword">model</span>
                 <span class="model-name">{}</span>
                 <span class="open-curly">{{</span>
               </div>
            "#,
            model.name()
        );
        html = html + &model_name_span;
        let mut fields_wrapper = format!(
            r#"
            <div class="fields-wrapper">
        "#
        );
        for field in model.fields() {
            let mut field_wrapper = format!(r#"<div class="field-wrapper">"#);
            let field_name_markup = format!(r#"<span class="field-name">{}</span>"#, field.name());
            let refined_field_type = field.refine();
            let field_modifier = match field.ast_field().arity {
                FieldArity::Required => "",
                FieldArity::Optional => "?",
                FieldArity::List => "[]",
            };

            match refined_field_type {
                RefinedFieldWalker::Scalar(f) => {
                    let default_attribute = if f.is_autoincrement() {
                        format!(
                            r#"<span class="attributes">
                                {}
                                <span class="open-argument">(</span>
                                <span class="arguments">{}</span>    
                                <span class="close-argument">)</span>
                               </span>
                            "#,
                            "@default", "autoincrement()"
                        )
                        .to_string()
                    } else {
                        "".to_string()
                    };
                    let id_attribute = if f.is_unique() {
                        r#"
                        <span class="attributes">
                         @id
                        </span>
                    "#
                    } else {
                        ""
                    };
                    let attributes = id_attribute.to_string() + &default_attribute;
                    let field_type = match f.scalar_type() {
                        Some(v) => match v {
                            ScalarType::Int => "Int".to_string(),
                            ScalarType::BigInt => "BigInt".to_string(),
                            ScalarType::Float => "Float".to_string(),
                            ScalarType::Boolean => "Boolean".to_string(),
                            ScalarType::String => "String".to_string(),
                            ScalarType::DateTime => "DateTime".to_string(),
                            ScalarType::Json => "Json".to_string(),
                            ScalarType::Bytes => "Bytes".to_string(),
                            ScalarType::Decimal => "Decimal".to_string(),
                        },
                        None => {
                            println!("Looks like field type is not resolvable");
                            "".to_string()
                        }
                    };
                    let field_type_span = format!(
                        r#"<span class="field-type">{}<span class="field-modifier">{}</span></span>"#,
                        field_type, field_modifier
                    );
                    field_wrapper =
                        field_wrapper + &field_name_markup + &field_type_span + &attributes
                }
                RefinedFieldWalker::Relation(f) => {
                    let field_type = f.name();
                    let field_type_markup = format!(
                        r#"<span class="relational-field-type">{}<span class="field-modifier">{}</span></span>"#,
                        field_type, field_modifier,
                    );
                    let mut relational_field_attributes = r#""#.to_string();
                    let mut relational_reference_attributes = r#""#.to_string();
                    let fields = f.fields();
                    let references = f.referenced_fields();
                    match fields {
                        Some(fields) => {
                            if fields.len() > 0 {
                                let relational_fields = r#"
                                    <span class="argument argument-relational-fields">
                                        fields:
                                        <span class="argument-list-container">
                                            <span class="open square-bracket">[</span>
                                "#
                                .to_string();
                                let arguments = fields.into_iter().map(|f|{
                                        format!(
                                            r#"
                                                <span class="relational-field-list-item argument-list-item">
                                                    {}
                                                </span>
                                            "#,
                                            f.name()
                                        )
                                }).collect::<Vec<String>>().join(r#"<span class="separator"></span>"#);
                                relational_field_attributes = relational_fields
                                    + &arguments
                                    + &r#"
                                            <span class="close square-bracket">]</span> 
                                         </span>
                                       </span>
                                    "#;
                            } else {
                            }
                        }
                        None => {}
                    }
                    match references {
                        Some(fields) => {
                            if fields.len() > 0 {
                                let relational_references = r#"
                                    <span class="argument argument-relational-references">
                                        refernces:
                                        <span class="argument-list-container">
                                            <span class="open square-bracket">[</span>
                                "#
                                .to_string();
                                let arguments = fields.into_iter().map(|f|{
                                        format!(
                                            r#"
                                                <span class="relational-field-list-item argument-list-item">
                                                    {}
                                                </span>
                                            "#,
                                            f.name()
                                        )
                                }).collect::<Vec<String>>().join(r#"<span class="separator"></span>"#);
                                relational_reference_attributes = relational_references
                                    + &arguments
                                    + &r#"
                                            <span class="close square-bracket">]</span> 
                                         </span>
                                       </span>
                                    "#;
                            } else {
                            }
                        }
                        None => {}
                    };
                    let attributes = format!(
                        r#"
                        <span class="attributes">
                            @relation
                            <span class="open-argument">(</span>
                            {}
                            <span class="separator">,</span>
                            {}
                            <span class="close-argument">)</span>    
                        </span>
                    "#,
                        relational_field_attributes, relational_reference_attributes,
                    );
                    field_wrapper =
                        field_wrapper + &field_name_markup + &field_type_markup + &attributes
                }
            }
            field_wrapper = field_wrapper + &format!("</div>");
            fields_wrapper = fields_wrapper + &field_wrapper;
        }
        // End of fields wrapper
        fields_wrapper = fields_wrapper + &format!(r#"</div>"#);

        html = html
            + &fields_wrapper
            + &String::from(
                r#"
                 <span class="closed-curly">}</span>
                 </div>
                "#,
            );

        html_layout.html = html;
        html_layout.span = WeakSpan {
            start: model.ast_model().span.start,
            end: model.ast_model().span.end,
        };
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
