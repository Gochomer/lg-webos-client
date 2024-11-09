use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SoundOutput {
    /// TV Speaker
    TvSpeaker,
    /// HDMI(ARC) Device
    ExternalArc,
    /// Optical Out Device
    ExternalOptical,
    /// Bluetooth Device
    BtSoundbar,
    /// Mobile Device
    MobilePhone,
    /// Audio Out Device
    Lineout,
    /// Wired Headphones
    Headphone,
    /// Bluetooth Device + TV Speaker
    TvSpeakerBluetooth,
    /// Optical Out Device + TV Speaker
    TvExternalSpeaker,
    /// Wired Headphones + TV Speaker
    TvSpeakerHeadphone,
    /// WiSA Speakers
    WisaSpeaker,
}

impl fmt::Display for SoundOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            SoundOutput::TvSpeaker => "tv_speaker",
            SoundOutput::ExternalArc => "external_arc",
            SoundOutput::ExternalOptical => "external_optical",
            SoundOutput::BtSoundbar => "bt_soundbar",
            SoundOutput::MobilePhone => "mobile_phone",
            SoundOutput::Lineout => "lineout",
            SoundOutput::Headphone => "headphone",
            SoundOutput::TvSpeakerBluetooth => "tv_speaker_bluetooth",
            SoundOutput::TvExternalSpeaker => "tv_external_speaker",
            SoundOutput::TvSpeakerHeadphone => "tv_speaker_headphone",
            SoundOutput::WisaSpeaker => "wisa_speaker",
        };
        write!(f, "{}", name)
    }
}