/// A preset with all the pitches represented by their name
pub enum Pitch {
    /// No sound
    Pause,  
    /// 31 Hertz
    B0,     
    /// 33 Hertz
    C1,     
    /// 35 Hertz
    CS1,    
    /// 37 Hertz
    D1,     
    /// 39 Hertz
    DS1,    
    /// 41 Hertz
    E1,     
    /// 44 Hertz
    F1,     
    /// 46 Hertz
    FS1,    
    /// 49 Hertz
    G1,     
    /// 52 Hertz
    GS1,    
    /// 55 Hertz
    A1,     
    /// 58 Hertz
    AS1,    
    /// 62 Hertz
    B1,     
    /// 65 Hertz
    C2,     
    /// 69 Hertz
    CS2,    
    /// 73 Hertz
    D2,     
    /// 78 Hertz
    DS2,    
    /// 82 Hertz
    E2,     
    /// 87 Hertz
    F2,     
    /// 93 Hertz
    FS2,    
    /// 98 Hertz
    G2,     
    /// 104 Hertz
    GS2,    
    /// 110 Hertz
    A2,     
    /// 117 Hertz
    AS2,    
    /// 123 Hertz
    B2,     
    /// 131 Hertz
    C3,     
    /// 139 Hertz
    CS3,    
    /// 147 Hertz
    D3,     
    /// 156 Hertz
    DS3,    
    /// 165 Hertz
    E3,     
    /// 175 Hertz
    F3,     
    /// 185 Hertz
    FS3,    
    /// 196 Hertz
    G3,     
    /// 208 Hertz
    GS3,    
    /// 220 Hertz
    A3,     
    /// 233 Hertz
    AS3,    
    /// 247 Hertz
    B3,     
    /// 262 Hertz
    C4,     
    /// 277 Hertz
    CS4,    
    /// 294 Hertz
    D4,     
    /// 311 Hertz
    DS4,    
    /// 330 Hertz
    E4,     
    /// 349 Hertz
    F4,     
    /// 370 Hertz
    FS4,    
    /// 392 Hertz
    G4,     
    /// 415 Hertz
    GS4,    
    /// 440 Hertz
    A4,     
    /// 466 Hertz
    AS4,    
    /// 494 Hertz
    B4,     
    /// 523 Hertz
    C5,     
    /// 554 Hertz
    CS5,    
    /// 587 Hertz
    D5,     
    /// 622 Hertz
    DS5,    
    /// 659 Hertz
    E5,     
    /// 698 Hertz
    F5,     
    /// 740 Hertz
    FS5,    
    /// 784 Hertz
    G5,     
    /// 831 Hertz
    GS5,    
    /// 880 Hertz
    A5,     
    /// 932 Hertz
    AS5,    
    /// 988 Hertz
    B5,     
    /// 1047 Hertz
    C6,     
    /// 1109 Hertz
    CS6,    
    /// 1175 Hertz
    D6,     
    /// 1245 Hertz
    DS6,    
    /// 1319 Hertz
    E6,     
    /// 1397 Hertz
    F6,     
    /// 1480 Hertz
    FS6,    
    /// 1568 Hertz
    G6,     
    /// 1661 Hertz
    GS6,    
    /// 1760 Hertz
    A6,     
    /// 1865 Hertz
    AS6,    
    /// 1976 Hertz
    B6,     
    /// 2093 Hertz
    C7,     
    /// 2217 Hertz
    CS7,    
    /// 2349 Hertz
    D7,     
    /// 2489 Hertz
    DS7,    
    /// 2637 Hertz
    E7,     
    /// 2794 Hertz
    F7,     
    /// 2960 Hertz
    FS7,    
    /// 3136 Hertz
    G7,     
    /// 3322 Hertz
    GS7,    
    /// 3520 Hertz
    A7,     
    /// 3729 Hertz
    AS7,    
    /// 3951 Hertz
    B7,     
    /// 4186 Hertz
    C8,     
    /// 4435 Hertz
    CS8,    
    /// 4699 Hertz
    D8,     
    /// 4978 Hertz
    DS8,    
}


