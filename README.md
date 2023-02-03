<p align="center">
  <a href="https://github.com/salvatore-giammanco/pi-tone">
    <img src="https://i.imgur.com/oX91gdh.png" alt="Logo" width="300">
  </a>
</p>

This project aims to become a Rust crate which provides the same functionalities as Arduino's `tone` and `noTone` functions and more to Raspberry PI.

## Enabling PWM Channels
Pi-Tone uses Raspberry Pi's hardware PWM to create tones.
To enable only PWM0 on its default pin (BCM GPIO 18, physical pin 12), add `dtoverlay=pwm` to `/boot/config.txt` on **Raspbian** or `boot/firmware/usercfg.txt` on **Ubuntu**. If you need both PWM channels, replace `pwm` with `pwm-2chan`, which enables PWM0 and PWM1 on BCM GPIO 19 (physical pin 35).
More details on enabling and configuring PWM on other GPIO pins than the default ones can be found in /boot/overlays/README.

*This is an extract of `rppal` documentation. Read more [here](https://docs.golemparts.com/rppal/0.14.1/rppal/pwm/)*

## Work in progress
The project is currently during it's **very early stage**.

## To do:
- [ ] Play tone in a separate thread with duration (?)
    - [ ] `blocking: bool` parameter
- [ ] Add possibility to play a song (sequence of pitches)
    - [x] Adjusting duration of the notes
    - [ ] Configuration from external files?
- [ ] Add algorithms for decoding audio files (mp3?) and play it
- [ ] Add license
- [ ] Complete docstring
- [ ] Add build pipeline (and release?) on github to crates.io