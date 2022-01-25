# Rust CO2 monitor

This code was adapter from https://github.com/vit1251/rs-co2mon for use with a slightly different CO2 monitor from TFA: https://www.tfa-dostmann.de/en/product/co2-monitor-airco2ntrol-coach-31-5009/

This device does not use enctryption at all, which is nice.

I've changed the code to remove logging and write data to a temporary file, which gets picked up by Telegraf and fed into InfluxDB.
I've also added an udev rule and systemd service, so that the program automatically starts when the monitor is inserted.