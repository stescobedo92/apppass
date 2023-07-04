use std::fs::*;
use std::io::*;
use rand::{thread_rng, Rng};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use rand::distributions::Alphanumeric;

const FILE_NAME: &str = "passwords";