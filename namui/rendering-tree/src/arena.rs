use bumpalo::Bump;
use std::cell::{RefCell, UnsafeCell};

struct RenderArena {
    bump: UnsafeCell<Bump>,
    drops: RefCell<Vec<(*mut u8, unsafe fn(*mut u8))>>,
}

thread_local! {
    static ARENA: RenderArena = RenderArena {
        bump: UnsafeCell::new(Bump::new()),
        drops: RefCell::new(Vec::new()),
    };
}

unsafe fn drop_in_place_as<T>(ptr: *mut u8) {
    unsafe { std::ptr::drop_in_place(ptr as *mut T) }
}

fn with_arena<R>(f: impl FnOnce(&'static RenderArena) -> R) -> R {
    ARENA.with(|arena| {
        // SAFETY: the thread-local arena lives for the whole thread lifetime,
        // so a `'static` view is valid as long as nothing reads an allocation
        // after `reset_render_arena` or sends it to another thread.
        let arena: &'static RenderArena = unsafe { &*(arena as *const RenderArena) };
        f(arena)
    })
}

/// Allocates a value in the per-thread frame render arena.
///
/// # Safety contract
/// The returned reference is only valid until the next [`reset_render_arena`]
/// call on the same thread. Callers must not read it across a frame boundary
/// or move it to another thread.
pub fn arena_alloc<T>(value: T) -> &'static T {
    with_arena(|arena| {
        let bump = unsafe { &*arena.bump.get() };
        let allocated: &'static mut T = bump.alloc(value);
        if std::mem::needs_drop::<T>() {
            arena
                .drops
                .borrow_mut()
                .push((allocated as *mut T as *mut u8, drop_in_place_as::<T>));
        }
        allocated
    })
}

/// Allocates a slice in the frame render arena. `T` must hold no `Drop`
/// payload of its own (only `RenderingTree` is used here).
pub fn arena_alloc_slice<T, I>(values: I) -> &'static [T]
where
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    debug_assert!(!std::mem::needs_drop::<T>());
    with_arena(|arena| {
        let bump = unsafe { &*arena.bump.get() };
        bump.alloc_slice_fill_iter(values)
    })
}

/// Drops every tracked arena value and resets the arena for the next frame.
///
/// # Safety contract
/// Every reference handed out by [`arena_alloc`] / [`arena_alloc_slice`] since
/// the previous reset becomes dangling. The caller guarantees nothing reads
/// them after this call.
pub fn reset_render_arena() {
    with_arena(|arena| {
        let drops = std::mem::take(&mut *arena.drops.borrow_mut());
        for (ptr, drop_fn) in drops {
            unsafe { drop_fn(ptr) }
        }
        unsafe {
            (*arena.bump.get()).reset();
        }
    })
}
