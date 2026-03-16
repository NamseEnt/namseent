use crate::*;
use std::cell::RefCell;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(target_os = "wasi")]
unsafe extern "C" {
    fn _audio_play(audio_id: usize, playback_id: usize, repeat: bool);
    fn _audio_play_spatial(audio_id: usize, playback_id: usize, repeat: bool);
    fn _audio_playback_drop(playback_id: usize);
    fn _audio_playback_set_volume(playback_id: usize, volume: f32);
    fn _audio_playback_set_position(playback_id: usize, x: f32, y: f32, z: f32);
    fn _audio_set_listener_position(x: f32, y: f32, z: f32);
    fn _audio_set_volume(volume: f32);
}

// --- No-op stubs for non-WASI targets ---
#[cfg(not(target_os = "wasi"))]
unsafe fn _audio_play(_audio_id: usize, _playback_id: usize, _repeat: bool) {}
#[cfg(not(target_os = "wasi"))]
unsafe fn _audio_play_spatial(_audio_id: usize, _playback_id: usize, _repeat: bool) {}
#[cfg(not(target_os = "wasi"))]
unsafe fn _audio_playback_drop(_playback_id: usize) {}
#[cfg(not(target_os = "wasi"))]
unsafe fn _audio_playback_set_volume(_playback_id: usize, _volume: f32) {}
#[cfg(not(target_os = "wasi"))]
unsafe fn _audio_playback_set_position(_playback_id: usize, _x: f32, _y: f32, _z: f32) {}
#[cfg(not(target_os = "wasi"))]
unsafe fn _audio_set_listener_position(_x: f32, _y: f32, _z: f32) {}
#[cfg(not(target_os = "wasi"))]
unsafe fn _audio_set_volume(_volume: f32) {}

static NEXT_PLAYBACK_ID: AtomicUsize = AtomicUsize::new(1);
static NEXT_PLAYBACK_ID_ATOM: Atom<usize> = Atom::uninitialized();
static VOLUME: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0x3F800000);

fn next_playback_id() -> usize {
    NEXT_PLAYBACK_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Copy)]
pub struct AudioAsset {
    id: usize,
    duration: Duration,
}

impl AudioAsset {
    pub const fn new(id: usize, duration: Duration) -> Self {
        Self { id, duration }
    }

    pub const fn duration(&self) -> Duration {
        self.duration
    }

    pub fn play(&self) -> PlayHandle {
        let playback_id = next_playback_id();
        unsafe { _audio_play(self.id, playback_id, false) }
        PlayHandle { playback_id }
    }

    pub fn play_repeat(&self) -> PlayHandle {
        let playback_id = next_playback_id();
        unsafe { _audio_play(self.id, playback_id, true) }
        PlayHandle { playback_id }
    }
}

pub struct PlayHandle {
    playback_id: usize,
}

impl PlayHandle {
    pub fn set_volume(&self, volume: f32) {
        unsafe { _audio_playback_set_volume(self.playback_id, volume) }
    }
}

impl Drop for PlayHandle {
    fn drop(&mut self) {
        unsafe { _audio_playback_drop(self.playback_id) }
    }
}

pub fn set_volume(volume: f32) {
    VOLUME.store(volume.to_bits(), Ordering::Relaxed);
    unsafe { _audio_set_volume(volume) }
}

pub fn volume() -> f32 {
    f32::from_bits(VOLUME.load(Ordering::Relaxed))
}

#[derive(Clone, Copy)]
pub struct AudioGroupSettings {
    pub volume: f32,
    pub z: f32,
}

impl Default for AudioGroupSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            z: 0.0,
        }
    }
}

thread_local! {
    static AUDIO_GROUP_STACK: RefCell<Vec<AudioGroupSettings>> = const { RefCell::new(Vec::new()) };
}

fn accumulated_audio_group() -> (f32, f32) {
    AUDIO_GROUP_STACK.with(|stack| {
        let stack = stack.borrow();
        let mut volume = 1.0;
        let mut z = 0.0;
        for entry in stack.iter() {
            volume *= entry.volume;
            z += entry.z;
        }
        (volume, z)
    })
}

pub struct AudioGroup<F: FnOnce(ComposeCtx)> {
    pub volume: f32,
    pub z: f32,
    pub children: F,
}

impl<F: FnOnce(ComposeCtx)> Component for AudioGroup<F> {
    fn render(self, ctx: &RenderCtx) {
        AUDIO_GROUP_STACK.with(|s| {
            s.borrow_mut().push(AudioGroupSettings {
                volume: self.volume,
                z: self.z,
            })
        });
        ctx.compose(self.children);
        AUDIO_GROUP_STACK.with(|s| s.borrow_mut().pop());
    }
}

