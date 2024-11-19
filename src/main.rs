use std::fs;
use serde::Deserialize;

/// A 'Plugin' is a driver for an external object that we would like to control with the scheduler. 
/// For example, this could be a powered outlet, a pump, or anything else.
///
/// `Plugin`s are configured with a `toml` file, which describes their type, name, and the location of their schedule file.
///
/// ```toml
/// # light.toml
///
/// plugin_type = "Tasmota"
/// name = "tasmota-light"
/// schedule_file = "light.json"
/// ```
#[derive(Deserialize, Debug)]
struct Plugin {
    plugin_type: String,
    name: String,
    schedule_file: String,
}
impl Plugin {
    fn new(config_file: &str) -> Self {
        let file: String = fs::read_to_string(&config_file).expect("Could not read input file");
        let plug: Self = toml::from_str(&file).expect("Could not parse TOML file");
        plug
    }

    fn print(&self) {
        println!("Name:         {}", self.name);
        println!("Type:         {}", self.plugin_type);
        println!("Sched. File:  {}", self.schedule_file);
    }
}

/// A 'System' describes a system which we would like to schedule automations for.
/// 
/// `System`s are configured with a `toml` file which describes the location of all the plugin configuration files.
/// 
/// By configuring the system in this way, we allow ourselves the flexibility to configure 
/// any number of plugins in our system, and the behavior of those plugins may be arbitrary.
/// 
/// ```toml
/// # system.toml
/// plugins = ['light.toml', 'pump.toml']
/// ```
struct System {
    plugins: Vec<Plugin>,
}

impl System {
    fn new(config_file: &str) -> Self {
        let file: String = fs::read_to_string(&config_file).expect("Could not read input file");
        let config: SystemConfig = toml::from_str(&file).expect("Could not parse TOML file");
        let mut system = System {
            plugins: Vec::new(),
        };
        for plugin in config.plugins {
            system.add_plugin(&plugin);
        }
        system
    }

    fn add_plugin(&mut self, plug_config_file: &str) {
        self.plugins.push(Plugin::new(plug_config_file));
    }

    fn print_plugins(&self) {
        println!("------PLUGINS------");
        for plugin in &self.plugins {
            plugin.print();
            println!("-------------------");
        }
    }
}

/// A `SystemConfig` is an intermediary we use to parse the input system configuration TOML file,
/// and use to construct our [`System`] with. 
#[derive(Deserialize, Debug)]
struct SystemConfig {
    plugins: Vec<String>,
}

fn main() {
    let s1 = System::new("config/DemoSystem.toml");
    s1.print_plugins();
}
