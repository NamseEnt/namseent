# Hooks v2

This is v2, an improved version based on the experience of creating multiple Hooks systems.

# Core system of Hooks v2

## Sig (Signal)

Hooks v2 uses Signals, just like Solid.js's Signal.

Since "Signal" sounds a bit long, we shorten it to "Sig." This is similar to how "Vector" is abbreviated as "Vec" and "Reference" as "Ref."

In v1, we had to check for updates using PartialEq of the data. We also had to store the data for comparison. For example, use_effect required listing the dependencies directly. While this may seem explicit, it is error-prone and not very good for Developer eXperience (DX).

In v2, Sig checks if a change has been attempted and updates the data based on that. Since no comparison is performed, PartialEq is no longer needed. Sig pays attention to the timing of Deref. For example, in use_effect, if you Deref a Sig within the effect, it recognizes that the effect is using that Sig and stores that information (SigId).

You can use set_state to change a Sig. When a Sig is changed, the system performs a complete re-render. It remembers the Id of the changed Sig and executes the hooks related to it. Hooks that don't use the changed Sig won't be executed.

For example:

```rust
let (a, set_a) = use_state(|| 0);

use_effect("When 'a' changed", || {
    println!("a: {}", *a);  // <- Because 'a' is Deref(*)ed, this use_effect saves that it used the Sig 'a'.
});                         //    So, if 'a' is changed via set_a, this effect will re-run.
                            //    If 'a' is not changed, this effect won't run.
```

## One Event per Component!

You can connect events that a component should handle. In such cases, when rendering, the EventContext that can create an EventCallback is passed as a parameter.

EventCallbacks are very lightweight and Cloneable, so it's safe to pass them deeply to children and grandchildren.

EventCallbacks contain the Component Id. Thus, during re-rendering, the event handler of the target component is executed based on this information.

```rust
enum Event {
    Hello
}

impl Component for Mycomponent {
    fn render(&self) -> RenderDone {
        // ...

        use_render_with_event(
            |event: Event| {
                match event {
                    Event::Hello => println!("World!"),
                }
            },
            |ctx| {
                Button {
                    on_click: ctx.event(Event::Hello),
                },
            },
        )
    }
}
```

If there are no events to connect to the component, you don't have to connect them!

```rust
impl Component for Mycomponent {
    fn render(&self) -> RenderDone {
        let text = "I don't have event!";
        use_render(
            |ctx| {
                Text {
                    text: text.as_sig()
                },
            },
        )
    }
}
```

## Always Rerender

Hooks v2 does not have partial re-rendering functionality. The reason is simple: it allows for simpler and safer code. Although optimizations might be possible in the future, we cannot guarantee that partial re-rendering will ever be present. For now, the focus is on maintaining the simplicity of the current code structure, as long as there are no severe performance issues.
