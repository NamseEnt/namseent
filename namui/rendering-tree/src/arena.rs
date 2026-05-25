use bumpalo::Bump;
use std::cell::{Cell, RefCell, UnsafeCell};
use std::thread::LocalKey;

type DropEntry = (*mut u8, unsafe fn(*mut u8));

struct RenderArena {
    bump: UnsafeCell<Bump>,
    drops: RefCell<Vec<DropEntry>>,
}

thread_local! {
    static ARENA: RenderArena = RenderArena {
        bump: UnsafeCell::new(Bump::new()),
        drops: RefCell::new(Vec::new()),
    };
    static SCOPE_ACTIVE: Cell<bool> = const { Cell::new(false) };
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

fn debug_assert_scope_active() {
    debug_assert!(
        SCOPE_ACTIVE.with(|s| s.get()),
        "arena_alloc/arena_alloc_slice called outside an arena scope. \
         Wrap the entry point with `swap_arena_slot`, or hold an `ArenaScopeGuard` \
         around the build (see `enter_arena_scope`)."
    );
}

/// RAII guard that marks the current thread as inside an arena scope.
///
/// While the guard is alive, `arena_alloc`/`arena_alloc_slice` are permitted.
/// On drop, the scope flag is cleared but the arena is **not** reset — any
/// `&'static` references handed out by allocs during the scope remain valid
/// until the next [`reset_render_arena`] call (typically performed by the next
/// scope's setup or by `swap_arena_slot`).
#[must_use = "ArenaScopeGuard clears the scope flag on drop"]
pub struct ArenaScopeGuard {
    _not_send: std::marker::PhantomData<*const ()>,
}

impl Drop for ArenaScopeGuard {
    fn drop(&mut self) {
        SCOPE_ACTIVE.with(|s| s.set(false));
    }
}

/// Opens an arena scope on the current thread. Nested scopes panic.
///
/// Pair this with a prior `reset_render_arena()` if the caller wants a fresh
/// arena for the scope; otherwise the new allocs share the existing arena
/// memory. The returned guard must be kept alive for the duration of the
/// allocations.
pub fn enter_arena_scope() -> ArenaScopeGuard {
    SCOPE_ACTIVE.with(|s| {
        assert!(!s.get(), "nested arena scope is not allowed");
        s.set(true);
    });
    ArenaScopeGuard {
        _not_send: std::marker::PhantomData,
    }
}

/// Allocates a value in the per-thread frame render arena.
///
/// # Safety contract
/// The returned reference is only valid until the next [`reset_render_arena`]
/// call on the same thread. Callers must not read it across a frame boundary
/// or move it to another thread.
pub fn arena_alloc<T>(value: T) -> &'static T {
    debug_assert_scope_active();
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
    debug_assert_scope_active();
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

/// Swaps the value held by a thread-local slot, replacing any arena-borrowing
/// payload safely:
///
/// 1. drop the slot's current value (releasing every `&'static` it held into
///    the arena),
/// 2. reset the arena (now no references remain pointing into it),
/// 3. open an arena scope and run `build` to produce a new value (which may
///    `arena_alloc` freely),
/// 4. store the new value back into the slot.
///
/// The arena is **not** reset on exit — the new value's arena references stay
/// valid until the next `swap_arena_slot` call on the same slot. This is the
/// pattern for receivers (e.g. the wasm drawer) that need to keep the most
/// recent decoded tree alive across cheap "redraw with last tree" calls.
pub fn swap_arena_slot<T>(slot: &'static LocalKey<RefCell<Option<T>>>, build: impl FnOnce() -> T) {
    slot.with(|cell| {
        *cell.borrow_mut() = None;
    });
    reset_render_arena();
    let _scope = enter_arena_scope();
    let new = build();
    slot.with(|cell| {
        *cell.borrow_mut() = Some(new);
    });
}
