use keyberon::action::{k, Action, Action::*, HoldTapAction, HoldTapConfig};
// Import KeyCode as a *type* separately from the glob that brings variants into scope.
// Without this, `[KeyCode; 2]` in static array types fails because the glob import
// shadows the type name with the last variant it resolves to.
use keyberon::key_code::KeyCode;
use keyberon::key_code::KeyCode::*;

use crate::{NUM_COLS, NUM_ROWS, NUM_LAYERS};

// NOTE: Ensure NUM_LAYERS is set to 5 in main.rs / lib.rs.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomActions {
    Bootloader,
}

#[allow(dead_code)]
const BOOTLOADER: Action<CustomActions> = Action::Custom(CustomActions::Bootloader);

// ── Shifted-key statics (brackets layer) ──────────────────────────────────
//   Characters that have no direct HID keycode and require Shift+key.
//   Statics are typed as &[KeyCode] (not [KeyCode; N]) so that &KC_* produces
//   the &&[KeyCode] double-reference that MultipleKeyCodes requires.
// MultipleKeyCodes(&'static &'static [KeyCode]) requires a double reference:
// the static must itself be a &[KeyCode] slice, so that &KC_* yields &&[KeyCode].
static KC_LBRC: &[KeyCode] = &[LShift, LBracket]; // {
static KC_RBRC: &[KeyCode] = &[LShift, RBracket]; // }
static KC_LPAR: &[KeyCode] = &[LShift, Kb9];      // (
static KC_RPAR: &[KeyCode] = &[LShift, Kb0];      // )
static KC_DQT:  &[KeyCode] = &[LShift, Quote];    // "

// ── Shifted numpad / symbol statics (layer 4) ─────────────────────────────
//   These map the numpad layout to its shifted symbols so the firmware sends
//   a single keycode rather than requiring the user to hold Shift themselves
//   (avoiding 3-key ghosting on the matrix).
//   Note: ( and ) reuse KC_LPAR / KC_RPAR already defined above.
static KC_EXCL: &[KeyCode] = &[LShift, Kb1];   // !
static KC_AT:   &[KeyCode] = &[LShift, Kb2];   // @
static KC_HASH: &[KeyCode] = &[LShift, Kb3];   // #
static KC_DLR:  &[KeyCode] = &[LShift, Kb4];   // $
static KC_PERC: &[KeyCode] = &[LShift, Kb5];   // %
static KC_CIRC: &[KeyCode] = &[LShift, Kb6];   // ^
static KC_AMP:  &[KeyCode] = &[LShift, Kb7];   // &
static KC_ASTR: &[KeyCode] = &[LShift, Kb8];   // *
static KC_UNDR: &[KeyCode] = &[LShift, Minus]; // _
static KC_PLUS: &[KeyCode] = &[LShift, Equal]; // +

// ── Home row mod statics ───────────────────────────────────────────────────
//   tap_hold_interval mirrors ZMK's quick-tap-ms = 150:
//   if you tap and re-press within 150 ms the tap action fires again
//   instead of the hold action, making repeated taps fast and comfortable.
//
//   Left hand  (pinky → index): LAlt / LCtrl / LShift / LGui
//   Right hand (index → pinky): LGui / LShift / LCtrl / LAlt
static S_A:    HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LAlt),   tap: k(A)      };
static S_S:    HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LCtrl),  tap: k(S)      };
static S_D:    HoldTapAction<CustomActions> = HoldTapAction { timeout: 200, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LShift), tap: k(D)      };
static S_F:    HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LGui),   tap: k(F)      };
static S_J:    HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LGui),   tap: k(J)      };
static S_K:    HoldTapAction<CustomActions> = HoldTapAction { timeout: 200, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LShift), tap: k(K)      };
static S_L:    HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LCtrl),  tap: k(L)      };
static S_SEMI: HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: k(LAlt),   tap: k(SColon) };

// ── Layer-tap statics ──────────────────────────────────────────────────────
static S_LT1_W: HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: Action::Layer(1), tap: k(W) };
static S_LT2_V: HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: Action::Layer(2), tap: k(V) };
static S_LT3_M: HoldTapAction<CustomActions> = HoldTapAction { timeout: 300, tap_hold_interval: 150, config: HoldTapConfig::PermissiveHold, hold: Action::Layer(3), tap: k(M) };

// ── Action aliases ─────────────────────────────────────────────────────────
const HRM_A:    Action<CustomActions> = HoldTap(&S_A);
const HRM_S:    Action<CustomActions> = HoldTap(&S_S);
const HRM_D:    Action<CustomActions> = HoldTap(&S_D);
const HRM_F:    Action<CustomActions> = HoldTap(&S_F);
const HRM_J:    Action<CustomActions> = HoldTap(&S_J);
const HRM_K:    Action<CustomActions> = HoldTap(&S_K);
const HRM_L:    Action<CustomActions> = HoldTap(&S_L);
const HRM_SEMI: Action<CustomActions> = HoldTap(&S_SEMI);
const LT1_W:    Action<CustomActions> = HoldTap(&S_LT1_W);
const LT2_V:    Action<CustomActions> = HoldTap(&S_LT2_V);
const LT3_M:    Action<CustomActions> = HoldTap(&S_LT3_M);

const LBRC: Action<CustomActions> = Action::MultipleKeyCodes(&KC_LBRC);
const RBRC: Action<CustomActions> = Action::MultipleKeyCodes(&KC_RBRC);
const LPAR: Action<CustomActions> = Action::MultipleKeyCodes(&KC_LPAR);
const RPAR: Action<CustomActions> = Action::MultipleKeyCodes(&KC_RPAR);
const DQT:  Action<CustomActions> = Action::MultipleKeyCodes(&KC_DQT);

