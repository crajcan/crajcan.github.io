use yew::prelude::*;
use crate::video::Video;
use crate::video_details::VideoDetails;
use crate::videos_list::VideosList;

use crate::company::Company;
use crate::company_details::CompanyDetails;
use crate::company_list::CompanyList;

use gloo_net::http::Request;

const COMPANIES_URL: &str = "https://rust-jobs-api.fly.dev/companies/query";

#[function_component(App)]
pub fn app() -> Html {
    /////videos stuff///////

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json") .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        }, ());
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });
    /////end videos stuff///////
     
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
            <h2>{ "The Companies" }</h2>
            <CompanyList companies={(*companies).clone()} on_click={on_company_select.clone()} />
            { for company_details }

            <h2 style="border-top: 1px solid black;">{ "RustConf Explorer" }</h2>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}