use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

use rppal::pwm::{Channel, Pwm};

use crate::pitch::Pitch;

pub mod pitch;

pub struct PiTone {
    pwm: Pwm,
}

impl PiTone {
    pub fn new(channel: Channel) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            pwm: Pwm::new(channel)?,
        })
    }

    /// Plays a tone on a given pin
    ///
    /// * Arguments
    ///
    /// * `channel`     - rppal::pwm::Channel representing the raspberry PWM pin to play the tone on (Pwm0 or Pwm1)
    /// * `frequency`   - f64 value representing the Frequency of the tone in hertz
    /// * `duration`    - std::time::Duration value representing the duration of the tone
    pub fn tone(&self, frequency: f64, duration: Duration) -> Result<(), Box<dyn Error>> {
        let start = Instant::now();
        if frequency == 0.0 {
            self.pwm.disable()?;
        } else {
            self.pwm.set_frequency(frequency, 0.5)?;
            self.pwm.enable()?;   
        }
        thread::sleep(duration);
        loop {
            if start.elapsed() >= duration {
                self.pwm.disable()?;
                break;
            }
        }
        Ok(())
    }

    /// Plays a song
    /// 
    /// * Arguments
    /// 
    /// * `song` - A Song object with a list of all the Tones to be played    
    pub fn song(&self, song: Song) -> Result<(), Box<dyn Error>> {
        for tone in song.tones.iter() {
            self.tone(tone.pitch.frequency(), tone.duration)?;
        }
        Ok(())
    }

}

pub struct Tone {
    pitch: Pitch,
    duration: Duration,
}

impl Tone {
    pub fn new(pitch: Pitch, duration: u64) -> Self {
        Self {
            pitch,
            duration: Duration::from_millis(duration)
        }
    }
}


pub struct Song {
    tones: Vec<Tone>,
}
