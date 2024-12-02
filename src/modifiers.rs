//! Modifier key data.
//!
//! Modifier keys like Shift and Control alter the character value
//! and are used in keyboard shortcuts.
//!
//! Use the constants to match for combinations of the modifier keys.
#[cfg(doc)]
use crate::NamedKey;

bitflags::bitflags! {
    /// Currently pressed modifier keys.
    ///
    /// Each flag represents a modifier and can be set if this modifier is active.
    ///
    /// Specification:
    /// <https://w3c.github.io/uievents-key/#keys-modifier>
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Modifiers: u32 {
        /// The <kbd>Alt</kbd> or <kbd>Option</kbd> key.
        ///
        /// Corresponds to [`NamedKey::Alt`].
        const ALT = 0x01;
        /// The Alternate Graphics key.
        ///
        /// Corresponds to [`NamedKey::AltGraph`].
        const ALT_GRAPH = 0x2;
        /// The <kbd>Caps Lock</kbd> key.
        ///
        /// Corresponds to [`NamedKey::Alt`].
        const CAPS_LOCK = 0x4;
        /// The <kbd>Control</kbd> or <kbd>Ctrl</kbd> key.
        ///
        /// Corresponds to [`NamedKey::Control`].
        const CONTROL = 0x8;
        /// The Function switch key.
        ///
        /// Corresponds to [`NamedKey::Fn`].
        const FN = 0x10;
        const FN_LOCK = 0x20;
        /// This is the "windows" key on PC and "command" key on Mac.
        const META = 0x40;
        const NUM_LOCK = 0x80;
        const SCROLL_LOCK = 0x100;
        /// The <kbd>Shift</kbd> key.
        const SHIFT = 0x200;
        const SYMBOL = 0x400;
        const SYMBOL_LOCK = 0x800;
        /// Legacy.
        const HYPER = 0x1000;
        /// Legacy.
        const SUPER = 0x2000;
    }
}

impl Modifiers {
    /// Return `true` if a shift key is pressed.
    pub fn shift(&self) -> bool {
        self.contains(Modifiers::SHIFT)
    }

    /// Return `true` if a control key is pressed.
    pub fn ctrl(&self) -> bool {
        self.contains(Modifiers::CONTROL)
    }

    /// Return `true` if an alt key is pressed.
    pub fn alt(&self) -> bool {
        self.contains(Modifiers::ALT)
    }

    /// Return `true` if a meta key is pressed.
    pub fn meta(&self) -> bool {
        self.contains(Modifiers::META)
    }
}
