use std::fs;
use std::path::Path;

const GPIO_CHIP0: &str = "gpiochip0"; 

fn main() -> std::io::Result<()> {
    println!("Hello, RK399!");
    println!("GPIO pinout via sysfs");

    // Path to the GPIO class directory
    let gpio_class_dir = Path::new("/sys/class/gpio");

    // Read the entries in the GPIO class directory
    let entries = fs::read_dir(gpio_class_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a GPIO chip directory (e.g., gpiochip0)
        if path.is_dir()
            && path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with(GPIO_CHIP0)
        {
            let chip_name = path.file_name().unwrap().to_string_lossy();
            println!("Found GPIO chip: {}", chip_name);

            // Read the number of lines (pins) for this chip
            let label_path = path.join("label");
            let label = fs::read_to_string(label_path).unwrap_or_else(|_| "Unknown".to_string());
            println!("  Label: {}", label.trim());

            let ngpio_path = path.join("ngpio");
            let ngpio = fs::read_to_string(ngpio_path)?
                .trim()
                .parse::<u32>()
                .unwrap_or(0);
            println!("  Number of GPIOs: {}", ngpio);

            // List each GPIO pin
            for pin in 0..ngpio {
                let pin_dir = path.join(format!("gpio{}", pin));
                if pin_dir.exists() {
                    let direction_path = pin_dir.join("direction");
                    let direction = fs::read_to_string(direction_path)
                        .unwrap_or_else(|_| "unknown".to_string());

                    let value_path = pin_dir.join("value");
                    let value =
                        fs::read_to_string(value_path).unwrap_or_else(|_| "unknown".to_string());

                    println!(
                        "    Pin {}: Direction = {}, Value = {}",
                        pin,
                        direction.trim(),
                        value.trim()
                    );
                } else {
                    println!("    Pin {}: Not exported", pin);
                }
            }
        }
    }

    Ok(())
}
