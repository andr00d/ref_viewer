use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;

pub struct Exiftool
{
    exif: std::process::Child,
    thread: thread::JoinHandle<()>,
    stdin: std::process::ChildStdin,
    thd_rx: std::sync::mpsc::Receiver<String>,
    stop_tx: std::sync::mpsc::Sender<String>, 
}

impl Drop for Exiftool {
    fn drop(&mut self) 
    {
        let command = "-stay_open\nFalse\n".to_string();
        self.stdin.write(command.as_bytes()).unwrap();

        let _ = self.stop_tx.send("".to_string());
    }
}

impl Exiftool 
{
    pub fn new() -> Exiftool
    {
        let mut exif = match Command::new("exiftool")
                            .args(["-stay_open", "true", "-@", "-"])
                            .stdin(Stdio::piped())
                            .stdout(Stdio::piped())
                            .spawn()
        {
            Err(x) => panic!("error starting exiftool: {}", x),
            Ok(x) => x,
        };   
        
        let (thd_tx, thd_rx) = mpsc::channel();
        let (stop_tx, stop_rx) = mpsc::channel();
        let stdin = exif.stdin.take().unwrap();
        let stdout = exif.stdout.take().unwrap();
        
        let thread = Self::thr_stdin(stdout, thd_tx, stop_rx);

        Exiftool{
            exif: exif,
            thread: thread,
            stdin: stdin,
            thd_rx: thd_rx,
            stop_tx: stop_tx,
        }
    }

    
    fn thr_stdin(stdout: std::process::ChildStdout, 
        tx: std::sync::mpsc::Sender<String>,
        rx: std::sync::mpsc::Receiver<String>) -> thread::JoinHandle<()>
    {
        thread::spawn(move ||
        {
            let mut stdout_lines = BufReader::new(stdout).lines();
            let mut readout = String::new();

            loop
            {
                match rx.try_recv() 
                {
                    Ok(_) => {break;}
                    Err(_) => ()
                }

                match stdout_lines.next()
                {
                    Some(line) =>
                    {
                        // println!("{}", line.as_ref().unwrap().as_str());
                        match line.as_ref().unwrap().as_str()
                        {
                            "{ready}" =>
                            {
                                let _ = tx.send(readout.clone());
                                readout.clear();
                            } 
                            other =>
                            {
                                readout.push_str(other);
                                readout.push('\n');
                            }
                        } 
                    },
                    None => (),
                }
            }
        })
    }  

    pub fn get_folder_data(&mut self, path: &String) ->  Result<String, String>
    {
        let mut command = "\n-FileOrder8\nFileName\n-Artist\n-PageName\n-ImageSize\n-UserComment\n-r\n-json\n".to_string();
        command.push_str("-ext\njpg\n-ext\npng\n-ext\nwebp\n-ext\ngif\n");
        command.push_str(path);
        command.push_str("\n-execute\n");

        self.stdin.write(command.as_bytes()).unwrap();
        let result = self.thd_rx.recv().unwrap();
        return Ok(result);
    }   

    pub fn set_usercomment(&mut self, path: &String, tag: &String) ->  Result<String, String>
    {
        let mut command = "-overwrite_original\n-m\n-usercomment=".to_string();
        command.push_str(tag);
        command.push_str("\n");
        command.push_str(path);
        command.push_str("\n-execute\n");

        self.stdin.write(command.as_bytes()).unwrap();
        let result = self.thd_rx.recv().unwrap();
        return Ok(result);
    }   

    pub fn set_link(&mut self, path: &String, tag: &String) ->  Result<String, String>
    {
        let mut command = "-overwrite_original\n-m\n-PageName=".to_string();
        command.push_str(tag);
        command.push_str("\n");
        command.push_str(path);
        command.push_str("\n-execute\n");

        self.stdin.write(command.as_bytes()).unwrap();
        let result = self.thd_rx.recv().unwrap();
        return Ok(result);
    }   

    pub fn set_artist(&mut self, path: &String, tag: &String) ->  Result<String, String>
    {
        let mut command = "-overwrite_original\n-m\n-Artist=".to_string();
        command.push_str(tag);
        command.push_str("\n");
        command.push_str(path);
        command.push_str("\n-execute\n");

        self.stdin.write(command.as_bytes()).unwrap();
        let result = self.thd_rx.recv().unwrap();
        return Ok(result);
    }   
}