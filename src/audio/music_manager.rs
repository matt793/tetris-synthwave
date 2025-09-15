use std::path::{Path, PathBuf};
use std::fs;
use rodio::{Decoder, OutputStream, Sink};
use rand::prelude::*;
use anyhow::{Result, Context};

#[derive(Debug, Clone, PartialEq)]
pub enum MusicState {
    Idle,
    Playing,
    Gap,
}

pub struct MusicManager {
    // Configuration
    gap_seconds: f32,
    peak_volume: f32,
    floor_volume: f32,
    master_volume: f32,
    enabled: bool,

    // Audio system
    _stream: OutputStream,
    sink: Sink,
    
    // Track management
    tracks: Vec<PathBuf>,
    current_playlist: Vec<PathBuf>,
    current_track_index: usize,
    current_track_name: String,
    current_track_duration: f32,
    recent_tracks: Vec<PathBuf>, // Track last few played songs to avoid repeats
    
    // State machine
    state: MusicState,
    playback_time: f32,
    gap_timer: f32,
}

impl MusicManager {
    pub fn new(
        music_dirs: Vec<&str>,
        gap_seconds: f32,
        peak: f32,
        floor: f32,
        master: f32
    ) -> Result<Self> {
        // Initialize audio system
        let (stream, stream_handle) = OutputStream::try_default()
            .context("Failed to initialize audio output")?;
        let sink = Sink::try_new(&stream_handle)
            .context("Failed to create audio sink")?;

        // Scan for music files
        let tracks = Self::scan_music_files(&music_dirs)?;
        if tracks.is_empty() {
            eprintln!("Warning: No music files found in directories: {:?}", music_dirs);
        }

        let mut manager = Self {
            gap_seconds,
            peak_volume: peak,
            floor_volume: floor,
            master_volume: master,
            enabled: !tracks.is_empty(),
            
            _stream: stream,
            sink,
            
            tracks,
            current_playlist: Vec::new(),
            current_track_index: 0,
            current_track_name: String::new(),
            current_track_duration: 120.0, // default fallback
            recent_tracks: Vec::new(), // Initialize empty recent tracks list
            
            state: MusicState::Idle,
            playback_time: 0.0,
            gap_timer: 0.0,
        };

        // Initialize playlist
        manager.shuffle_playlist();
        
        Ok(manager)
    }

    fn scan_music_files(music_dirs: &[&str]) -> Result<Vec<PathBuf>> {
        let mut tracks = Vec::new();
        let extensions = ["mp3", "ogg", "wav", "flac"];
        
        for dir_str in music_dirs {
            let dir_path = Path::new(dir_str);
            if !dir_path.exists() {
                eprintln!("Warning: Music directory doesn't exist: {}", dir_str);
                continue;
            }
            
            match fs::read_dir(dir_path) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if let Some(ext) = path.extension() {
                                if let Some(ext_str) = ext.to_str() {
                                    if extensions.contains(&ext_str.to_lowercase().as_str()) {
                                        tracks.push(path);
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("Warning: Failed to read music directory {}: {}", dir_str, e),
            }
        }
        
        Ok(tracks)
    }

    fn shuffle_playlist(&mut self) {
        self.current_playlist = self.tracks.clone();
        let mut rng = thread_rng();
        self.current_playlist.shuffle(&mut rng);
        self.current_track_index = 0;
        
        if !self.current_playlist.is_empty() {
            println!("Shuffled playlist with {} tracks", self.current_playlist.len());
        }
    }

    fn get_track_duration(path: &Path) -> f32 {
        // Try to get duration from metadata
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                if ext_str.to_lowercase() == "mp3" {
                    if let Ok(duration) = mp3_duration::from_path(path) {
                        return duration.as_secs_f32();
                    }
                }
            }
        }
        
        // Fallback to safe default and log warning
        eprintln!("Warning: Could not determine duration for {:?}, using 120s fallback", 
                 path.file_name().unwrap_or_default());
        120.0
    }

    fn get_track_name(path: &Path) -> String {
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Unknown Track")
            .to_string()
    }

    pub fn start(&mut self) -> Result<()> {
        if !self.enabled || self.tracks.is_empty() {
            return Ok(());
        }
        
        self.play_next_track()
    }

