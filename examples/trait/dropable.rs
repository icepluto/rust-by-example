struct Dropable {
    name: &'static str,
}

impl Drop for Dropable {
    fn drop(&mut self) {
        println!("> droped name {}", self.name);
    }
}
fn main() {
    let _a = Dropable { name: "a" };
    {
        let _b = Dropable { name: "b" };

        {
            let _c = Dropable { name: "c" };
        }
    }
    let _d = Dropable { name: "d" };
    let _e = Dropable { name: "e" };
}
