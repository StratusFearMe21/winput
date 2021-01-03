mod vk;
pub use vk::Vk;

mod input;
pub use input::{send_inputs, Action, Input, MouseMotion, WheelDirection};

mod keylike;
pub use keylike::{send_keys, send_str, Keylike};
