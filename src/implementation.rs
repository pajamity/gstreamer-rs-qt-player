use interface::*;

// We need to create `Player` struct and implement `PlayerTrait` trait on it
// Create struct in main.rs since Qt part and Gstreamer part cannot share same data between separated structs (?)
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