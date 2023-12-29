[esp-idf-hal](https://github.com/esp-rs/esp-idf-hal) based firmware for esp32 and probably other esp32 variants supported by the idf.

Will run as AP and return a page to insert wifi credentials.
Once those are in it will connect to the specified wifi (or reboot to AP if they arent valid) and host a simple REST server.

The API are as follow:
  - POST /turn_on - Turn PWN on a given gpio on. Takes a json object in the following format { "alias": "some_gpio_alias" }
  - POST /turn_off - Turn PWN off a given gpio on. Takes a json object in the following format { "alias": "some_gpio_alias" }
  - POST /dim - Sets a given gpio's cycle duty to a specified value.  Takes a json object in the following format { "alias": "some_gpio_alias", "dim": 100 }

GPIO configuration is done in the gpio_config.yaml file within the root directory. Format is as follow:

```yaml
light_1: # just a label - will probably turn it into the actual alias in the future
  gpio: 23 # target gpio
  channel: 0 # ledc channel, could be anything between 0 and 7 inclusive. GPIOs on the same channel will share the same PWM.
  frequency: 1000 # PWM frequency in HZ.
  alias: "light_1" # alias used with the REST apis. Will probably swap it for the label on top at some point 
```