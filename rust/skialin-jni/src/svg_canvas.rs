use jni::sys::{jboolean, jbyteArray, jfloat, jlong};
use jni::JNIEnv;

use skialin_core::{Rect, SVGCanvas, SVGCanvasFlags};

use crate::util::{borrow_mut, box_ptr, take_ptr};

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_SVGCanvasNative_nMake(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    convert_text_to_paths: jboolean,
    no_pretty_xml: jboolean,
    relative_path_encoding: jboolean,
) -> jlong {
    let flags = SVGCanvasFlags {
        convert_text_to_paths: convert_text_to_paths != 0,
        no_pretty_xml: no_pretty_xml != 0,
        relative_path_encoding: relative_path_encoding != 0,
    };
    box_ptr(SVGCanvas::new(Rect::new(left, top, right, bottom), flags))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGCanvasNative_nGetCanvas(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow_mut::<SVGCanvas>(ptr) }.canvas.as_raw() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGCanvasNative_nFinish(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jbyteArray {
    let svg_canvas = unsafe { take_ptr::<SVGCanvas>(ptr) };
    let data = svg_canvas.finish();
    let bytes = data.as_bytes();
    let array = env.new_byte_array(bytes.len() as i32).expect("new_byte_array");
    let signed: Vec<i8> = bytes.iter().map(|&b| b as i8).collect();
    env.set_byte_array_region(&array, 0, &signed).expect("set_byte_array_region");
    array.into_raw()
}
