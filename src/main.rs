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
  fn main_cpp(app: *const ::std::os::raw::c_char, sink: *const gstreamer_sys::GstElement);
}

// struct PlayerElements {
//   playbin: gst::Element,
//   sink: gst::Element,
// }

struct Player {
  emit: PlayerEmitter,
  playbin: gst::Element,
  sink: gst::Element,
}

impl PlayerTrait for Player {
  fn new(emit: PlayerEmitter) -> Self {
    Self {
      emit,
    }
  }

  fn emit(&mut self) -> &mut PlayerEmitter {
    &mut self.emit
  }

  fn pause(&mut self) -> () {

  }

  fn play(&mut self) -> () {

  }
}

impl Player {
  fn play(&self) {
    self.playbin
      .set_state(gst::State::Playing)
      .expect("could not change the state");
  }
  
  fn pause(&self) {
    self.playbin
      .set_state(gst::State::Paused)
      .expect("could not change the state");
  }
}

fn main() {
  // Load GStreamer plugin for qmlglsink beforehand
  gst::init().unwrap();
  let _ = gst::ElementFactory::make("qmlglsink", Some("qmlglsink"));

  // setup sink + playbin
  let player = setup();

  // Call Qt via FFI
  use std::ffi::CString;
  let app_name = ::std::env::args().next().unwrap();
  let app_name = CString::new(app_name).unwrap();
  println!("Address of sink Rust gives C++: {:?}", player.sink.as_ptr());
  unsafe {
    main_cpp(app_name.as_ptr(), player.sink.as_ptr());
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