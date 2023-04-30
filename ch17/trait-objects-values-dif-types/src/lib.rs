pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/* Alternative implementation of Screen struct
 * This restricts us to a Screen instance that has a list of components all of type Button or all of
 * type TextField
 *
 * pub struct Screen {
 *     pub components: Vec<T>,
   }
 * 
 * impl<T> Screen<T>
 * where
 *     T: Draw,
 * {
 *      pub fn run(&self) {
 *          for component in self.components.iter() {
 *              component.draw();
 *          }
 *      }
 *  }
*/

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
