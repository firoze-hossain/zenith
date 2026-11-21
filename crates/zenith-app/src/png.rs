//! Pure Rust uncompressed PNG encoder with Adler32 and CRC32.

pub fn encode_png_32(width: u32, height: u32, argb_buffer: &[u32]) -> Vec<u8> {
    let mut out = Vec::new();
    // 1. PNG Signature
    out.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]);

    // 2. IHDR Chunk
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.push(8); // Bit depth
    ihdr.push(6); // Color type: 6 = RGBA
    ihdr.push(0); // Compression method
    ihdr.push(0); // Filter method
    ihdr.push(0); // Interlace method
    write_chunk(&mut out, b"IHDR", &ihdr);

    // 3. IDAT Chunk: Raw uncompressed deflate stream (type 00)
    let row_bytes = width as usize * 4;
    let mut raw_data = Vec::with_capacity((row_bytes + 1) * height as usize);

    for y in 0..height as usize {
        raw_data.push(0); // Filter type 0 (None)
        let row_start = y * width as usize;
        for x in 0..width as usize {
            let px = argb_buffer[row_start + x];
            let a = ((px >> 24) & 0xFF) as u8;
            let r = ((px >> 16) & 0xFF) as u8;
            let g = ((px >> 8) & 0xFF) as u8;
            let b = (px & 0xFF) as u8;
            raw_data.push(r);
            raw_data.push(g);
            raw_data.push(b);
            raw_data.push(a);
        }
    }

    let idat = create_uncompressed_zlib_stream(&raw_data);
    write_chunk(&mut out, b"IDAT", &idat);

    // 4. IEND Chunk
    write_chunk(&mut out, b"IEND", &[]);

    out
}

fn write_chunk(out: &mut Vec<u8>, chunk_type: &[u8; 4], data: &[u8]) {
    let len = data.len() as u32;
    out.extend_from_slice(&len.to_be_bytes());
    out.extend_from_slice(chunk_type);
    out.extend_from_slice(data);

    let mut crc = Crc32::new();
    crc.update(chunk_type);
    crc.update(data);
    out.extend_from_slice(&crc.finalize().to_be_bytes());
}

fn create_uncompressed_zlib_stream(data: &[u8]) -> Vec<u8> {
    let mut zlib = Vec::new();
    // Zlib header: CMINFO=78 (Deflate 32k window), FLG=01 (No preset dict)
    zlib.push(0x78);
    zlib.push(0x01);

    // Chunk data into 65,535 byte Deflate non-compressed blocks
    let mut offset = 0;
    while offset < data.len() {
        let remaining = data.len() - offset;
        let chunk_size = remaining.min(65535);
        let is_last = (offset + chunk_size) >= data.len();

        let header = if is_last { 0x01 } else { 0x00 };
        zlib.push(header);

        let len_u16 = chunk_size as u16;
        let nlen_u16 = !len_u16;

        zlib.extend_from_slice(&len_u16.to_le_bytes());
        zlib.extend_from_slice(&nlen_u16.to_le_bytes());
        zlib.extend_from_slice(&data[offset..offset + chunk_size]);

        offset += chunk_size;
    }

    // Adler-32 checksum
    let adler = adler32(data);
    zlib.extend_from_slice(&adler.to_be_bytes());

    zlib
}

fn adler32(data: &[u8]) -> u32 {
    let mut s1: u32 = 1;
    let mut s2: u32 = 0;
    for &b in data {
        s1 = (s1 + b as u32) % 65521;
        s2 = (s2 + s1) % 65521;
    }
    (s2 << 16) | s1
}

struct Crc32 {
    crc: u32,
}

impl Crc32 {
    fn new() -> Self {
        Self { crc: 0xFFFFFFFF }
    }

    fn update(&mut self, data: &[u8]) {
        for &byte in data {
            let mut c = (self.crc ^ byte as u32) & 0xFF;
            for _ in 0..8 {
                if c & 1 != 0 {
                    c = 0xEDB88320 ^ (c >> 1);
                } else {
                    c >>= 1;
                }
            }
            self.crc = c ^ (self.crc >> 8);
        }
    }

    fn finalize(&self) -> u32 {
        self.crc ^ 0xFFFFFFFF
    }
}
