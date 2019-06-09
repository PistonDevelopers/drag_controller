#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

//! A drag controller

extern crate input;

use input::{
    GenericEvent,
    MouseButton,
};
use input::Button::Mouse;

/// Describes a drag
#[derive(Copy, Clone)]
pub enum Drag {
    /// When the drag is interrupted by something,
    /// for example when the window is defocused.
    /// By returning true, the drag will continue when
    /// the window retrieves focus.
    Interrupt,
    /// Starts the drag.
    Start(f64, f64),
    /// Moves the drag.
    Move(f64, f64),
    /// Ends the drag.
    End(f64, f64),
}

/// Controls dragging.
#[derive(Copy, Clone)]
pub struct DragController {
    /// Whether to drag or not.
    pub drag: bool,
    /// The current positon of dragging.
    pub pos: [f64; 2],
}

impl DragController {
    /// Creates a new drag controller.
    pub fn new() -> DragController {
        DragController {
            drag: false,
            pos: [0.0, 0.0],
        }
    }

    /// Handles event.
    ///
    /// Calls closure when events for dragging happen.
    /// If the drag event callback returns `false`, it will cancel dragging.
    pub fn event<E: GenericEvent, F>(&mut self, e: &E, mut f: F)
        where
            F: FnMut(Drag) -> bool
    {
        e.mouse_cursor(|pos| {
            self.pos = pos;
            if self.drag {
                self.drag = f(Drag::Move(pos[0], pos[1]));
            }
        });
        e.press(|button| {
            match button {
                Mouse(MouseButton::Left) => {
                    if !self.drag {
                        self.drag = f(Drag::Start(self.pos[0], self.pos[1]));
                    }
                }
                _ => {}
            }
        });

        // Rest of the event are only handled when dragging.
        if !self.drag { return; }

        e.release(|button| {
            match button {
                Mouse(MouseButton::Left) => {
                    if self.drag {
                        f(Drag::End(self.pos[0], self.pos[1]));
                    }
                    self.drag = false;
                }
                _ => {}
            }
        });
        e.focus(|focused| {
            if focused == false {
                self.drag = f(Drag::Interrupt);
            }
        });
    }
}

