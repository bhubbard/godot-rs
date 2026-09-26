use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AudioBus {
    pub name: String,
    pub volume_db: f32,
    pub mute: bool,
}

#[derive(Debug, Default)]
pub struct AudioServer {
    buses: Vec<AudioBus>,
}

static AUDIO_SERVER: RwLock<Option<AudioServer>> = RwLock::new(None);

impl AudioServer {
    pub fn init() {
        let mut lock = AUDIO_SERVER.write().unwrap();
        let master = AudioBus {
            name: "Master".to_string(),
            volume_db: 0.0,
            mute: false,
        };
        *lock = Some(Self { buses: vec![master] });
    }

    pub fn get_bus_count() -> usize {
        AUDIO_SERVER.read().map(|l| l.as_ref().map(|s| s.buses.len()).unwrap_or(0)).unwrap_or(0)
    }

    pub fn get_bus_volume_db(bus_idx: usize) -> f32 {
        AUDIO_SERVER
            .read()
            .ok()
            .and_then(|l| l.as_ref().and_then(|s| s.buses.get(bus_idx).map(|b| b.volume_db)))
            .unwrap_or(0.0)
    }

    pub fn set_bus_volume_db(bus_idx: usize, volume_db: f32) {
        if let Ok(mut lock) = AUDIO_SERVER.write() {
            if let Some(ref mut s) = *lock {
                if let Some(b) = s.buses.get_mut(bus_idx) {
                    b.volume_db = volume_db;
                }
            }
        }
    }
}
