```rust
use iced::widget::component::{Component, component};
use icedxdy::widget::{column, row, sidebar, text};

/// Which project is active is UI state, so a user `Component` keeps it; the
/// provider keeps whether the sidebar is open.
struct Projects;

impl<'a> Component<'a, Message, Theme> for Projects {
    type State = usize;
    type Event = usize;

    fn update(&self, selected: &mut usize, project: usize, _: &Renderer) -> Option<Message> {
        *selected = project;
        None
    }

    fn view(&self, selected: &usize) -> impl Widget<usize, Theme> + 'a {
        let selected = *selected;

        Element::from(sidebar::provider(move |s| {
            row![
                s.sidebar(sidebar::floating(sidebar::content([sidebar::group([
                    sidebar::group_label("Projects").into(),
                    sidebar::group_content([sidebar::menu(
                        ["Design Engineering", "Sales & Marketing", "Travel", "Support", "Feedback"]
                            .iter()
                            .enumerate()
                            .map(|(i, label)| {
                                sidebar::menu_item([sidebar::menu_button(*label)
                                    .active(i == selected)
                                    .on_press(i)
                                    .into()])
                                .into()
                            }),
                    )
                    .into()])
                    .into(),
                ])
                .into()]))),
                sidebar::main(column![sidebar::trigger(), text::muted("Press the trigger, or Cmd+B.")]),
            ]
            .into()
        }))
    }
}

component(Projects)
```
