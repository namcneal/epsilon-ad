mod epsilon_duals;

use crate::epsilon_duals::duals::*;

fn main() {
    let d = Dual::<f64>::from(1.0);
    println!("{:?}", d);
}
