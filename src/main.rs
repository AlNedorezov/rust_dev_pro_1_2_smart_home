use std::fmt;

#[derive(Debug)]
enum SocketState {
    On,
    Off,
}

impl fmt::Display for SocketState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct SmartSocket {
    state: SocketState,
    description: String,
    current_power_consumption_in_kilowatt_hours: f32,
}

impl SmartSocket {
    fn state(&self) -> &SocketState {
        &self.state
    }
    fn description(&self) -> &String {
        &self.description
    }
    fn current_power_consumption_in_kilowatt_hours(&self) -> f32 {
        self.current_power_consumption_in_kilowatt_hours
    }
}

struct SmartThermometer {
    current_tempperature_in_celsius: f32,
}

impl SmartThermometer {
    fn current_tempperature_in_celsius(&self) -> f32 {
        self.current_tempperature_in_celsius
    }
}

fn main() {
    let mut smart_socket1 = SmartSocket {
        state: SocketState::On,
        description: String::from("cool smart socket"),
        current_power_consumption_in_kilowatt_hours: 15.0,
    };
    let smart_thermometer = SmartThermometer {
        current_tempperature_in_celsius: 25.0,
    };
    print_socket_data(&smart_socket1);
    smart_socket1.state = SocketState::Off;
    print_socket_data(&smart_socket1);
    print_thermometer_data(&smart_thermometer);
}

fn print_socket_data(smart_socket: &SmartSocket) {
    println!("smart socket with state={}, description {}, and curent power consumption {} kilowatt/hours", 
        smart_socket.state(), smart_socket.description(), smart_socket.current_power_consumption_in_kilowatt_hours());
}

fn print_thermometer_data(smart_thermometer: &SmartThermometer) {
    println!(
        "smart thermometer with curent power temperature {} celsius",
        smart_thermometer.current_tempperature_in_celsius()
    );
}
