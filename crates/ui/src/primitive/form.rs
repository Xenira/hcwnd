use std::fmt::Display;

use api::UiState;
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::{
    component::{icon, Icons},
    htmx::HxEncoding,
    primitive::button::{Button, ButtonType},
};

pub trait FormElement: Render {
    fn set_form(&mut self, id: &str);
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
#[builder(mutators(
    #[mutator(requires = [id])]
    pub fn add_form_element(&mut self, mut element: impl FormElement) -> &mut Self {
        element.set_form(&self.id);
        self.elements.push(element.render());

        self
    }

    pub fn add_element(&mut self, element: impl Render + 'static) -> &mut Self {
        self.elements.push(element.render());

        self
    }
))]
pub struct Form {
    id: String,
    url: String,
    submit_label: String,
    #[builder(default)]
    submit_icon: Option<Icons>,
    #[builder(default)]
    back_action: Option<BackAction>,
    #[builder(default)]
    method: Method,
    #[builder(default)]
    header: Option<Markup>,
    #[builder(default="main".to_string())]
    hx_target: String,
    #[builder(default = true)]
    hx_boost: bool,
    #[builder(default = true)]
    hx_push_url: bool,
    #[builder(default)]
    hx_encoding: Option<HxEncoding>,
    #[builder(via_mutators(init = vec![]))]
    elements: Vec<Markup>,
}

impl Form {
    pub fn add_form_element(&mut self, mut element: impl FormElement) -> &mut Self {
        element.set_form(&self.id);
        self.elements.push(element.render());

        self
    }

    pub fn add_element(&mut self, element: impl Render + 'static) -> &mut Self {
        self.elements.push(element.render());

        self
    }
}

impl Render for Form {
    fn render(&self) -> Markup {
        let submit_button = Button::builder()
            .label(&self.submit_label)
            .button_type(ButtonType::Submit)
            .icon_opt(self.submit_icon.clone())
            .form_id(&self.id)
            .build();

        html! {
            section {
                form id=(self.id)
                    action=(self.url)
                    method=(self.method.to_string())
                    hx-target=(self.hx_target)
                    hx-boost=(self.hx_boost)
                    hx-push-url=(self.hx_push_url)
                    hx-encoding=[self.hx_encoding.as_ref().map(ToString::to_string)]
                {
                }

                @if let Some(header) = &self.header {
                    header {
                        (header)
                    }
                }
                @for element in &self.elements {
                    (element)
                }
                footer {
                    @if let Some(back_action) = &self.back_action {
                        button.secondary
                            type="submit"
                            form=(self.id)
                            formaction=(back_action.url)
                            formnovalidate
                        {
                            @if let Some(i) = &back_action.icon {
                                (icon(i, None))
                            }
                            (back_action.label)
                        }
                    } @else {
                        span {}
                    }

                    (submit_button)
                }
            }
        }
    }
}

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct BackAction {
    url: String,
    label: String,
    form_id: String,
    #[builder(default)]
    icon: Option<Icons>,
}

#[derive(Default)]
pub enum Method {
    Get,
    #[default]
    Post,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "get"),
            Method::Post => write!(f, "post"),
        }
    }
}

#[derive(Debug)]
pub enum FormValidation {
    Invalid(String),
    Valid(Option<String>),
    Required(String),
    Warning(String),
    Info(String),
}

impl Render for FormValidation {
    fn render(&self) -> Markup {
        match self {
            FormValidation::Invalid(msg) => html! {
                span.error {
                    (icon(&Icons::ValidationError, None))
                    (msg)
                }
            },
            FormValidation::Valid(msg) => html! {
                span.success {
                    (icon(&Icons::ValidationSuccess, None))
                    @if let Some(msg) = msg  {
                        (msg)
                    }
                }
            },
            FormValidation::Required(msg) => html! {
                span.error {
                    (icon(&Icons::ValidationRequired, None))
                    (msg)
                }
            },
            FormValidation::Warning(msg) => html! {
                span.warning {
                    (icon(&Icons::ValidationWarning, None))
                    (msg)
                }
            },
            FormValidation::Info(msg) => html! {
                span.info {
                    (icon(&Icons::ValidationInfo, None))
                    (msg)
                }
            },
        }
    }
}
