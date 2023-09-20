mod app;
mod video;
mod video_details;
mod videos_list;

use app::App;

fn main() {
    let document = gloo::utils::document();
    let body = document.query_selector(".main").unwrap().unwrap();

    let yew_mount = document.create_element("div").unwrap();
    let classes = yew_mount.class_list();
    classes.add_1("yew-mount").unwrap();

    body.append_child(&yew_mount).unwrap();

    yew::Renderer::<App>::with_root(yew_mount).render();
}