const EXCL: Action<CustomActions> = Action::MultipleKeyCodes(&KC_EXCL);
const AT:   Action<CustomActions> = Action::MultipleKeyCodes(&KC_AT);
const HASH: Action<CustomActions> = Action::MultipleKeyCodes(&KC_HASH);
const DLR:  Action<CustomActions> = Action::MultipleKeyCodes(&KC_DLR);
const PERC: Action<CustomActions> = Action::MultipleKeyCodes(&KC_PERC);
const CIRC: Action<CustomActions> = Action::MultipleKeyCodes(&KC_CIRC);
const AMP:  Action<CustomActions> = Action::MultipleKeyCodes(&KC_AMP);
const ASTR: Action<CustomActions> = Action::MultipleKeyCodes(&KC_ASTR);
const UNDR: Action<CustomActions> = Action::MultipleKeyCodes(&KC_UNDR);
const PLUS: Action<CustomActions> = Action::MultipleKeyCodes(&KC_PLUS);

// ── Keymap ─────────────────────────────────────────────────────────────────
//
//   Physical layout (keezyboost40, 4×10):
//
//   Row 0  Q    W    E    R    T  │  Y    U    I    O    P
//   Row 1  A    S    D    F    G  │  H    J    K    L    ;
//   Row 2  Z    X    C    V    B  │  N    M    ,    .    /
//   Row 3  ___  ___  ___  ×    ×  │  ×    ×    ___  ___  ___
//                         │    │     │    │
//                        TAB  ESC  SPC  BSP   ← thumb cluster
//
//   ___ = NoOp (outer 3 keys each side)
//   ×   = inner 2 keys each side, mirrors Ferris Sweep thumb cluster
//
//   Layer map:
//     0 · Default  — QWERTY + home row mods + layer-taps
//     1 · Numpad   — right-hand numpad, left-hand mod passthrough
//     2 · Arrows   — right-hand nav + media; Delete replaces Backspace
//     3 · Brackets — left-hand all paired delimiters
//     4 · Sym-pad  — numpad layout with shifted symbols (avoids 3-key ghosting)
//
#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<NUM_COLS, NUM_ROWS, NUM_LAYERS, CustomActions> = [

    // ── Layer 0 · Default ─────────────────────────────────────────────────
    [
        //  0        1        2        3          4          5          6           7        8        9
        [k(Q),    LT1_W,   k(E),    k(R),      k(T),      k(Y),      k(U),      k(I),    k(O),    k(P)    ],
        [HRM_A,   HRM_S,   HRM_D,   HRM_F,     k(G),      k(H),      HRM_J,     HRM_K,   HRM_L,   HRM_SEMI],
        [k(Z),    k(X),    k(C),    LT2_V,     k(B),      k(N),      LT3_M,     k(Comma),k(Dot),  k(Slash)],
        [NoOp,    NoOp,    NoOp,    k(Tab),    k(Escape), k(Space),  k(BSpace), NoOp,    NoOp,    NoOp    ],
    ],

    // ── Layer 1 · Numpad ──────────────────────────────────────────────────
    [
        //  0          1          2           3       4       5       6       7       8          9
        [NoOp,      NoOp,      NoOp,       NoOp,   NoOp,   NoOp,   k(Kb7), k(Kb8), k(Kb9),    k(Minus)],
        [k(LAlt),   k(LCtrl),  k(LShift),  k(LGui),NoOp,   NoOp,   k(Kb4), k(Kb5), k(Kb6),    k(Equal)],
        [NoOp,      NoOp,      NoOp,       NoOp,   NoOp,   NoOp,   k(Kb1), k(Kb2), k(Kb3),    NoOp   ],
        [NoOp,      NoOp,      NoOp,       NoOp,   NoOp,   k(Kb0), NoOp,   NoOp,   NoOp,      NoOp    ],
    ],

    // ── Layer 2 · Arrows + Media ──────────────────────────────────────────
    // Media names confirmed from compiler suggestion: VolDown, VolUp.
    // Mute and MediaPlayPause are inferred — if either fails, check
    // firmware/keyberon/src/key_code.rs for the exact variant spellings.
    [
        //  0          1          2           3       4          5           6          7        8         9
        [NoOp,      NoOp,      NoOp,       NoOp,   NoOp,      k(Home),    k(PgDown), k(PgUp), k(End),   NoOp],
        [k(LAlt),   k(LCtrl),  k(LShift),  k(LGui),NoOp,      k(Left),    k(Down),   k(Up),   k(Right), NoOp],
        [NoOp,      NoOp,      NoOp,       NoOp,   NoOp,      k(Mute),    k(VolDown), k(VolUp), k(MediaPlayPause), NoOp],
        [NoOp,      NoOp,      NoOp,       NoOp,   NoOp,      k(Enter),   k(Delete), NoOp,    NoOp,     NoOp ],
    ],

    // ── Layer 3 · Brackets ────────────────────────────────────────────────
    [
        //  0            1            2      3      4       5       6           7           8           9
        [k(LBracket), k(RBracket), LBRC,  RBRC,  NoOp,   NoOp,   NoOp,       NoOp,       NoOp,       NoOp  ],
        [DQT,         k(Quote),    LPAR,  RPAR,  NoOp,   NoOp,   k(LGui),    k(LShift),  k(LCtrl),   k(LAlt)],
        [k(Grave),    NoOp,        k(Bslash), NoOp,  NoOp,  NoOp,  NoOp,     NoOp,       NoOp,       Trans  ],
        [NoOp,        NoOp,        NoOp,  NoOp,  NoOp,   NoOp,   NoOp,       NoOp,       NoOp,       NoOp   ],
    ],
];
