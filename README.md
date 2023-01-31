<p align="center">
  <a href="https://github.com/salvatore-giammanco/pi-tone">
    <img src="https://i.imgur.com/oX91gdh.png" alt="Logo" height="140" width="543>
  </a>
</p>

This project aims to become a Rust crate which provides the same functionalities as Arduino's `tone` and `noTone` functions and more to Raspberry PI.

## Work in progress
The project is currently during it's **very early stage**.

## To do:
- [ ] Add in the readme the configuration for enabling Pwm pins on the raspberry: https://docs.golemparts.com/rppal/0.14.1/rppal/pwm/
- [ ] Play tone in a separate thread with duration (?)
    - [ ] `blocking: bool` parameter
- [ ] Add possibility to play a song (sequence of pitches)
    - [x] Adjusting duration of the notes
    - [ ] Configuration from external files?
- [ ] Add algorithms for decoding audio files (mp3?) and play it
- [ ] Add license
- [ ] Complete docstring
- [ ] Add build pipeline (and release?) on github to crates.io