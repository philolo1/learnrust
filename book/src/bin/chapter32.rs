pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button");
    }
}

fn main() {
    let screen = Screen {
        components: vec![Box::new(Button {
            width: 1,
            height: 1,
            label: String::from("Test")
        })]
    };

    screen.run();
}
