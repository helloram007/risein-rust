fn main() {
    let mut honda = ElectricCar { battery_level: 42 };
    let mut ford = GasCar { gas_level: 20 };

    honda.refuel(58);
    println!("{}", honda.battery_level);
}

trait Vehicle {
    type Fuel;

    fn refuel(&mut self, fuel: Self::Fuel);
}

struct ElectricCar {
    battery_level: u32,
}

struct GasCar {
    gas_level: u32,
}

impl  Vehicle for ElectricCar {
    type Fuel = u32;

    fn refuel(&mut self, charge: Self::Fuel) {
        self.battery_level += charge;
        println!("Battery charged to {}%", self.battery_level);
    }
}

impl Vehicle for GasCar {
    type Fuel = f32;

    fn refuel(&mut self, gas: Self::Fuel) {
        self.gas_level += (gas * 100.0) as u32;
        println!("Gas tank filled to {}%", self.gas_level);
    }
}