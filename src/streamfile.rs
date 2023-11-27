use std::io::{BufReader, Cursor, Read, Seek, self};

use crate::coding::ffmpeg_opus::OpusIOData;

#[derive(Clone, PartialEq, Eq)]
pub struct Streamfile {
    pub stream_index: i32,
    pub name: String,
    pub reader: Cursor<Vec<u8>>,

    pub read: Option<fn(&mut Streamfile, usize, usize, *mut std::ffi::c_void) -> Vec<u8>>,
    pub get_size: Option<fn(&mut Streamfile, *mut std::ffi::c_void) -> usize>,
    pub get_name: Option<fn(&mut Streamfile, *mut std::ffi::c_void) -> String>,
    pub open: Option<fn(String) -> Option<Streamfile>>,
    pub close: Option<fn(&mut Streamfile, *mut std::ffi::c_void)>,

    pub data: *mut std::ffi::c_void,
}

impl Streamfile {
    pub fn new(stream_index: i32, name: String, reader: Cursor<Vec<u8>>) -> Self {
        Self {
            stream_index,
            name,
            reader,
            open: Some(Self::open_stdio),
            read: Some(Self::read),
            get_size: Some(Self::get_size),
            get_name: Some(Self::get_name),
            close: Some(Self::close),
            data: std::ptr::null_mut(),
        }
    }

    pub fn open_stdio(filename: String) -> Option<Self> {
        let reader = std::fs::File::open(filename.clone()).ok()?;
        let bytes = BufReader::new(reader).bytes();
        let mut vec = Vec::new();
        for byte in bytes {
            vec.push(byte.unwrap());
        }
        // println!("{:?}", vec);
        Some(Self {
            stream_index: 0,
            name: filename.clone(),
            reader: Cursor::new(vec),
            open: Some(Self::open_stdio),
            read: Some(Self::read),
            get_size: Some(Self::get_size),
            get_name: Some(Self::get_name),
            close: Some(Self::close),
            data: std::ptr::null_mut(),
        })
    }

    pub fn get_name(&mut self, data: *mut std::ffi::c_void) -> String {
        return self.name.clone();
    }

    pub fn close(&mut self, data: *mut std::ffi::c_void) {
        self.reader = Cursor::new(Vec::new());
    }

    pub fn read(&mut self, offset: usize, length: usize, data: *mut std::ffi::c_void) -> Vec<u8> {
        let mut frame = vec![0; length as usize];
        self.reader
            .seek(std::io::SeekFrom::Start(offset as u64))
            .unwrap();
        self.reader.read_exact(&mut frame).unwrap();
        return frame;
    }

    pub fn get_size(&mut self, data: *mut std::ffi::c_void) -> usize {
        self.reader.get_ref().len()
    }
}

impl Default for Streamfile {
    fn default() -> Self {
        Self {
            stream_index: 0,
            name: String::new(),
            reader: Cursor::new(Vec::new()),
            open: Some(Self::open_stdio),
            read: Some(Self::read),
            get_size: Some(Self::get_size),
            get_name: Some(Self::get_name),
            close: Some(Self::close),
            data: std::ptr::null_mut(),
        }
    }
}

impl std::fmt::Debug for Streamfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Streamfile")
            .field("stream_index", &self.stream_index)
            .finish()
    }
}

pub fn check_extensions(sf: &mut Streamfile, extensions: Vec<&str>) -> bool {
    let name = sf.name.clone();
    let mut ext = name.split(".").last().unwrap().to_string();
    ext.make_ascii_lowercase();
    for extension in extensions {
        let mut ex = extension.to_string();
        ex.make_ascii_lowercase();
        if ext == ex {
            return true;
        }
    }
    return false;
}

pub fn read_u8(sf: &mut Streamfile, offset: usize) -> u8 {
    let mut buf = [0; 1];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return buf[0];
}

pub fn read_s8(sf: &mut Streamfile, offset: usize) -> i8 {
    return read_u8(sf, offset) as i8;
}

pub fn read_u16le(sf: &mut Streamfile, offset: usize) -> u16 {
    let mut buf = [0; 2];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return u16::from_le_bytes(buf);
}

pub fn read_s16le(sf: &mut Streamfile, offset: usize) -> i16 {
    return read_u16le(sf, offset) as i16;
}

pub fn read_u32le(sf: &mut Streamfile, offset: usize) -> u32 {
    let mut buf = [0; 4];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return u32::from_le_bytes(buf);
}

pub fn read_s32le(sf: &mut Streamfile, offset: usize) -> i32 {
    return read_u32le(sf, offset) as i32;
}

pub fn read_u64le(sf: &mut Streamfile, offset: usize) -> u64 {
    let mut buf = [0; 8];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return u64::from_le_bytes(buf);
}

pub fn read_s64le(sf: &mut Streamfile, offset: usize) -> i64 {
    return read_u64le(sf, offset) as i64;
}

pub fn read_u16be(sf: &mut Streamfile, offset: usize) -> u16 {
    let mut buf = [0; 2];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return u16::from_be_bytes(buf);
}

pub fn read_s16be(sf: &mut Streamfile, offset: usize) -> i16 {
    return read_u16be(sf, offset) as i16;
}

pub fn read_u32be(sf: &mut Streamfile, offset: usize) -> u32 {
    let mut buf = [0; 4];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return u32::from_be_bytes(buf);
}

pub fn read_s32be(sf: &mut Streamfile, offset: usize) -> i32 {
    return read_u32be(sf, offset) as i32;
}

pub fn read_u64be(sf: &mut Streamfile, offset: usize) -> u64 {
    let mut buf = [0; 8];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return u64::from_be_bytes(buf);
}

pub fn read_s64be(sf: &mut Streamfile, offset: usize) -> i64 {
    return read_u64be(sf, offset) as i64;
}

pub fn read_f32le(sf: &mut Streamfile, offset: usize) -> f32 {
    let mut buf = [0; 4];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return f32::from_le_bytes(buf);
}

pub fn read_f32be(sf: &mut Streamfile, offset: usize) -> f32 {
    let mut buf = [0; 4];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return f32::from_be_bytes(buf);
}

pub fn read_f64le(sf: &mut Streamfile, offset: usize) -> f64 {
    let mut buf = [0; 8];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return f64::from_le_bytes(buf);
}

pub fn read_f64be(sf: &mut Streamfile, offset: usize) -> f64 {
    let mut buf = [0; 8];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return f64::from_be_bytes(buf);
}

pub fn is_id32be(sf: &mut Streamfile, offset: usize, id: &str) -> bool {
    return read_u32be(sf, offset) == u32::from_be_bytes(id.as_bytes().try_into().unwrap());
}

// #[allow(arithmetic_overflow)]
pub fn get_id32be(s: &str) -> u32 {
    let s = s.as_bytes();
    return ((s[0] as u32) << 24) | ((s[1] as u32) << 16) as u32 | ((s[2] as u32) << 8) as u32 | ((s[3] as u32) << 0) as u32;
}

pub fn read_exact_bytes(sf: &mut Streamfile, offset: usize, size: usize) -> Vec<u8> {
    let mut buf = vec![0; size];
    sf.reader
        .seek(std::io::SeekFrom::Start(offset as u64))
        .unwrap();
    sf.reader.read_exact(&mut buf).unwrap();
    return buf;
}