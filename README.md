# SYSTEM
A 'System' describes a system which we would like to schedule automations for. 

`System`s are configured with a `toml` file which describes the location of all the plugin configuration files. 

By configuring the system in this way, we allow ourselves the flexibility to configure any number of plugins in our system, and the behavior of those plugins may be arbitrary.  

## EXAMPLE
```toml
# system.toml
plugins = ['light.toml', 'pump.toml']
```

# PLUGINS
A 'Plugin' is a driver for an external object that we would like to control with the scheduler. For example, this could be a powered outlet, a pump, or anything else. 

`Plugin`s are configured with a `toml` file, which describes their type, name, and the location of their schedule file. 

## EXAMPLE
```toml
# light.toml

plugin_type = "Tasmota"
name = "tasmota-light"
schedule_file = "light.json"
```
