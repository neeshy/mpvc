use std::collections::HashMap;

use ipc::*;

pub enum VolumeChangeOptions {
    Absolute,
    Increase,
    Decrease,
}

pub enum SeekOptions {
    Relative,
    Absolute,
    RelativePercent,
    AbsolutePercent,
}

pub enum PlaylistAddOptions {
    Replace,
    Append,
    AppendPlay,
}

pub struct Playlist {
    pub socket: String,
    pub entries: Vec<PlaylistEntry>,
}

pub trait PlaylistHandler {
    fn get_from(socket: Socket) -> Result<Playlist, String>;
    fn shuffle(&mut self) -> &mut Playlist;
    fn remove_id(&mut self, id: usize) -> &mut Playlist;
    fn move_entry(&mut self, from: usize, to: usize) -> &mut Playlist;
    fn current_id(&self) -> Option<usize>;
}

impl PlaylistHandler for Playlist {
    fn get_from(socket: Socket) -> Result<Playlist, String> {
        match get_mpv_property(&socket, "playlist") {
            Ok(playlist) => {
                Ok(Playlist {
                       socket: socket,
                       entries: playlist,
                   })
            }
            Err(why) => Err(why),
        }
    }

    fn shuffle(&mut self) -> &mut Playlist {
        if let Err(error_msg) = run_mpv_command(&self.socket, "playlist-shuffle", &vec![]) {
            error!("Error: {}", error_msg);
        }
        if let Ok(mut playlist_entries) =
            get_mpv_property::<Vec<PlaylistEntry>>(&self.socket, "playlist") {
            if self.entries.len() == playlist_entries.len() {
                for (i, entry) in playlist_entries.drain(0..).enumerate() {
                    self.entries[i] = entry;
                }
            }
        }
        self
    }

    fn remove_id(&mut self, id: usize) -> &mut Playlist {
        self.entries.remove(id);
        if let Err(error_msg) = run_mpv_command(&self.socket,
                                                "playlist-remove",
                                                &vec![&id.to_string()]) {
            error!("Error: {}", error_msg);
        }
        self
    }

    fn move_entry(&mut self, from: usize, to: usize) -> &mut Playlist {
        if from != to {
            if let Err(error_msg) = run_mpv_command(&self.socket,
                                                    "playlist-move",
                                                    &vec![&from.to_string(), &to.to_string()]) {
                error!("Error: {}", error_msg);
            }
            if from < to {
                self.entries[from].id = to - 1;
                self.entries[to].id = to - 2;
                for i in from..to - 2 {
                    self.entries[i + 1].id = i;
                }
                self.entries.sort_by_key(|entry| entry.id);
            } else if from > to {
                self.entries[from].id = to;
                for i in to..from - 1 {
                    self.entries[i].id = i + 1;
                }
                self.entries.sort_by_key(|entry| entry.id);
            }
        }
        self
    }

    fn current_id(&self) -> Option<usize> {
        for entry in self.entries.iter() {
            if entry.current {
                return Some(entry.id);
            }
        }
        None
    }
}

pub trait Commands {
    fn pause(&self) -> Result<(), String>;
    fn toggle(&self) -> Result<(), String>;
    fn next(&self) -> Result<(), String>;
    fn prev(&self) -> Result<(), String>;
    fn restart(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    fn get_metadata(&self) -> Result<HashMap<String, String>, String>;
    fn set_volume(&self, input_volume: f64, option: VolumeChangeOptions) -> Result<(), String>;
    fn seek(&self, seconds: f64, option: SeekOptions) -> Result<(), String>;
    fn playlist_add(&self, file: &str, option: PlaylistAddOptions) -> Result<(), String>;
    fn playlist_shuffle(&self) -> Result<(), String>;
    fn playlist_clear(&self) -> Result<(), String>;
    fn playlist_remove_id(&self, id: usize) -> Result<(), String>;
    fn playlist_move_id(&self, from: usize, to: usize) -> Result<(), String>;
    fn playlist_play_next(&self, id: usize) -> Result<(), String>;
    fn playlist_play_id(&self, id: usize) -> Result<(), String>;
    fn get_playlist(&self) -> Result<Playlist, String>;
}

impl Commands for Socket {
    fn pause(&self) -> Result<(), String> {
        set_mpv_property(self, "pause", true)
    }

