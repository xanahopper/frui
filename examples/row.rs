//! This example shows usage of the [`Row`] widget and its different options.
//!
//! [`DebugContainer`] is used to visualize layout bounds of the [`Row`] widget.
//!
//! Feel free to modify each of the properties of the [`Row`] to see how it
//! affects the way its children are laid out.

#![feature(type_alias_impl_trait)]

use frui::prelude::*;

mod misc;
use misc::children_combinations::Big;

#[derive(ViewWidget)]
struct App;

impl ViewWidget for App {
    fn build<'w>(&'w self, _: BuildContext<'w, Self>) -> Self::Widget<'w> {
        DebugContainer::child(
            Row::builder()
                .main_axis_size(MainAxisSize::Max)
                .main_axis_alignment(MainAxisAlignment::Center)
                .cross_axis_alignment(CrossAxisAlignment::Center)
                .children((
                    Big(Color::rgb8(13, 245, 152)),
                    Big(Color::rgb8(255, 0, 110)),
                    Big(Color::rgb8(0, 186, 255)),
                )),
        )
    }
}

fn main() {
    run_app(App);
}

#[cfg(all(test, feature = "miri"))]
mod test {
    use super::*;
    use frui::{
        app::runner::miri::MiriRunner,
        druid_shell::{keyboard_types::Key, Modifiers},
    };

    #[test]
    pub fn run_example_under_miri() {
        let mut runner = MiriRunner::new(App);

        for _ in 0..4 {
            runner.key_down(KeyEvent::for_test(
                Modifiers::default(),
                Key::Character(" ".into()),
            ));
            runner.update(true);
        }
    }
}
