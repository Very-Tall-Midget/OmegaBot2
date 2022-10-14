use std::ffi::CString;
use winapi::{
    shared::{
        minwindef::{DWORD, LPVOID},
        winerror::ERROR_PIPE_BUSY,
    },
    um::{
        fileapi::OPEN_EXISTING,
        handleapi::INVALID_HANDLE_VALUE,
        minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES},
        winnt::{GENERIC_READ, GENERIC_WRITE, HANDLE, LPCSTR},
    },
};

use std::sync::Mutex;

use crate::{
    hack_handler::HackName, omegabot::NoClip, replay::ReplayType, spam_bot::SpamBotPlayer,
};
use macros::messages;

messages! {
    Ping,
    Error(String) | {
        Error(string_from_utf16!(data))
    } | {
        Error(err) => {
            let mut buffer = vec![0u16; err.len() + 1];
            buffer.copy_from_slice(err.encode_utf16().collect::<Vec<u16>>().as_slice());
            buffer
        }
    },
    Received,
    Exit,
    ChangeFPS(f32) | {
        ChangeFPS(unsafe { *std::mem::transmute::<*const u16, *const f32>(data.as_ptr()) })
    } | {
        ChangeFPS(fps) => {
            let mut buffer = vec![0u16; 3];
            unsafe {
                *std::mem::transmute::<*mut u16, *mut f32>(buffer.as_mut_ptr()) = fps;
            }
            buffer
        }
    },
    Speedhack(f32) | {
        Speedhack(unsafe { *std::mem::transmute::<*const u16, *const f32>(data.as_ptr()) })
    } | {
        Speedhack(speedhack) => {
            let mut buffer = vec![0u16; 3];
            unsafe {
                *std::mem::transmute::<*mut u16, *mut f32>(buffer.as_mut_ptr()) = speedhack;
            }
            buffer
        }
    },
    RespawnTime(f32) | {
        RespawnTime(unsafe { *std::mem::transmute::<*const u16, *const f32>(data.as_ptr()) })
    } | {
        RespawnTime(respawn_time) => {
            let mut buffer = vec![0u16; 3];
            unsafe {
                *std::mem::transmute::<*mut u16, *mut f32>(buffer.as_mut_ptr()) = respawn_time;
            }
            buffer
        }
    },
    FrameAdvance(bool) | {
        FrameAdvance(data[0] == 1)
    } | {
        FrameAdvance(frame_advance) => {
            let mut buffer = vec![0u16; 2];
            buffer[0] = if frame_advance { 1 } else { 0 };
            buffer
        }
    },
    AccuracyFix(bool) | {
        AccuracyFix(data[0] == 1)
    } | {
        AccuracyFix(accuracy_fix) => {
            let mut buffer = vec![0u16; 2];
            buffer[0] = if accuracy_fix { 1 } else { 0 };
            buffer
        }
    },
    PracticeFix(bool) | {
        PracticeFix(data[0] == 1)
    } | {
        PracticeFix(practice_fix) => {
            let mut buffer = vec![0u16; 2];
            buffer[0] = if practice_fix { 1 } else { 0 };
            buffer
        }
    },
    SetNoClip(NoClip) | {
        SetNoClip(data[0].into())
    } | {
        SetNoClip(no_clip) => {
            let mut buffer = vec![0u16; 2];
            buffer[0] = no_clip.into();
            buffer
        }
    },
    StartPlayback,
    StopPlayback,
    StartRecording,
    StopRecording,
    Append,
    SaveReplay(String) | {
        SaveReplay(string_from_utf16!(data))
    } | {
        SaveReplay(filename) => {
            let mut buffer = vec![0u16; filename.len() + 1];
            buffer.copy_from_slice(filename.encode_utf16().collect::<Vec<u16>>().as_slice());
            buffer
        }
    },
    LoadReplay(String) | {
        LoadReplay(string_from_utf16!(data))
    } | {
        LoadReplay(filename) => {
            let mut buffer = vec![0u16; filename.len() + 1];
            buffer.copy_from_slice(filename.encode_utf16().collect::<Vec<u16>>().as_slice());
            buffer
        }
    },
    ApplyHack(HackName) | {
        ApplyHack(data[0].into())
    } | {
        ApplyHack(hack) => {
            vec![hack.into(), 0]
        }
    },
    RestoreHack(HackName) | {
        RestoreHack(data[0].into())
    } | {
        RestoreHack(hack) => {
            vec![hack.into(), 0]
        }
    },

    SetReplayType(ReplayType) | {
        SetReplayType(if data[0] == 1 {
            ReplayType::Frame
        } else {
            ReplayType::XPos
        })
    } | {
        SetReplayType(replay_type) => {
            vec![if replay_type == ReplayType::Frame { 1 } else { 2 }, 0]
        }
    },

    // SpamBot
    SetStraightFlyAccuracy(f32) | {
        SetStraightFlyAccuracy(unsafe { *std::mem::transmute::<*const u16, *const f32>(data.as_ptr()) })
    } | {
        SetStraightFlyAccuracy(accuracy) => {
            let mut buffer = vec![0u16; 3];
            unsafe {
                *std::mem::transmute::<*mut u16, *mut f32>(buffer.as_mut_ptr()) = accuracy;
            }
            buffer
        }
    },
    SetStraightFlyPlayer(SpamBotPlayer) | {
        SetStraightFlyPlayer(match data[0] {
            1 => SpamBotPlayer::Player1,
            2 => SpamBotPlayer::Player2,
            3 => SpamBotPlayer::Both,
            _ => unreachable!(),
        })
    } | {
        SetStraightFlyPlayer(player) => {
            vec![match player {
                SpamBotPlayer::Player1 => 1,
                SpamBotPlayer::Player2 => 2,
                SpamBotPlayer::Both => 3,
            }, 0]
        }
    },
    SetStraightFlyKeybind(char) | {
        SetStraightFlyKeybind(char::from_u32(data[0] as u32).unwrap())
    } | {
        SetStraightFlyKeybind(key) => {
            vec![key as u16, 0]
        }
    },
    SetSpamPress(u32) | {
        SetSpamPress(unsafe { *std::mem::transmute::<*const u16, *const u32>(data.as_ptr()) })
    } | {
        SetSpamPress(frames) => {
            let mut buffer = vec![0u16; 3];
            unsafe {
                *std::mem::transmute::<*mut u16, *mut u32>(buffer.as_mut_ptr()) = frames;
            }
            buffer
        }
    },
    SetSpamRelease(u32) | {
        SetSpamRelease(unsafe { *std::mem::transmute::<*const u16, *const u32>(data.as_ptr()) })
    } | {
        SetSpamRelease(frames) => {
            let mut buffer = vec![0u16; 3];
            unsafe {
                *std::mem::transmute::<*mut u16, *mut u32>(buffer.as_mut_ptr()) = frames;
            }
            buffer
        }
    },
    SetSpamPlayer(SpamBotPlayer) | {
        SetSpamPlayer(match data[0] {
            1 => SpamBotPlayer::Player1,
            2 => SpamBotPlayer::Player2,
            3 => SpamBotPlayer::Both,
            _ => unreachable!(),
        })
    } | {
        SetSpamPlayer(player) => {
            vec![match player {
                SpamBotPlayer::Player1 => 1,
                SpamBotPlayer::Player2 => 2,
                SpamBotPlayer::Both => 3,
            }, 0]
        }
    },
    SetSpamKeybind(char) | {
        SetSpamKeybind(char::from_u32(data[0] as u32).unwrap())
    } | {
        SetSpamKeybind(key) => {
            vec![key as u16, 0]
        }
    },
}

