/*fn main() {
    let mut a = 5;
    let b = &mut a;
    let mut c = 10;
    let d = &c;
    *d = 20;
}
*/

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger {
        pub fn new(messenger: &T, max: uszie) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            todo!();
        }
}

fn main() {
}
