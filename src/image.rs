use crate::*;
pub struct Image {
    pub texture: Texture,
}

#[cfg(target_arch = "wasm32")]
mod image_web {
    use super::*;
    use js_sys;
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::JsFuture;
    use web_sys::*;

    #[wasm_bindgen(module = "/src/helpers.js")]
    extern "C" {
        fn loadImage(path: &str) -> js_sys::Promise;
    }
    pub async fn load_image(gl: &GL, path: &str) -> Result<Image, ()> {
        let path = path.to_owned();
        let image = JsFuture::from(loadImage(&path)).await.unwrap();
        let image: HtmlImageElement = image.dyn_into().unwrap();

        unsafe {
            let texture = gl.create_texture().unwrap();
            gl.bind_texture(TEXTURE_2D, Some(texture));

            gl.tex_image_2d_with_html_image(
                TEXTURE_2D,
                0, /* mip level */
                RGBA as i32,
                RGBA,
                UNSIGNED_BYTE,
                &image,
            );
            Ok(Image { texture })
        }
    }
}
#[cfg(target_arch = "wasm32")]
pub use image_web::*;

#[cfg(not(target_arch = "wasm32"))]
pub async fn load_image(gl: &GL, path: &str) -> Result<Image, ()> {
    unimplemented!()
}
