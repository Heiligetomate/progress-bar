use progress_bar::bar::{OutputBar, ProgressBar};
use std::io::Result;
use std::{thread::sleep, time::Duration};

struct MyTestStruct {
    val: u64,
    progress_bar: ProgressBar,
}

impl MyTestStruct {
    fn new(val: u64) -> Self {
        let progress_bar = ProgressBar::default();
        Self { val, progress_bar }
    }
    fn calculate(&mut self, time_per_step: Duration) -> Result<()> {
        let goal_value = 100;
        while self.val < goal_value {
            let progress = self.val as f32 / goal_value as f32;
            self.output(progress)?;
            sleep(time_per_step);
            self.val += 1;
        }

        Ok(())
    }
}

impl OutputBar for MyTestStruct {
    fn get_bar(&self) -> &ProgressBar {
        &self.progress_bar
    }
}

fn main() -> std::io::Result<()> {
    let mut foo = MyTestStruct::new(0);
    foo.calculate(Duration::from_millis(12))
}
