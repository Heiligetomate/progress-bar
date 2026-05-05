use progress_bar::bar::{GetBar, OutputBar, ProgressBar};

struct MyTestStruct {
    val: u64,
    progress_bar: ProgressBar,
}

impl MyTestStruct {
    fn new(val: u64) -> Self {
        let progress_bar = ProgressBar::default();
        Self { val, progress_bar }
    }
}

impl GetBar for MyTestStruct {
    fn get_bar(&self) -> &ProgressBar {
        &self.progress_bar
    }
}

impl OutputBar for MyTestStruct {}

fn main() -> std::io::Result<()> {
    let foo = MyTestStruct::new(42);
    foo.output()
}
