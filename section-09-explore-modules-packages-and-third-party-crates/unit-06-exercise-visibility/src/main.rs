mod car_factory {
    pub(crate) fn build_car() {
        println!("Honk honk!");
    }
}

fn main() {
    car_factory::build_car();
}
