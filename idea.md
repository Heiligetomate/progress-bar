meow

```rs

#[derive(ProgressBar)]
struct Test {
    #[progress(max = 161)]
    val: u64,
    some_other_stuff: Foo,
}

impl Test {
    fn calculate(&mut self) -> u64 {
        while self.val >= 161 {
            val += 1;
            // some calculations
            self.update_bar().unwrap();
        }
    }
}

```


