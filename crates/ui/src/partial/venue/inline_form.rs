use api::UiState;
use maud::{html, Markup, Render};
use typed_builder::TypedBuilder;

use crate::{
    atom::input::{InputType, TextField},
    component::{icon, Icons},
};

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(into, strip_option(ignore_invalid, fallback_suffix = "_opt"))))]
pub struct VenueInlineForm<'a> {
    state: &'a UiState,
    #[builder(default)]
    name: Option<String>,
    #[builder(default)]
    address: Option<String>,
    #[builder(default)]
    city: Option<String>,
    #[builder(default)]
    postal_code: Option<String>,
    #[builder(default)]
    country: Option<String>,
}

impl Render for VenueInlineForm<'_> {
    fn render(&self) -> Markup {
        let form_icon = icon(&Icons::Location, None);
        let name_input = TextField::builder()
            .name("venue.name")
            .label(t!("venue.name.label", locale = &self.state.locale))
            .placeholder(t!("venue.name.placeholder", locale = &self.state.locale))
            .input_type(InputType::Text)
            .value_opt(self.name.clone())
            .required(true)
            .build();
        let address_input = TextField::builder()
            .name("venue.address")
            .label(t!("venue.address.label", locale = &self.state.locale))
            .placeholder(t!("venue.address.placeholder", locale = &self.state.locale))
            .input_type(InputType::Text)
            .value_opt(self.address.clone())
            .required(true)
            .build();
        let city_input = TextField::builder()
            .name("venue.city")
            .label(t!("venue.city.label", locale = &self.state.locale))
            .placeholder(t!("venue.city.placeholder", locale = &self.state.locale))
            .input_type(InputType::Text)
            .value_opt(self.city.clone())
            .required(true)
            .build();
        let postal_code_input = TextField::builder()
            .name("venue.postal_code")
            .label(t!("venue.postal_code.label", locale = &self.state.locale))
            .placeholder(t!(
                "venue.postal_code.placeholder",
                locale = &self.state.locale
            ))
            .input_type(InputType::Text)
            .value_opt(self.postal_code.clone())
            .required(true)
            .build();
        let country_input = TextField::builder()
            .name("venue.country")
            .label(t!("venue.country.label", locale = &self.state.locale))
            .placeholder(t!("venue.country.placeholder", locale = &self.state.locale))
            .input_type(InputType::Text)
            .value_opt(self.country.clone())
            .required(true)
            .build();

        html! {
            section {
                header {
                    (form_icon)
                    (t!("venue.form.title", locale = &self.state.locale))
                }
                (name_input)
                (address_input)
                .form_row {
                    (city_input)
                    (postal_code_input)
                }
                (country_input)
            }
        }
    }
}
