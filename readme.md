# Rust + Esp32 + Esp-idf + Mpu6050 Motion Analyzer Project

A simple project that utilizes the Mpu6050 sensor, using an ESP32 and Rust as the programming language. This is just a personal project meant to complete a physics post exam assignment.

The reason for using esp-idf instead of the no_std version is because I had skill issues implementing the DHCP WiFi function in the no_std version, even though its binary file is smaller than the esp-idf version. (contact me if you can do these implementation)

## Completed
- Added WiFi Implementation

## Todo
- Add Websocket Implementation
- Fix the Mpu6050 Drifting Issues
- Make the Binary File as small as possible
