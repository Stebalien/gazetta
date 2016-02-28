initSidebarItems({"struct":[["FnRenderer","A template renderer. The `html! {}` macro returns a `FnRenderer`."],["TemplateBuffer","A template buffer. This is the type that gets passed to closures inside templates.Example:"]],"trait":[["Render","Something that can be rendered by reference."],["RenderBox","Something that can be rendered once out of a box.This should only ever be used in the form `Box<RenderBox>` by casting `Box<RenderOnce>` to `Box<RenderBox>`. This trait has methods but I've hidden them because you should never call them directly.  Instead, you should call the `RenderOnce` methods implemented on `Box<RenderBox>`."],["RenderMut","Something that can be rendered by mutable reference."],["RenderOnce","Something that can be rendered once."],["Template","A template that can be rendered into something.Don't let the single impl below fool you, these methods are available on all `Render*`'s (through impls on references and boxes)."]]});