    fn play_next_track(&mut self) -> Result<()> {
        if self.current_playlist.is_empty() {
            return Ok(());
        }

        // Check if we need to reshuffle
        if self.current_track_index >= self.current_playlist.len() {
            println!("End of playlist reached, reshuffling...");
            self.shuffle_playlist();
        }

        let track_path = &self.current_playlist[self.current_track_index];
        
        // Improved randomization: avoid recently played tracks
        if self.tracks.len() >= 3 && self.recent_tracks.contains(track_path) {
            // Try to find a non-recent track
            let mut attempts = 0;
            let max_attempts = 10; // Prevent infinite loop
            
            while attempts < max_attempts && self.recent_tracks.contains(&self.current_playlist[self.current_track_index]) {
                self.current_track_index = (self.current_track_index + 1) % self.current_playlist.len();
                attempts += 1;
            }
            
            if attempts >= max_attempts {
                // If we couldn't find a non-recent track, clear recent history and shuffle
                println!("Couldn't avoid recent tracks, clearing recent history and reshuffling...");
                self.recent_tracks.clear();
                self.shuffle_playlist();
            }
        }

        let track_path = &self.current_playlist[self.current_track_index];
        self.current_track_name = Self::get_track_name(track_path);
        self.current_track_duration = Self::get_track_duration(track_path);
        
        // Update recent tracks (keep last 2 tracks)
        self.recent_tracks.push(track_path.clone());
        if self.recent_tracks.len() > 2 {
            self.recent_tracks.remove(0);
        }
        
        println!("Now playing: {} ({:.1}s)", self.current_track_name, self.current_track_duration);
        
        // Stop current playback and clear the sink
        self.sink.stop();
        
        // Load and play the track
        match fs::File::open(track_path) {
            Ok(file) => {
                match Decoder::new(std::io::BufReader::new(file)) {
                    Ok(source) => {
                        self.sink.append(source);
                        self.sink.play(); // Ensure playback starts
                        self.state = MusicState::Playing;
                        self.playback_time = 0.0;
                        self.current_track_index += 1;
                    }
                    Err(e) => {
                        eprintln!("Failed to decode audio file {:?}: {}", track_path, e);
                        self.current_track_index += 1;
                        return self.play_next_track(); // Try next track
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open audio file {:?}: {}", track_path, e);
                self.current_track_index += 1;
                return self.play_next_track(); // Try next track
            }
        }
        
        Ok(())
    }

    pub fn update(&mut self, dt_ms: f32) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        let dt_seconds = dt_ms / 1000.0;
        
        match self.state {
            MusicState::Idle => {
                // Do nothing, waiting for start() to be called
            }
            
            MusicState::Playing => {
                self.playback_time += dt_seconds;
                
                // Apply volume envelope
                self.apply_volume_envelope();
                
                // More reliable track finished detection
                let track_finished = self.sink.empty() || 
                    (self.playback_time >= (self.current_track_duration - 0.5)); // End 0.5s early for smooth transition
                
                if track_finished {
                    println!("Track finished: {} (played {:.1}s / {:.1}s)", 
                             self.current_track_name, self.playback_time, self.current_track_duration);
                    self.state = MusicState::Gap;
                    self.gap_timer = 0.0;
                    
                    // Clear the sink to ensure it's ready for next track
                    self.sink.stop();
                }
            }
            
            MusicState::Gap => {
                self.gap_timer += dt_seconds;
                
                if self.gap_timer >= self.gap_seconds {
                    println!("Gap finished ({:.1}s), playing next track...", self.gap_seconds);
                    self.play_next_track()?;
                }
            }
        }
        
        Ok(())
    }

    fn apply_volume_envelope(&mut self) {
        let progress = (self.playback_time / self.current_track_duration).clamp(0.0, 1.0);
        
        // Create envelope: fade in to peak at midpoint, then fade out
        let envelope_volume = if progress <= 0.5 {
            // First half: floor -> peak
            let fade_progress = progress * 2.0; // 0.0 to 1.0
            self.floor_volume + (self.peak_volume - self.floor_volume) * fade_progress
        } else {
            // Second half: peak -> floor
            let fade_progress = (progress - 0.5) * 2.0; // 0.0 to 1.0
            self.peak_volume - (self.peak_volume - self.floor_volume) * fade_progress
        };
        
        let final_volume = (envelope_volume * self.master_volume).clamp(0.0, 1.0);
        self.sink.set_volume(final_volume);
    }

    pub fn toggle(&mut self, enabled: bool) {
        self.enabled = enabled;
        
        if !enabled {
            self.sink.pause();
            self.state = MusicState::Idle;
            println!("Music disabled");
        } else if !self.tracks.is_empty() {
            self.sink.play();
            if self.state == MusicState::Idle {
                let _ = self.start();
            }
            println!("Music enabled");
        }
    }


    pub fn current_track(&self) -> &str {
        if self.state == MusicState::Playing {
            &self.current_track_name
        } else {
            ""
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }

    /// Skip to the next track immediately
    pub fn skip_track(&mut self) -> Result<()> {
        if !self.enabled || self.tracks.is_empty() {
            return Ok(());
        }

        println!("Skipping current track...");
        
        // Stop current track and immediately start the next one
        self.sink.stop();
        self.state = MusicState::Gap;
        self.gap_timer = self.gap_seconds; // Skip the gap too
        self.play_next_track()
    }
}

impl Drop for MusicManager {
    fn drop(&mut self) {
        // Clean shutdown
        self.sink.stop();
    }
}
