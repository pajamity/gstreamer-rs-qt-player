extern crate libc;

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_sys;

extern crate glib;
use glib::translate::{ToGlib, FromGlib};

extern crate gobject_sys;

mod implementation;
pub mod interface {
    include!(concat!(env!("OUT_DIR"), "/src/interface.rs"));
}

// functions in main.cpp
extern {
    fn main_cpp(app: *const ::std::os::raw::c_char);
}

fn main() {
    // Load GStreamer plugin for qmlglsink beforehand
    gst::init().unwrap();
    let _ = gst::ElementFactory::make("qmlglsink", Some("qmlglsink"));

    // Call Qt via FFI
    use std::ffi::CString;
    let app_name = ::std::env::args().next().unwrap();
    let app_name = CString::new(app_name).unwrap();
    unsafe {
        main_cpp(app_name.as_ptr());
    }
}

extern crate glib_sys;
use glib_sys::{gboolean, gconstpointer, gpointer, GType};

#[no_mangle]
pub extern "C" fn set_qmlglsink_widget(widget: *mut usize) { // libc::uintptr_t == usize cannot be transformed into glib::Value
    println!("Rust: Raw pointer to QQuickItem: {:?}", widget);
    play(widget);
}

struct Player {
    playbin: gst::Element,
    sink: gst::Element,
}

fn play(widget: *mut usize) {
    let playbin = gst::ElementFactory::make("playbin", None).unwrap();
    playbin.set_property("uri", &glib::Value::from("file:///usr/share/big-buck-bunny_trailer.webm")).unwrap();
    let sink = gst::ElementFactory::make("qmlglsink", None).unwrap();
    let sinkbin = gst::ElementFactory::make("glsinkbin", None).unwrap();

    sinkbin
        .set_property("sink", &sink.to_value())
        .unwrap();
    playbin
        .set_property("video-sink", &sinkbin.to_value())
        .unwrap();
    // Didn't work: 
    // unsafe {
    //     sink
    //     // .set_property("widget", &widget.as_ref().unwrap().to_value())
    //     .set_property("widget", widget.to_value())
    //     .unwrap();
    // }
    unsafe {
        // Didn't work: could not convert raw pointer to gpointer
        // let widget_gpointer = glib::Value::from_type(gobject_sys::G_TYPE_POINTER::static_type());
        // gobject_sys::G_TYPE_POINTER::set_value(&mut ret, self);
        // sink
        //     // .set_property("widget", widget_gpointer)
        //     .set_property("widget", &widget.to_glib_none())
        //     .unwrap();

        // Didn't work: As C++ code does, put pointer to widget directly since glib-rs checks type strictly
        // (see definition of `fn set_property<'a, N: Into<&'a str>>`)
        // but this caused the segmentation fault.
        gobject_sys::g_object_set(
            sink.as_ptr() as *mut gobject_sys::GObject,
            b"widget\0".as_ptr() as *const _,
            widget as *const gobject_sys::GValue,
            None);
    }
    
    playbin
        .set_state(gst::State::Playing)
        .expect("could not change the state");
}