impl Pitch {
    /// Gets the frequency in Hertz of the pitch
    pub fn frequency(&self) -> f64 {
        match *self {
            Pitch::Pause => 0.0,
            Pitch::B0 =>  31.0,
            Pitch::C1 =>  33.0,
            Pitch::CS1 => 35.0,
            Pitch::D1 =>  37.0,
            Pitch::DS1 => 39.0,
            Pitch::E1 =>  41.0,
            Pitch::F1 =>  44.0,
            Pitch::FS1 => 46.0,
            Pitch::G1 =>  49.0,
            Pitch::GS1 => 52.0,
            Pitch::A1 =>  55.0,
            Pitch::AS1 => 58.0,
            Pitch::B1 =>  62.0,
            Pitch::C2 =>  65.0,
            Pitch::CS2 => 69.0,
            Pitch::D2 =>  73.0,
            Pitch::DS2 => 78.0,
            Pitch::E2 =>  82.0,
            Pitch::F2 =>  87.0,
            Pitch::FS2 => 93.0,
            Pitch::G2 =>  98.0,
            Pitch::GS2 => 104.0,
            Pitch::A2 =>  110.0,
            Pitch::AS2 => 117.0,
            Pitch::B2 =>  123.0,
            Pitch::C3 =>  131.0,
            Pitch::CS3 => 139.0,
            Pitch::D3 =>  147.0,
            Pitch::DS3 => 156.0,
            Pitch::E3 =>  165.0,
            Pitch::F3 =>  175.0,
            Pitch::FS3 => 185.0,
            Pitch::G3 =>  196.0,
            Pitch::GS3 => 208.0,
            Pitch::A3 =>  220.0,
            Pitch::AS3 => 233.0,
            Pitch::B3 =>  247.0,
            Pitch::C4 =>  262.0,
            Pitch::CS4 => 277.0,
            Pitch::D4 =>  294.0,
            Pitch::DS4 => 311.0,
            Pitch::E4 =>  330.0,
            Pitch::F4 =>  349.0,
            Pitch::FS4 => 370.0,
            Pitch::G4 =>  392.0,
            Pitch::GS4 => 415.0,
            Pitch::A4 =>  440.0,
            Pitch::AS4 => 466.0,
            Pitch::B4 =>  494.0,
            Pitch::C5 =>  523.0,
            Pitch::CS5 => 554.0,
            Pitch::D5 =>  587.0,
            Pitch::DS5 => 622.0,
            Pitch::E5 =>  659.0,
            Pitch::F5 =>  698.0,
            Pitch::FS5 => 740.0,
            Pitch::G5 =>  784.0,
            Pitch::GS5 => 831.0,
            Pitch::A5 =>  880.0,
            Pitch::AS5 => 932.0,
            Pitch::B5 =>  988.0,
            Pitch::C6 =>  1047.0,
            Pitch::CS6 => 1109.0,
            Pitch::D6 =>  1175.0,
            Pitch::DS6 => 1245.0,
            Pitch::E6 =>  1319.0,
            Pitch::F6 =>  1397.0,
            Pitch::FS6 => 1480.0,
            Pitch::G6 =>  1568.0,
            Pitch::GS6 => 1661.0,
            Pitch::A6 =>  1760.0,
            Pitch::AS6 => 1865.0,
            Pitch::B6 =>  1976.0,
            Pitch::C7 =>  2093.0,
            Pitch::CS7 => 2217.0,
            Pitch::D7 =>  2349.0,
            Pitch::DS7 => 2489.0,
            Pitch::E7 =>  2637.0,
            Pitch::F7 =>  2794.0,
            Pitch::FS7 => 2960.0,
            Pitch::G7 =>  3136.0,
            Pitch::GS7 => 3322.0,
            Pitch::A7 =>  3520.0,
            Pitch::AS7 => 3729.0,
            Pitch::B7 =>  3951.0,
            Pitch::C8 =>  4186.0,
            Pitch::CS8 => 4435.0,
            Pitch::D8 =>  4699.0,
            Pitch::DS8 => 4978.0,
        }
    }
}