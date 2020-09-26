#[derive(Debug)]
pub struct AcStyler {
    pub matchers: Vec<&'static str>,
    pub style: console::Style,
}

impl AcStyler {
    pub fn style(&self, input: &mut String) {
        for m in self.matchers.iter() {
            if input.contains(m) {
                *input = input.replace(m, &self.style.apply_to(m).to_string());
            }
        }
    }
}
