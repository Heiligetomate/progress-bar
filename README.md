### A progress bar lib written in rust for displaying progress bars 
*not really usable rn*


example: 
```rs
struct MyTestStruct {
    val: u64,
    progress_bar: ProgressBar,
}

impl MyTestStruct {
    fn calculate(&mut self, time_per_step: Duration) -> std::io::Result<()> {
        let goal_value = 100;
        while self.val <= goal_value {
            let progress = self.val as f32 / goal_value as f32;
            self.output(progress, true)?;
            // some calculations that actually take some time
            self.val += 1;
        }

        Ok(())
    }
}

impl OutputBar for MyTestStruct {
    fn get_bar(&mut self) -> &mut ProgressBar {
        &mut self.progress_bar
    }
}

fn main() -> std::io::Result<()> {
    let mut foo = MyTestStruct::new(0);
    foo.calculate()?;
    Ok(())
}
```

