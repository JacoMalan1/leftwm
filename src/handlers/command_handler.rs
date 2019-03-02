use super::*;
use crate::display_action::DisplayAction;
use crate::models::WindowHandle;

pub fn process(manager: &mut Manager, command: Command, val: Option<String>) -> bool {
    match command {
        Command::MoveToTag => {
            if let Some(tag) = val {
                if let Some(window) = manager.focused_window() {
                    window.clear_tags();
                    window.tag(tag);
                    return true;
                }
            }
            false
        }

        Command::GotoTag => {
            if let Some(tag) = val {
                goto_tag_handler::process(manager, tag)
            } else {
                false
            }
        }

        Command::Execute => {
            if let Some(cmd) = val {
                use std::process::Command;
                Command::new(&cmd).spawn();
                println!("{}", &cmd);
                false
            } else {
                println!("NO VAL");
                false
            }
        }
        Command::CloseWindow => {
            if let Some(window) = manager.focused_window() {
                let act = DisplayAction::KillWindow(window.handle.clone());
                manager.actions.push_back( act );
            }
            false
        },
        Command::SwapTags => false,
    }
}