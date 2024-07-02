Simple program that displays bluetooth (BLE) and their charge level.

Also supports reading Dygma Defy wireless battery level from Neuron. [Commands spec](https://github.com/Dygmalab/Bazecor/blob/development/src/api/hardware-virtual/DefyWireless.ts)

Example:
```
$ battery-status --bluetooth-support --dygma-support
MX Master 3S: 90
Defy BLE - 2: 100
```