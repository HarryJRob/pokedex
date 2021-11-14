use core::repositories::Repository;
use std::sync::Arc;

use super::{prompt_name, prompt_number, prompt_type};

pub fn run(repo: Arc<dyn Repository>) {
    let number = prompt_number();
    let name = prompt_name();
    let types = prompt_type();
}