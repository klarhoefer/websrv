
use std::io;
use std::io::prelude::*;
use std::net::TcpListener;

mod registry;

mod request;
use request::Request;

mod mime;
use mime::MimeType;

fn open_browser(path: &str, port: u16) -> io::Result<()> {
    let addr = format!("http://localhost:{}", port);
    std::process::Command::new(&path).arg(addr).spawn().map(|_| ())
}

struct Resource<'a> {
    location: &'a str,
    mime_type: MimeType,
    content: &'a [u8]
}

impl<'a> Resource<'a> {
    fn send(&self, stream: &mut impl Write) -> io::Result<()> {
        write!(stream, "HTTP/1.1 200 OK\nContent-Type: {}\nContent-Length: {}\n\n",
            self.mime_type.as_str(), self.content.len())?;
        stream.write(self.content)?;
        Ok(())
    }
}

fn main() -> io::Result<()> {

    if let Some(chrome_path) = registry::get_chrome_path() {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let port = listener.local_addr()?.port();
        println!("Listening at port {}, starting browser...", port);

        open_browser(&chrome_path, port)?;

        let resources = [
            Resource { location: "/", mime_type: MimeType::Html, content: include_bytes!("..\\res\\index.html") },
            Resource { location: "/quit", mime_type: MimeType::Html, content: include_bytes!("..\\res\\exit.html") },
            Resource { location: "/favicon.ico", mime_type: MimeType::Ico, content: include_bytes!("..\\res\\favicon.ico") },
            Resource { location: "/index.css", mime_type: MimeType::Css, content: include_bytes!("..\\res\\index.css") },
        ];

        'incoming: for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                if let Ok(req) = Request::parse(&mut stream) {
                    dbg!(&req);
                    let mut handled = false;

                    let location = req.location();
                    for res in &resources {
                        if res.location == location {
                            let _ = res.send(&mut stream);
                            if location == "/quit" {
                                break 'incoming;
                            }
                            handled = true;
                            break;
                        }
                    }

                    if !handled {
    
                    }
                }
            }
        }
    } else {
        eprintln!("Could not find Chrome browser on this machine");
    }

    Ok(())
}
