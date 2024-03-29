use yew::prelude::*;

use crate::company::Company;
use crate::company_details::CompanyDetails;
use crate::company_list::CompanyList;

use gloo_net::http::Request;
use gloo_timers::callback::Timeout;

const COMPANIES_URL: &str = "https://rust-jobs-api.fly.dev/companies/query";


#[function_component(App)]
pub fn app() -> Html {
    let companies = use_state(|| vec![]);
    {
        let companies = companies.clone();
        use_effect_with_deps(move |_| {
            let companies = companies.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_companies: Vec<Company> = Request::post(COMPANIES_URL) .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                companies.set(fetched_companies);
            });
            || ()
        }, ());
    }

    let selected_company = use_state(|| None);

    let on_company_select = {
        let selected_company = selected_company.clone();
        Callback::from(move |company: Company| {
            selected_company.set(Some(company))
        })
    };

    let company_details = selected_company.as_ref().map(|company| html! {
        <CompanyDetails company={company.clone()} />
    });

    html! {
        <>
            <div class="table-header-section">
              <span>
                <h2 class="table-topic jobs-button">{ "The Jobs" }</h2>
              </span>
              <span>
                <h2 class="table-topic companies-button">{ "The Companies" }</h2>
              </span>
            </div>

            if companies.is_empty() {
                <div class="loader companies-loader"></div>
            } else {
                <table class="company-list">
                <CompanyList companies={(*companies).clone()} on_click={on_company_select.clone()} />
                </table>
                { for company_details }
            }
        </>
    }
}