    fn toggle(&self) -> Result<(), String> {
        match get_mpv_property::<bool>(self, "pause") {
            Ok(paused) => set_mpv_property(self, "pause", !paused),
            Err(msg) => Err(msg),
        }
    }

    fn next(&self) -> Result<(), String> {
        run_mpv_command(self, "playlist-next", &vec![])
    }

    fn prev(&self) -> Result<(), String> {
        run_mpv_command(self, "playlist-prev", &vec![])
    }

    fn restart(&self) -> Result<(), String> {
        run_mpv_command(self, "seek", &vec!["0", "absolute"])
    }

    fn stop(&self) -> Result<(), String> {
        run_mpv_command(self, "stop", &vec![])
    }

    fn get_metadata(&self) -> Result<HashMap<String, String>, String> {
        match get_mpv_property(self, "metadata") {
            Ok(map) => Ok(map),
            Err(err) => Err(err),
        }
    }

    fn set_volume(&self, input_volume: f64, option: VolumeChangeOptions) -> Result<(), String> {
        match get_mpv_property::<f64>(self, "volume") {
            Ok(volume) => {
                match option {
                    VolumeChangeOptions::Increase => {
                        set_mpv_property(self, "volume", volume + input_volume)
                    }

                    VolumeChangeOptions::Decrease => {
                        set_mpv_property(self, "volume", volume - input_volume)
                    }

                    VolumeChangeOptions::Absolute => set_mpv_property(self, "volume", input_volume),
                }
            }
            Err(msg) => Err(msg),
        }
    }

    fn seek(&self, seconds: f64, option: SeekOptions) -> Result<(), String> {
        match option {
            SeekOptions::Absolute => {
                run_mpv_command(self, "seek", &vec![&seconds.to_string(), "absolute"])
            }
            SeekOptions::AbsolutePercent => {
                run_mpv_command(self,
                                "seek",
                                &vec![&seconds.to_string(), "absolute-percent"])
            }
            SeekOptions::Relative => {
                run_mpv_command(self, "seek", &vec![&seconds.to_string(), "relative"])
            }
            SeekOptions::RelativePercent => {
                run_mpv_command(self,
                                "seek",
                                &vec![&seconds.to_string(), "relative-percent"])
            }
        }
    }

    fn playlist_add(&self, file: &str, option: PlaylistAddOptions) -> Result<(), String> {
        match option {
            PlaylistAddOptions::Replace => {
                run_mpv_command(self, "loadfile", &vec![file, "replace"])
            }
            PlaylistAddOptions::Append => run_mpv_command(self, "loadfile", &vec![file, "append"]),
            PlaylistAddOptions::AppendPlay => {
                run_mpv_command(self, "loadfile", &vec![file, "append-play"])
            }
        }
    }

    fn playlist_shuffle(&self) -> Result<(), String> {
        run_mpv_command(self, "playlist-shuffle", &vec![])
    }

    fn playlist_clear(&self) -> Result<(), String> {
        run_mpv_command(self, "playlist-clear", &vec![])
    }

    fn playlist_remove_id(&self, id: usize) -> Result<(), String> {
        run_mpv_command(self, "playlist-remove", &vec![&id.to_string()])
    }

    fn playlist_move_id(&self, from: usize, to: usize) -> Result<(), String> {
        run_mpv_command(self,
                        "playlist-remove",
                        &vec![&from.to_string(), &to.to_string()])
    }

    fn playlist_play_next(&self, id: usize) -> Result<(), String> {
        match Playlist::get_from(self.to_string()) {
            Ok(playlist) => {
                if let Some(current_id) = playlist.current_id() {
                    run_mpv_command(self,
                                    "playlist-move",
                                    &vec![&id.to_string(), &(current_id + 1).to_string()])
                } else {
                    Err("There is no file playing at the moment.".to_string())
                }
            }
            Err(why) => Err(why),
        }
    }

    fn playlist_play_id(&self, id: usize) -> Result<(), String> {
        set_mpv_property(self, "playlist-pos", id)
    }

    fn get_playlist(&self) -> Result<Playlist, String> {
        Playlist::get_from(self.to_string())
    }
}