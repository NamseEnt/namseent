mod camera_controller {
    // globalThis.camera_controller = {
    use crate::game_state::mutate_game_state;
    use crate::*;
    struct KeyboardNav {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for KeyboardNav {
        // impl_for(KeyboardNav, ::core::clone::Clone, {
        #[inline]
        fn clone(&self) -> KeyboardNav {
            // clone(self) {
            let _: ::core::clone::AssertParamIsClone<bool>; // ignored
            *self // return _asterisk(self);
        } // },
    } // });
    #[automatically_derived]
    impl ::core::marker::Copy for KeyboardNav {} // impl_for(KeyboardNav, ::core::clone::Copy { });
    #[automatically_derived]
    impl ::core::default::Default for KeyboardNav {
        // impl_for(KeyboardNav, ::core::default::Default, {
        #[inline]
        fn default() -> KeyboardNav {
            // default() {
            KeyboardNav {
                // return
                up: ::core::default::Default::default(), // fn = ::core::default::Default::default, args = [], generic_args = [bool]
                down: ::core::default::Default::default(),
                left: ::core::default::Default::default(),
                right: ::core::default::Default::default(),
            }
        } // },
    } // });
    impl bincode::Encode for KeyboardNav {
        fn encode<__E: bincode::enc::Encoder>(
            &self,
            encoder: &mut __E,
        ) -> core::result::Result<(), bincode::error::EncodeError> {
            bincode::Encode::encode(&self.up, encoder)?;
            bincode::Encode::encode(&self.down, encoder)?;
            bincode::Encode::encode(&self.left, encoder)?;
            bincode::Encode::encode(&self.right, encoder)?;
            Ok(())
        }
    }
    impl bincode::Decode<()> for KeyboardNav {
        fn decode<__D: bincode::de::Decoder<Context = ()>>(
            decoder: &mut __D,
        ) -> core::result::Result<Self, bincode::error::DecodeError> {
            Ok(Self {
                up: bincode::Decode::decode(decoder)?,
                down: bincode::Decode::decode(decoder)?,
                left: bincode::Decode::decode(decoder)?,
                right: bincode::Decode::decode(decoder)?,
            })
        }
    }
    impl Serialize for KeyboardNav {
        fn serialize(&self, buf: &mut Vec<u8>) {
            buf.write_string(std::any::type_name::<Self>());
            self.serialize_without_name(buf);
        }
        fn serialize_without_name(&self, buf: &mut Vec<u8>) {
            buf.write_string("up");
            self.up.serialize_without_name(buf);
            buf.write_string("down");
            self.down.serialize_without_name(buf);
            buf.write_string("left");
            self.left.serialize_without_name(buf);
            buf.write_string("right");
            self.right.serialize_without_name(buf);
        }
    }
    impl Deserialize for KeyboardNav {
        fn deserialize(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            buf.read_name(std::any::type_name::<Self>())?;
            Self::deserialize_without_name(buf)
        }
        fn deserialize_without_name(buf: &mut &[u8]) -> Result<Self, DeserializeError> {
            let field_name = buf.read_name("up")?;
            let up = Deserialize::deserialize_without_name(buf)?;
            let field_name = buf.read_name("down")?;
            let down = Deserialize::deserialize_without_name(buf)?;
            let field_name = buf.read_name("left")?;
            let left = Deserialize::deserialize_without_name(buf)?;
            let field_name = buf.read_name("right")?;
            let right = Deserialize::deserialize_without_name(buf)?;
            Ok(Self {
                up,
                down,
                left,
                right,
            })
        }
    }
    pub struct CameraController;
    impl Component for CameraController {
        fn render(self, ctx: &RenderCtx) {
            let (keyboard_nav, set_keyboard_nav) = ctx.state(KeyboardNav::default);
            ctx.attach_event(move |event| match event {
                Event::KeyDown { event } => match event.code {
                    Code::KeyW => set_keyboard_nav.mutate(|nav| nav.up = true),
                    Code::KeyS => set_keyboard_nav.mutate(|nav| nav.down = true),
                    Code::KeyA => set_keyboard_nav.mutate(|nav| nav.left = true),
                    Code::KeyD => set_keyboard_nav.mutate(|nav| nav.right = true),
                    _ => {}
                },
                Event::KeyUp { event } => match event.code {
                    Code::KeyW => set_keyboard_nav.mutate(|nav| nav.up = false),
                    Code::KeyS => set_keyboard_nav.mutate(|nav| nav.down = false),
                    Code::KeyA => set_keyboard_nav.mutate(|nav| nav.left = false),
                    Code::KeyD => set_keyboard_nav.mutate(|nav| nav.right = false),
                    _ => {}
                },
                _ => {}
            });
            ctx.interval("camera move", Duration::from_millis(16), move |real_dt| {
                let nav = *keyboard_nav;
                if nav.up || nav.down || nav.left || nav.right {
                    let speed_px_per_sec: f32 = 1024.0;
                    let dt_secs = real_dt.as_millis() as f32 / 1000.0;
                    let step = (speed_px_per_sec * dt_secs).max(0.0);
                    let mut dir_x: f32 = 0.0;
                    let mut dir_y: f32 = 0.0;
                    if nav.left {
                        dir_x -= 1.0;
                    }
                    if nav.right {
                        dir_x += 1.0;
                    }
                    if nav.up {
                        dir_y -= 1.0;
                    }
                    if nav.down {
                        dir_y += 1.0;
                    }
                    let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
                    if len > 0.0 {
                        let nx = dir_x / len;
                        let ny = dir_y / len;
                        let dx = (nx * step).px();
                        let dy = (ny * step).px();
                        mutate_game_state(move |game_state| {
                            game_state.camera.move_by(Xy::new(dx, dy));
                        });
                    }
                }
            });
        }
    }
}
