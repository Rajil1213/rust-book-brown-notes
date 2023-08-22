pub trait Draw {
    fn draw(&self) -> String;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) -> Vec<String> {
        let mut output = vec![];
        for component in self.components.iter() {
            output.push(component.draw());
        }

        output
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        String::from("drawing a button")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn draws_a_button() {
        let button = Button {
            width: 10,
            height: 12,
            label: String::from("Test Button"),
        };

        let screen = Screen {
            components: vec![Box::new(button)],
        };

        assert_eq!(vec![String::from("drawing a button")], screen.run());
    }

    #[test]
    fn draws_a_custom_component() {
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }

        impl Draw for SelectBox {
            fn draw(&self) -> String {
                format!(
                    "drawing a select box with height: {}, width: {}, options: {:?}",
                    self.height, self.width, self.options
                )
            }
        }

        let width = 10;
        let height = 12;
        let options = vec![String::from("option A"), String::from("option B")];
        let select_box = SelectBox {
            width,
            height,
            options: options.clone(),
        };

        let screen = Screen {
            components: vec![Box::new(select_box)],
        };

        assert_eq!(
            vec![format!(
                "drawing a select box with height: {}, width: {}, options: {:?}",
                height, width, options,
            )],
            screen.run()
        );
    }

    #[test]
    fn draws_two_different_components() {
        const WIDTH: u32 = 10;
        const HEIGHT: u32 = 12;
        let placeholder: String = String::from("placeholder text");

        struct TextField {
            width: u32,
            height: u32,
            placeholder: String,
        }

        impl Draw for TextField {
            fn draw(&self) -> String {
                format!(
                    "drawing a text field with width: {}, height: {}, placeholder: {}",
                    self.width, self.height, self.placeholder
                )
            }
        }

        let screen = Screen {
            components: vec![
                Box::new(Button {
                    width: WIDTH,
                    height: HEIGHT,
                    label: String::from("test button label"),
                }),
                Box::new(TextField {
                    width: WIDTH,
                    height: HEIGHT,
                    placeholder: placeholder.clone(),
                }),
            ],
        };

        assert_eq!(vec![
            String::from("drawing a button"),
            format!("drawing a text field with width: {WIDTH}, height: {HEIGHT}, placeholder: {placeholder}"),
        ], screen.run());
    }
}
