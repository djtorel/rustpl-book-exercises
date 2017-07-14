use std::io;

fn main() {
    loop {
        println!("Options:");
        println!("Enter 1 to convert Farenheit to Celsius");
        println!("Enter 2 to convert Celsius to Farenheit");
        let option: u32 = match user_input().trim().parse() {
            Ok(num) => {
                if num != 1 && num != 2 {
                    send_error(0);
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                send_error(0);
                continue
            }
        };

        println!("Enter temperature to convert");
        let temperature: f64 = match user_input().trim().parse() {
            Ok(num) => num,
            Err(_) => {
                send_error(0);
                continue
            }
        };

        println!("Converted temp: {}", convert_temp(option, temperature));
        break;
    }
}

fn convert_temp(opt: u32, temp: f64) -> f64 {
    if opt == 1 {
        (temp - 32.0) / 1.8
    } else {
        (temp * 1.8) + 32.0
    }
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect(
        "Failed to read line",
    );
    input
}

fn send_error(err_num: usize) {
    let error = [
        "Invalid option selected.",
        "Temperature must be a number.",
        "Failed to read line."
    ];
    println!("Error: {}", error[err_num]);
}
