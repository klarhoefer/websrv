
use std::io;


#[derive(Debug)]
enum HttpMethod {
    Get, Post, Put,
    Unknown
}


#[derive(Debug)]
pub struct Request {
    method: HttpMethod,
    location: String,
    version: String,
}


fn parse_line(buffer: &[u8]) -> Option<(usize, usize, usize)> {
    let mut blanks = [0usize; 2];
    let mut count = 0;
    let mut i = 0;
    for b in buffer {
        match b {
            b' ' => {
                if count > 1 {
                    return None;
                }
                blanks[count] = i;
                count += 1;
            },
            b'\r' => break,
            _ => (),
        }
        i += 1;
    }
    if count == 2 {
        Some((blanks[0], blanks[1], i))
    } else {
        None
    }
}


impl Request {
    pub fn parse(stream: &mut impl io::Read) -> io::Result<Self> {
        let mut buffer = [0u8; 1024];
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Nothing read"));
        }

        if let Some((a, b, c)) = parse_line(&buffer[..count]) {
            let method = match &buffer[..a] {
                b"GET" => HttpMethod::Get,
                b"POST" => HttpMethod::Post,
                b"PUT" => HttpMethod::Put,
                _ => HttpMethod::Unknown
            };
    
            let location = String::from_utf8_lossy(&buffer[a+1..b]).to_string();
            let version = String::from_utf8_lossy(&buffer[b+1..c]).to_string();
            Ok(Request { method, location, version })
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid request"))
        }
    }

    pub fn location(&self) -> &str {
        &self.location
    }
}
