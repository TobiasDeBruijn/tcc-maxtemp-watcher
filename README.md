# tcc-maxtemp-watcher
Simple program to set the temperature of a Honeywell TCC heating zone if it gets above a configured threshold.
TCC has a built-in feature that does this, but it doesn't allow you to go below 21.0 degrees. This program allows you
to go below 21.0 degrees.

This program uses your TCC login credentials to perform its magic.

## Configuration
Set the following environmental variables:
- `TCC_EMAIL` Your TCC login email
- `TCC_PASSWORD` Your TCC login password
- `MAX_TEMPERATURE` The maximum temperature, in steps of half a degrees.
- `INTERVAL_SEC` How often the temperature should be checked. Setting it too short may result in API errors.

## License
This project is licensed under the MIT or Apache 2.0 license, at your option.