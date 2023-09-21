use yew::prelude::*;
use crate::company::Company;

#[derive(Properties, PartialEq)]
pub(crate) struct CompanyDetailsProps {
    pub company: Company,
}

#[function_component(CompanyDetails)]
pub(crate) fn company_details(CompanyDetailsProps { company }: &CompanyDetailsProps) -> Html {
    html! {
        <div style="border: 1px solid orange;">
            <h3>{ company.name.clone() }</h3>
            <ul>
              <li>{ format!("website: {}", company.url.clone().unwrap_or("".to_string())) }</li>
              <li>{ format!("space: {}", company.space.clone()) }</li>
              <li>{ format!("startup: {}", company.startup.clone()) }</li>
              <li>{ format!("github: {}", company.open_source_github_org.clone().unwrap_or("".to_string())) }</li>
            </ul>
        </div>
    }
}