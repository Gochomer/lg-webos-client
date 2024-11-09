use serde::Serialize;
use serde_json::{json, Value};
use sound::SoundOutput;

pub mod sound;


pub enum LunaCommands {
    ChangeSoundOutput(SoundOutput)
}

pub struct LunaCommand {
    pub uri: String,
    pub params: LunaParams,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LunaParams {
    pub category: String,
    #[serde(flatten)]
    pub settings: LunaSetting,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LunaSetting {
    Settings(Value),
    Keys(Vec<String>),
}


impl Into<LunaCommand> for LunaCommands {

    fn into(self) -> LunaCommand {
        match self {
            LunaCommands::ChangeSoundOutput(value) => LunaCommand {
                uri: "luna://com.webos.settingsservice/setSystemSettings".to_string(),
                params: LunaParams { 
                    category: "sound".to_string(),
                    settings: LunaSetting::Settings(json!({"soundOutput":value.to_string()}))
                }
            },
        }
    }

}