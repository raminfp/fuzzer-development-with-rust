# Track Statistics

* _Tracking statistics and metrics is important in fuzzing to measure progress, effectiveness, and guide optimization._

# Build

```bash
$ cargo run .
```

# Output

```bash
Iterations: 404400, Crashes: 1619, Duration: 683.068459ms
Iterations: 404500, Crashes: 1619, Duration: 683.243411ms
Iterations: 404600, Crashes: 1619, Duration: 683.416157ms
Iterations: 404700, Crashes: 1619, Duration: 683.592143ms
Iterations: 404800, Crashes: 1619, Duration: 683.766659ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 404900, Crashes: 1620, Duration: 683.984433ms
Iterations: 405000, Crashes: 1620, Duration: 684.168334ms
Iterations: 405100, Crashes: 1620, Duration: 684.353465ms
Iterations: 405200, Crashes: 1620, Duration: 684.540648ms
Iterations: 405300, Crashes: 1620, Duration: 684.728361ms
Iterations: 405400, Crashes: 1620, Duration: 684.919743ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 405500, Crashes: 1622, Duration: 685.156631ms
Iterations: 405600, Crashes: 1622, Duration: 685.325019ms
Iterations: 405700, Crashes: 1622, Duration: 685.490191ms
Iterations: 405800, Crashes: 1622, Duration: 685.655595ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 405900, Crashes: 1623, Duration: 685.833477ms
Iterations: 406000, Crashes: 1623, Duration: 686.015053ms
Iterations: 406100, Crashes: 1623, Duration: 686.174719ms
Iterations: 406200, Crashes: 1623, Duration: 686.333121ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 406300, Crashes: 1624, Duration: 686.505011ms
Iterations: 406400, Crashes: 1624, Duration: 686.663543ms
Iterations: 406500, Crashes: 1624, Duration: 686.821723ms
Iterations: 406600, Crashes: 1624, Duration: 686.981492ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 406700, Crashes: 1625, Duration: 687.155282ms
Iterations: 406800, Crashes: 1625, Duration: 687.313633ms
Iterations: 406900, Crashes: 1625, Duration: 687.471874ms
Iterations: 407000, Crashes: 1625, Duration: 687.63ms
Iterations: 407100, Crashes: 1625, Duration: 687.78826ms
Iterations: 407200, Crashes: 1625, Duration: 687.946412ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 407300, Crashes: 1627, Duration: 688.130895ms
Iterations: 407400, Crashes: 1627, Duration: 688.296362ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 407500, Crashes: 1628, Duration: 688.478637ms
Iterations: 407600, Crashes: 1628, Duration: 688.656729ms
thread 'main' panicked at 'Crash!', src/main.rs:14:9
Iterations: 407700, Crashes: 1629, Duration: 688.858374ms
Iterations: 407800, Crashes: 1629, Duration: 689.05478ms
Iterations: 407900, Crashes: 1629, Duration: 689.241047ms

```