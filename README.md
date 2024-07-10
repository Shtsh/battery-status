Simple program that displays bluetooth (BLE) and their charge level.

Also supports reading Dygma Defy wireless battery level from Neuron. [Commands spec](https://github.com/Dygmalab/Bazecor/blob/development/src/api/hardware-virtual/DefyWireless.ts)

Example:
```bash
battery-status --bluetooth-support --dygma-support
> MX Master 3S: 90
> Defy BLE - 2: 100
```

It is possible to print results as json
```bash
battery-status --bluetooth-support --dygma-support --json
> [{"battery_level":"90","battery_status":"Discharging","name":"MX Master 3S"},{"battery_level":"100","battery_status":"Discharging","name":"Defy BLE - 2"}]
```
