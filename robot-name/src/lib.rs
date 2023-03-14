use std::collections::HashSet;
use std::ops::{DerefMut, Range};
use std::sync::{Once, Mutex};
use rand::{self, Rng, distributions::Uniform};

static CAPS: Range<u8> = ('A' as u8)..('Z' as u8 + 1);
static DIGITS: Range<u8> = ('0' as u8)..('9' as u8 + 1);

static USED_NAMES: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static ONCE: Once = Once::new();

pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        ONCE.call_once(|| *USED_NAMES.lock().unwrap() = Some(HashSet::new()));

        Self(Self::gen_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = Self::gen_name()
    }

    fn gen_name() -> String {
        loop {
            let name: String = rand::thread_rng()
                .sample_iter(Uniform::from(CAPS.clone()))
                .take(2)
                .chain(
                    rand::thread_rng()
                        .sample_iter(Uniform::from(DIGITS.clone()))
                        .take(3)
                )
                .map(char::from)
                .collect();

            match USED_NAMES.lock().unwrap().deref_mut() {
                Some(set) => if set.insert(name.clone()) { break name },
                None => ()
            }
        }
    }
}
