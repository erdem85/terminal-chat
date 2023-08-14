use crossbeam_channel::Sender;
use cursive::direction::Direction;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};
use cursive::{Cursive, View};

use crate::ui::config::CONFIG;
use crate::ui::init::init_app;
use crate::ui::util::UICommand;

pub fn show_username_dialog(siv: &mut Cursive, ui_tx: Sender<UICommand>, init_after: bool) {
    if let Some(ref mut username_dialog) = siv.find_name::<Dialog>("username_dialog") {
        username_dialog.take_focus(Direction::none()).unwrap();
        return;
    }

    siv.add_layer(
        Dialog::new()
            .title("set username")
            .content(
                EditView::new()
                    .content(CONFIG.lock().unwrap().username.clone().unwrap_or_else(|| {
                        gethostname::gethostname()
                            .to_string_lossy()
                            .split('.')
                            .next()
                            .unwrap_or("")
                            .to_string()
                    }))
                    .on_submit({
                        let ui_tx = ui_tx.clone();
                        move |siv, username| {
                            ui_tx
                                .try_send(UICommand::UpdateUsername(username.to_string()))
                                .unwrap();
                            siv.pop_layer();
                            if init_after {
                                init_app(siv, ui_tx.clone());
                            }
                        }
                    })
                    .with_name("username_input"),
            )
            .button("Save", move |siv| {
                let username = siv
                    .call_on_name("username_input", |input: &mut EditView| input.get_content())
                    .unwrap();
                ui_tx
                    .try_send(UICommand::UpdateUsername(username.to_string()))
                    .unwrap();
                siv.pop_layer();
                if init_after {
                    init_app(siv, ui_tx.clone());
                }
            })
            .with_name("username_dialog")
            .full_width()
            .max_width(48),
    );
}