struct SourceUpdate {
    playback_id: usize,
    position: Option<(f32, f32, f32)>,
    volume: f32,
}

struct AudioFrame {
    plays: Vec<(usize, usize, bool, bool)>,
    stops: Vec<usize>,
    source_updates: Vec<SourceUpdate>,
    listener_position: Option<(f32, f32, f32)>,
}

impl AudioFrame {
    fn new() -> Self {
        Self {
            plays: Vec::new(),
            stops: Vec::new(),
            source_updates: Vec::new(),
            listener_position: None,
        }
    }
}

thread_local! {
    static AUDIO_FRAME: RefCell<AudioFrame> = RefCell::new(AudioFrame::new());
}

fn push_play(asset_id: usize, playback_id: usize, repeat: bool, spatial: bool) {
    AUDIO_FRAME.with(|f| {
        f.borrow_mut()
            .plays
            .push((asset_id, playback_id, repeat, spatial));
    });
}

fn push_stop(playback_id: usize) {
    AUDIO_FRAME.with(|f| {
        f.borrow_mut().stops.push(playback_id);
    });
}

fn push_source_update(playback_id: usize, position: Option<(f32, f32, f32)>, volume: f32) {
    AUDIO_FRAME.with(|f| {
        f.borrow_mut().source_updates.push(SourceUpdate {
            playback_id,
            position,
            volume,
        });
    });
}

fn push_listener_position(x: f32, y: f32, z: f32) {
    AUDIO_FRAME.with(|f| {
        f.borrow_mut().listener_position = Some((x, y, z));
    });
}

pub fn flush_audio() {
    AUDIO_FRAME.with(|f| {
        let mut frame = f.borrow_mut();

        for (asset_id, playback_id, repeat, spatial) in frame.plays.drain(..) {
            if spatial {
                unsafe { _audio_play_spatial(asset_id, playback_id, repeat) }
            } else {
                unsafe { _audio_play(asset_id, playback_id, repeat) }
            }
        }

        for playback_id in frame.stops.drain(..) {
            unsafe { _audio_playback_drop(playback_id) }
        }

        for update in frame.source_updates.drain(..) {
            if let Some((x, y, z)) = update.position {
                unsafe { _audio_playback_set_position(update.playback_id, x, y, z) }
            }
            unsafe { _audio_playback_set_volume(update.playback_id, update.volume) }
        }

        if let Some((x, y, z)) = frame.listener_position.take() {
            unsafe { _audio_set_listener_position(x, y, z) }
        }
    });
}

pub struct Audio {
    pub asset: AudioAsset,
    pub repeat: bool,
    pub spatial: bool,
}

impl Component for Audio {
    fn render(self, ctx: &RenderCtx) {
        let (playback_id, set_playback_id) = ctx.state(|| 0usize);
        let (next_id_sig, _) = ctx.init_atom(&NEXT_PLAYBACK_ID_ATOM, || {
            NEXT_PLAYBACK_ID.load(Ordering::Relaxed)
        });

        let atom_val = *next_id_sig;
        let atomic_val = NEXT_PLAYBACK_ID.load(Ordering::Relaxed);
        if atom_val > atomic_val {
            NEXT_PLAYBACK_ID.store(atom_val, Ordering::Relaxed);
        }

        let restored_id = *playback_id;

        ctx.effect("audio", || {
            let id = if restored_id != 0 {
                restored_id
            } else {
                let id = next_playback_id();
                NEXT_PLAYBACK_ID_ATOM.set(NEXT_PLAYBACK_ID.load(Ordering::Relaxed));
                set_playback_id.set(id);
                push_play(self.asset.id, id, self.repeat, self.spatial);
                id
            };
            move || {
                push_stop(id);
            }
        });

        let current_id = if restored_id != 0 {
            restored_id
        } else {
            *playback_id
        };
        if current_id != 0 {
            let (group_volume, group_z) = accumulated_audio_group();

            if self.spatial {
                let matrix = ctx.accumulated_matrix();
                push_source_update(
                    current_id,
                    Some((matrix.x(), matrix.y(), group_z)),
                    group_volume,
                );
            } else {
                push_source_update(current_id, None, group_volume);
            }
        }
    }
}

pub struct AudioListener;

impl Component for AudioListener {
    fn render(self, ctx: &RenderCtx) {
        let matrix = ctx.accumulated_matrix();
        let (_, group_z) = accumulated_audio_group();
        push_listener_position(matrix.x(), matrix.y(), group_z);
    }
}
