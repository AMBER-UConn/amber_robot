use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    rustodrive_gui::ui_test::ui_main();

    let mut signals = Signals::new(&[SIGINT])?;
    for sig in signals.forever() {
        println!("\nQuitting the program {:?}", sig);
        break;
    }

    println!("all done!");
    Ok(())
}
