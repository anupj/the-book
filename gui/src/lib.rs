pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
   pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
&            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("The width is {}, the height is {}, 
                  the label is {}", self.width, self.height, self.label);
        

    }
}

