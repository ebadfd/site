use maud::{html, Markup};

pub fn yt_video(id: String) -> Markup {
    let url = format!(
        "https://www.youtube-nocookie.com/embed/{}?amp;controls=1&iv_load_policy=3",
        id
    );
    html! {
        div.yt-video-container {
            iframe src={(url)} frameborder="0"
                style=".ytp-watermark"
                title="YouTube video player"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                allowfullscreen{

                }
        }
    }
}
