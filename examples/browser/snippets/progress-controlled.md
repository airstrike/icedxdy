```rust
use iced::widget::component::{Component, component};
use icedxdy::widget::{column, progress, slider};

/// The value is UI state shared by two widgets, so the pair is a user
/// `Component` holding it.
struct Controlled;

impl<'a> Component<'a, Message, Theme> for Controlled {
    type State = Option<f32>;
    type Event = f32;

    fn update(&self, value: &mut Option<f32>, new: f32, _: &Renderer) -> Option<Message> {
        *value = Some(new);
        None
    }

    fn view(&self, value: &Option<f32>) -> impl Widget<f32, Theme> + 'a {
        let value = value.unwrap_or(50.0);

        Element::from(column![
            progress(value).width(320),
            slider(0.0..=100.0).value(value).on_value_change(|v| v).step(1.0),
        ])
    }
}

component(Controlled)
```
