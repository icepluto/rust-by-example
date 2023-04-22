struct Years(u32);
struct Days(u32);
impl Years {
    fn to_days(&self) -> Days {
        Days(&self.0 * 365)
    }
}
impl Days {
    fn to_years(&self) -> Years {
        Years(&self.0 / 365)
    }
}
fn old_enough(o: &Years) -> bool {
    o.0 >= 18
}
fn main() {
    let age = Years(19);
    let days = age.to_days();
    println!("{}",old_enough(&age));
    println!("{}",old_enough(&days.to_years()));
}
