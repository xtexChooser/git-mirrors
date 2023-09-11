use peerd::{
    action::{apply::apply, daemon::daemon},
    args::{Action, ARGS},
};

fn main() {
    match ARGS.action {
        Action::Daemon => daemon(),
        Action::Apply => apply(),
        Action::Show => {}
    }
}
