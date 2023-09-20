use yew::prelude::*;
use crate::video::Video;

#[derive(Properties, PartialEq)]
pub(crate) struct VideosDetailsProps {
    pub video: Video,
}

#[function_component(VideoDetails)]
pub(crate) fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}