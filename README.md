# HyperXBattery
This is a very little tool that reads the current battery level of the HyperX Cloud Alpha Wireless headset. 

## Installation
To use the command `hyperXBattery`, clone the repository, install rust and run the installer:
```
git clone https://github.com/Brannium/hyperXBattery.git
cd hyperXBattery
chmod +x install.sh
./install.sh
```

## Usage
Show the battery level by running this command when the dongle is plugged in and the headset is connected:
```
hyperXBattery
```

### Dank Material Shell
The battery percentage can be displayed in the [Dank Material Shell](https://duxlabs.dev/docs/dms/installation) menu bar using the [Dank Actions Plugin](https://github.com/duxlabs-dev/dux-plugins). The config could look like this:
![Text: -% | Cmd: hyperXBattery | Update: 300s | Vis: [[ $(hyperXBattery) =~ ^[0-9]+%$ ]]](/img/dms-dankaction-config.png?raw=true "Dank Action config")

## Uninstall
Uninstall the tool using the following commands:
```
chmod +x uninstall.sh
./uninstall.sh
```