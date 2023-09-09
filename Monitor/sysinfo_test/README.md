# Sysinfo

* _Sysinfo or system information in fuzzing refers to collecting details about the environment and system where fuzzing is being performed._

# Build

```bash
$ cargo run .
```

# Output

```bash
[16:42] raminfp@zenbook:sysinfo_test (main) # cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/sysinfo_test`
Getting processes' information...
Done.
To get the commands' list, enter 'help'.
> help
== Help menu ==
help               : show this menu
signals            : show the available signals
refresh            : reloads all processes information
refresh [pid]      : reloads corresponding process information
refresh_disks      : reloads only disks information
refresh_users      : reloads only users information
refresh_networks   : reloads only networks information
show [pid | name]  : show information of the given process corresponding to [pid | name]
kill [pid] [signal]: send [signal] to the process with this [pid]. 0 < [signal] < 32
cpus               : Displays CPUs state
memory             : Displays memory state
temperature        : Displays components' temperature
disks              : Displays disks' information
network            : Displays network' information
all                : Displays all process name and pid
uptime             : Displays system uptime
boot_time          : Displays system boot time
vendor_id          : Displays CPU vendor id
brand              : Displays CPU brand
load_avg           : Displays system load average
frequency          : Displays CPU frequency
users              : Displays all users and their groups
system             : Displays system information (such as name, version and hostname)
pid                : Display this example's PID
quit               : Exit the program
> cpus
number of physical cores: 4
total CPU usage: 13.624922%
Cpu { name: "cpu0", CPU usage: 0.0, frequency: 3315, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }
Cpu { name: "cpu1", CPU usage: 0.0, frequency: 3400, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }
Cpu { name: "cpu2", CPU usage: 0.0, frequency: 3400, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }
Cpu { name: "cpu3", CPU usage: 0.0, frequency: 3300, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }
Cpu { name: "cpu4", CPU usage: 0.0, frequency: 3290, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }
Cpu { name: "cpu5", CPU usage: 0.0, frequency: 3401, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }
Cpu { name: "cpu6", CPU usage: 0.0, frequency: 3301, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }
Cpu { name: "cpu7", CPU usage: 0.0, frequency: 3400, vendor ID: "GenuineIntel", brand: "Intel(R) Core(TM) i7-10510U CPU @ 1.80GHz" }

```