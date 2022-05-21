use std::error::Error;

pub fn error_and_exit(error_message: &str, origional_message: &(dyn Error + 'static)) -> ! {
    println!(
        "{} '{}'. Please file an issue: {}/issues/new",
        error_message,
        origional_message.to_string(),
        env!("CARGO_PKG_REPOSITORY")
    );
    std::process::exit(-1)
}