pub struct Pipe {
    hpipe: HANDLE,
    name: String,
    connected: bool,
    mutex: Mutex<()>,
}

unsafe impl Send for Pipe {}
unsafe impl Sync for Pipe {}

impl Pipe {
    pub fn new(name: &str) -> Self {
        Self {
            hpipe: INVALID_HANDLE_VALUE,
            name: format!("\\\\.\\pipe\\{}", name),
            connected: false,
            mutex: Mutex::new(()),
        }
    }

    pub fn connect(&mut self) {
        let name = CString::new(self.name.as_str()).unwrap();
        let name: LPCSTR = name.as_ptr();

        loop {
            self.hpipe = unsafe {
                winapi::um::fileapi::CreateFileA(
                    name,
                    GENERIC_READ | GENERIC_WRITE,
                    0,
                    0 as LPSECURITY_ATTRIBUTES,
                    OPEN_EXISTING,
                    0,
                    0 as LPVOID,
                )
            };

            if self.hpipe != INVALID_HANDLE_VALUE {
                break;
            }
            if unsafe { winapi::um::errhandlingapi::GetLastError() } != ERROR_PIPE_BUSY {
                unsafe { winapi::um::handleapi::CloseHandle(self.hpipe) };
            } else if unsafe { winapi::um::winbase::WaitNamedPipeA(name, 20000) } == 0 {
                println!("Pipe timed out, retrying...");
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        self.connected = true;
    }

    pub fn write(&mut self, msg: Message, wait: bool) {
        if !self.exists() {
            panic!("Pipe not connected");
        }

        let guard = self.mutex.lock().unwrap();

        let buffer: Vec<u16> = msg.into();
        let mut bytes_written: DWORD = 0;

        if unsafe {
            winapi::um::fileapi::WriteFile(
                self.hpipe,
                buffer.as_ptr() as LPVOID,
                (buffer.len() * 2) as u32,
                &mut bytes_written,
                0 as LPOVERLAPPED,
            )
        } == 0
        {
            self.connected = false;
            panic!("Failed to write to pipe");
        }
        drop(guard);

        if bytes_written != (buffer.len() * 2) as u32 {
            self.connected = false;
            panic!("Failed to write to pipe");
        }

        if wait && self.read() != Message::Received {
            self.connected = false;
            panic!("Failed to wait for response");
        }
    }

    pub fn read(&mut self) -> Message {
        if !self.exists() {
            panic!("Pipe not connected");
        }

        let guard = self.mutex.lock().unwrap();

        let mut buffer = vec![0u16; 1024];
        let mut bytes_read = 0;
        if unsafe {
            winapi::um::fileapi::ReadFile(
                self.hpipe,
                buffer.as_mut_ptr() as LPVOID,
                (buffer.len() * 2) as u32,
                &mut bytes_read,
                0 as LPOVERLAPPED,
            )
        } == 0
        {
            self.connected = false;
            panic!("Failed to read from pipe, error: {}", unsafe {
                winapi::um::errhandlingapi::GetLastError()
            });
        }
        drop(guard);

        buffer.truncate(bytes_read as usize);
        buffer.into()
    }

    pub fn disconnect(&mut self) {
        if self.exists() {
            unsafe {
                winapi::um::handleapi::CloseHandle(self.hpipe);
            }
            self.connected = false;
        }
    }

    pub fn exists(&self) -> bool {
        self.hpipe != INVALID_HANDLE_VALUE && self.connected
    }
}
