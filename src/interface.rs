/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

use implementation::*;

pub struct PlayerQObject {}

pub struct PlayerEmitter {
    qobject: Arc<AtomicPtr<PlayerQObject>>,
}

unsafe impl Send for PlayerEmitter {}

impl PlayerEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> PlayerEmitter {
        PlayerEmitter {
            qobject: self.qobject.clone(),
        }
    }
    fn clear(&self) {
        let n: *const PlayerQObject = null();
        self.qobject.store(n as *mut PlayerQObject, Ordering::SeqCst);
    }
}

pub trait PlayerTrait {
    fn new(emit: PlayerEmitter) -> Self;
    fn emit(&mut self) -> &mut PlayerEmitter;
    fn on_video_item_loaded(&self) -> ();
    fn pause(&mut self) -> ();
    fn play(&mut self) -> ();
}

#[no_mangle]
pub extern "C" fn player_new(
    player: *mut PlayerQObject,
) -> *mut Player {
    let player_emit = PlayerEmitter {
        qobject: Arc::new(AtomicPtr::new(player)),
    };
    let d_player = Player::new(player_emit);
    Box::into_raw(Box::new(d_player))
}

#[no_mangle]
pub unsafe extern "C" fn player_free(ptr: *mut Player) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn player_on_video_item_loaded(ptr: *const Player) {
    let o = &*ptr;
    o.on_video_item_loaded()
}

#[no_mangle]
pub unsafe extern "C" fn player_pause(ptr: *mut Player) {
    let o = &mut *ptr;
    o.pause()
}

#[no_mangle]
pub unsafe extern "C" fn player_play(ptr: *mut Player) {
    let o = &mut *ptr;
    o.play()
}
