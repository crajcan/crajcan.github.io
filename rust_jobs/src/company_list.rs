use yew::prelude::*;
use crate::company::Company;

#[derive(Properties, PartialEq)]
pub(crate) struct CompanyListProps {
    pub companies: Vec<Company>,
    pub on_click: Callback<Company>
}

#[function_component(CompanyList)]
pub(crate) fn company_list(CompanyListProps{ companies, on_click }: &CompanyListProps) -> Html {
    //clone on_click because it gets passed to every CompanyList
    let on_click = on_click.clone();

    companies.iter().map(|company| {

        // build a CallBack to bind to the html element 
        let on_company_select = {
            let on_click = on_click.clone();
            let company = company.clone();

            Callback::from(move |_| {
                on_click.emit(company.clone())
            })
        };

        html! {
            // bind the Callback to the html element
            <li class="company" key={company.id} onclick={on_company_select}>{format!("{}: {}", company.name, company.space)}</li>
        }
    }).collect()
}