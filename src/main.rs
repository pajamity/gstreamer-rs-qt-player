extern crate libc;

extern crate gstreamer as gst;
use gst::prelude::*;

extern crate gstreamer_sys;

extern crate glib;
use glib::translate::{ToGlib, FromGlib};

extern crate gobject_sys;

use std::sync::Mutex;

mod implementation;
use implementation::*;
// pub mod interface {
//   // include!(concat!(env!("OUT_DIR"), "/src/interface.rs"));
//   include!("./interface.rs");
// }

mod interface;
use interface::*;

// I couldn't avoid using global state...
// lazy_static (static Mutex) can't be used as *const usize "cannot be sent between threads safely" according to the compiler
static mut VIDEO_ITEM: *const usize = 0 as *const usize;

// functions in main.cpp
extern {
  fn main_cpp(app: *const ::std::os::raw::c_char) -> *const usize;
  fn set_widget_to_sink(sink: *const gstreamer_sys::GstElement, video_item: *const usize);
}

impl PlayerTrait for Player {
  fn new(emit: PlayerEmitter) -> Self {
    let (sink, playbin) = setup();
    unsafe {
      println!("Address of sink Rust gives C++: {:?}", sink.as_ptr());
      println!("Address of videoItem Rust gives C++: {:?}", VIDEO_ITEM);
      set_widget_to_sink(sink.as_ptr(), VIDEO_ITEM);
    }

    Self {
      emit,
      playbin,
      sink
    }
  }

  fn emit(&mut self) -> &mut PlayerEmitter {
    &mut self.emit
  }

  fn play(&mut self) {
    println!("Play");
    self.playbin
      .set_state(gst::State::Playing)
      .expect("could not change the state");
  }
  
  fn pause(&mut self) {
    println!("Pause");
    self.playbin
      .set_state(gst::State::Paused)
      .expect("could not change the state");
  }

  fn on_video_item_loaded(&self) {
    println!("GstGLVideoItem is loaded");
    unsafe {
      set_widget_to_sink(self.sink.as_ptr(), VIDEO_ITEM);
    }
  }
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

fn setup() -> (gst::Element, gst::Element) {
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

  (sink, playbin)
}

#[no_mangle]
pub unsafe fn set_video_item_pointer(video_item: *const usize) {
  VIDEO_ITEM = video_item;
  println!("Pointer for videoItem: {:?}", video_item);
}