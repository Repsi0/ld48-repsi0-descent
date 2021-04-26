use quad_snd::mixer::SoundMixer;
use quad_snd::mixer::Sound;
use quad_snd::mixer::SoundId;
use quad_snd::mixer::PlaybackStyle;
use quad_snd::decoder::read_ogg_ext;

use crate::FINAL_SONG_PATH;
use crate::SHOP_SONG_PATH;
use crate::FALLING_SONG_PATH;
use crate::MISSILE_SHOOT_SOUND_PATH;
use crate::SHIELD_LOSS_SOUND_PATH;
use crate::LOSE_SOUND_PATH;

use std::fs::read as fs_read;

pub struct SoundController {
    falling_song: Sound,
    shop_song: Sound,
    final_song: Sound,
    missile_shoot: Sound,
    shield_loss: Sound,
    lose_sound: Sound,
    mixer: SoundMixer,
    shield_sound: Option<SoundId>,
    shop_mus: Option<SoundId>,
    falling_mus: Option<SoundId>,
}
impl SoundController {
    fn read_file(path: &str) -> Vec<u8> {
        let file = match fs_read(path) {
            Ok(o) => o,
            Err(e) => panic!("Error reading file {}! {}",path,e),
        };
        return file;
    }
    pub fn new() -> SoundController {
        let falling_song = read_ogg_ext(&SoundController::read_file(FALLING_SONG_PATH), PlaybackStyle::Looped).unwrap();
        let shop_song = read_ogg_ext(&SoundController::read_file(SHOP_SONG_PATH), PlaybackStyle::Looped).unwrap();
        let final_song = read_ogg_ext(&SoundController::read_file(FINAL_SONG_PATH), PlaybackStyle::Once).unwrap();
        let missile_shoot = read_ogg_ext(&SoundController::read_file(MISSILE_SHOOT_SOUND_PATH), PlaybackStyle::Once).unwrap();
        let shield_loss = read_ogg_ext(&SoundController::read_file(SHIELD_LOSS_SOUND_PATH), PlaybackStyle::Once).unwrap();
        let lose_sound = read_ogg_ext(&SoundController::read_file(LOSE_SOUND_PATH), PlaybackStyle::Once).unwrap();
        let mixer = SoundMixer::new();
        let shield_sound: Option<SoundId> = None;
        let shop_mus: Option<SoundId> = None;
        let falling_mus: Option<SoundId> = None;
        SoundController {
            falling_song,shop_song,final_song,missile_shoot,shield_loss,lose_sound,mixer,shield_sound,falling_mus,shop_mus
        }
    }
    pub fn play_final(&mut self) {
        self.mixer.play(self.final_song.clone());
    }
    pub fn play_missile(&mut self) {
        self.mixer.play(self.missile_shoot.clone());
    }
    pub fn play_shield(&mut self) {
        match self.shield_sound {
            Some(s) => {
                self.mixer.stop(s);
            },
            _ => (),
        };
        self.shield_sound = Some(self.mixer.play(self.shield_loss.clone()));
    }
    pub fn play_lose(&mut self) {
        self.mixer.play(self.lose_sound.clone());
    }
    pub fn play_shop(&mut self) {
        match self.shop_mus {
            Some(s) => {
                self.mixer.stop(s);
            },
            _ => (),
        }
        self.shop_mus = Some(self.mixer.play(self.shop_song.clone()));
    }
    pub fn stop_shop(&mut self) {
        match self.shop_mus {
            Some(s) => {
                self.mixer.stop(s);
                self.shop_mus = None;
            },
            _ => (),
        }
    }
    pub fn play_falling(&mut self) {
        match self.falling_mus {
            Some(s) => {
                self.mixer.stop(s);
            },
            _ => (),
        }
        self.falling_mus = Some(self.mixer.play(self.falling_song.clone()));
    }
    pub fn stop_falling(&mut self) {
        match self.falling_mus {
            Some(s) => {
                self.mixer.stop(s);
                self.falling_mus = None;
            },
            _ => (),
        }
    }
}