use jni::sys::jlong;
use jni::JNIEnv;

use skialin_core::{Typeface, TypefaceFontProvider};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceFontProviderNative_nNew(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(TypefaceFontProvider::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceFontProviderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<TypefaceFontProvider>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceFontProviderNative_nRegisterTypeface(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, typeface_ptr: jlong) -> jlong {
    let typeface = unsafe { borrow::<Typeface>(typeface_ptr) };
    unsafe { borrow_mut::<TypefaceFontProvider>(ptr) }.register_typeface(typeface) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceFontProviderNative_nRegisterTypefaceAlias<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    ptr: jlong,
    typeface_ptr: jlong,
    alias: jni::objects::JString<'l>,
) -> jlong {
    let alias: String = env.get_string(&alias).expect("get_string").into();
    let typeface = unsafe { borrow::<Typeface>(typeface_ptr) };
    unsafe { borrow_mut::<TypefaceFontProvider>(ptr) }.register_typeface_with_alias(typeface, &alias) as jlong
}
