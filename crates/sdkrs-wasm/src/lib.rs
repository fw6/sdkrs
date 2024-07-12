// use sdkrs_utils::dynamic_image::DynamicImageBuilder;
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_dynamic_image_url(
    raw_url: &[u8],
    width: Option<u32>,
    height: Option<u32>,
    reset_png: Option<bool>,
) -> Result<JsString, JsError> {
    // let mut builder = DynamicImageBuilder::default();

    // if let Some(true) = reset_png {
    //     builder.reset_png(true);
    // }

    // if let Some(width) = width {
    //     builder.width(width);
    // }

    // if let Some(height) = height {
    //     builder.height(height);
    // }

    // let raw_url = String::from_utf8(raw_url.to_vec()).map_err(|e| JsError::new(&e.to_string()))?;
    // let dynamic_image = builder.raw_url(raw_url).build()?;
    // let final_url = dynamic_image.get_url()?;

    Ok("final_url".into())
}
