use yew::prelude::*;
use crate::video::Video;

#[derive(Properties, PartialEq)]
pub(crate) struct VideosListProps {
    pub videos: Vec<Video>,
    pub on_click: Callback<Video>
}

#[function_component(VideosList)]
pub(crate) fn videos_list(VideosListProps{ videos, on_click }: &VideosListProps) -> Html {
     //clone on_click because it gets passed to every VideosList
    let on_click = on_click.clone();

    videos.iter().map(|video| {

        // build a CallBack to bind to the html element 
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();

            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        }; 

        html! {
            // bind the Callback to the html element
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}