use jni::sys::{jint, jlong, jstring};
use jni::JNIEnv;

use skialin_core::{FontMgr, FontStyle, Slant};

use crate::util::{borrow, box_ptr, drop_ptr};

fn slant_from_ordinal(ordinal: jint) -> Slant {
    match ordinal {
        1 => Slant::Italic,
        2 => Slant::Oblique,
        _ => Slant::Upright,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nSystem(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(FontMgr::system())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nEmpty(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(FontMgr::empty())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<FontMgr>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nCountFamilies(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<FontMgr>(ptr) }.count_families()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nFamilyName<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, index: jint) -> jstring {
    let name = unsafe { borrow::<FontMgr>(ptr) }.family_name(index);
    env.new_string(name).expect("new_string").into_raw()
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nMatchFamilyStyle<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    ptr: jlong,
    family_name: jni::objects::JString<'l>,
    weight: jint,
    width: jint,
    slant: jint,
) -> jlong {
    let name: Option<String> = if family_name.is_null() {
        None
    } else {
        Some(env.get_string(&family_name).expect("get_string").into())
    };
    let style = FontStyle::new(weight, width, slant_from_ordinal(slant));
    match unsafe { borrow::<FontMgr>(ptr) }.match_family_style(name.as_deref(), style) {
        Some(typeface) => box_ptr(typeface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nMakeFromData(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, data_ptr: jlong, ttc_index: jint) -> jlong {
    let data = unsafe { borrow::<skialin_core::Data>(data_ptr) };
    match unsafe { borrow::<FontMgr>(ptr) }.make_from_data(data, ttc_index) {
        Some(typeface) => box_ptr(typeface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontMgrNative_nMakeFromFile<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, path: jni::objects::JString<'l>, ttc_index: jint) -> jlong {
    let path: String = env.get_string(&path).expect("get_string").into();
    match unsafe { borrow::<FontMgr>(ptr) }.make_from_file(&path, ttc_index) {
        Some(typeface) => box_ptr(typeface),
        None => 0,
    }
}
