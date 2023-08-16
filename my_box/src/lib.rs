use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping custom smart pointer with data `{}`", self.data);
    }
}

fn hello(name: &str) -> String {
    format!("Hello, {name}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ref() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(x, *y);
    }

    #[test]
    fn test_box_ref() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(x, *y);
    }

    #[test]
    fn test_mybox_ref() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(x, *y);
    }

    #[test]
    fn says_hello_to_my_box() {
        let m = MyBox::new(String::from("Rust"));
        assert_eq!("Hello, Rust", hello(&m));

        let n = &MyBox::new(String::from("again"));
        // implicit coercion
        assert_eq!("Hello, again", hello(n)); // n => &str
                                              // explicit coercion
        assert_eq!("Hello, again", hello(&(n)[..])); // &(n)[..] = &str
        assert_eq!("Hello, again", hello(&((*n).deref())[..])); // (*n).deref() = &String
    }

    #[test]
    fn drop_test() {
        let _c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let _d = CustomSmartPointer {
            data: String::from("my other stuff"),
        };

        println!("CustomSmartPointers Created");
    }
}
