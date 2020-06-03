use interface::*;

extern crate gstreamer as gst;

use std::sync::{Arc};

// We need to create `Player` struct and implement `PlayerTrait` trait on it
// Put Qt-related properties and GStreamer-related properties in the same struct 

// this struct is referred from both of main.rs and interface.rs so is put here
pub struct Player {
  pub emit: PlayerEmitter,
  pub playbin: gst::Element,
  pub sink: Arc<gst::Element>,
}

// struct Player {
//   emit: PlayerEmitter,
  
// }

// impl PlayerTrait for Player {
//   fn new(emit: PlayerEmitter) -> Self {
//     Self {
//       emit,
//     }
//   }

//   fn emit(&mut self) -> &mut PlayerEmitter {
//     &mut self.emit
//   }

//   fn pause(&mut self) -> () {

//   }

//   fn play(&mut self) -> () {

//   }
// }