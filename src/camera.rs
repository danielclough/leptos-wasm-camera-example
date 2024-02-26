use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, HtmlVideoElement,
    MediaStreamConstraints,
};

pub fn open_camera(set_streaming: WriteSignal<bool>) -> () {
    leptos_dom::log!("Set Streaming");
    set_streaming.set(true);

    leptos_dom::log!("Open Camera");
    // show_view_live_result_button(set_streaming);
    let document = leptos_dom::document();
    let video = document
        .get_element_by_id("video")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()
        .unwrap();
    video.set_autoplay(true);

    let canvas = document
        .get_element_by_id("photo-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&JsValue::from(true));

    let promise = leptos_dom::window()
        .navigator()
        .media_devices()
        .unwrap()
        .get_user_media_with_constraints(&constraints)
        .unwrap();

    let width = 320;
    let height = 320;

    spawn_local(async move {
        leptos_dom::log!("Spawn Local");
        let stream = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
        video.set_src_object(Some(&stream.into()));

        _ = video.set_attribute("width", width.to_string().as_str());
        _ = video.set_attribute("height", height.to_string().as_str());
        _ = canvas.set_attribute("width", width.to_string().as_str());
        _ = canvas.set_attribute("height", height.to_string().as_str());
    });
}

pub fn take_picture(set_streaming: WriteSignal<bool>) {
    leptos_dom::log!("Take Picture");

    let document = leptos_dom::window().document().unwrap();

    let video = document
        .get_element_by_id("video")
        .unwrap()
        .dyn_into::<HtmlVideoElement>()
        .unwrap();

    let canvas = document
        .get_element_by_id("photo-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    let photo = document
        .get_element_by_id("photo")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    let width = 320;
    let height = 320;

    _ = context.draw_image_with_html_video_element_and_dw_and_dh(
        &video,
        0.0,
        0.0,
        width as f64,
        height as f64,
    );

    let data = canvas.to_data_url_with_type("image/png").unwrap();
    photo.set_src(&data);
    set_streaming.set(false);
}

#[component]
pub fn Photo() -> impl IntoView {
    let (streaming, set_streaming) = create_signal(false);

    view! {
        <div class="camera">
            <Show when=move || streaming.get()>
                <video id="video">Video stream not available.</video>
            </Show>
            <button
                id="startbutton"

                on:click=move |_| {
                    if !streaming.get() {
                        open_camera(set_streaming)
                    } else {
                        take_picture(set_streaming)
                    }
                }
            >

                {move || if streaming.get() { "Take Picture" } else { "Open Camera" }}
            </button>
        </div>

        <canvas style=if streaming.get() { "" } else { "display:none;" } id="photo-canvas"></canvas>
        <div class="output">
            <img id="photo" alt="The screen capture will appear in this box."/>
        </div>
    }
}
