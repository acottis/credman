mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::Security::*,
    Windows::Win32::System::SystemServices::{PSTR, PWSTR},
    Windows::Win32::System::WindowsProgramming::FILETIME,
    Windows::Win32::System::Diagnostics::Debug::GetLastError,
};

use std::{borrow::BorrowMut};

// fn main() {
//     //store("Adam", "test", "Azphel:CredMan").unwrap();
//     //windows_rs()
//     rust_read().unwrap();
// }

#[no_mangle]
extern "C" fn store(user: *mut u8, pass: *mut u8, service: *mut u8) -> i32{

    let user = read_raw_string(user).expect(msg);
    let pass = read_raw_string(pass).unwrap();
    let service = read_raw_string(service).unwrap();

    rust_store(user, pass, service).expect("Could not store credentials");
    println!("Credentials wrote Sucessfully Stored");
    0
}

#[no_mangle]
extern "C" fn read(service: *mut u8){

    let service = read_raw_string(service).unwrap();

    match rust_read(service) {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg),
    }
}

fn rust_read<'a>(target: String) -> Result<&'a str, &'a str> {

    let target: *mut u8 = (target+"\0").to_string().as_mut_ptr();

    let mut credential = &CREDENTIALA::default();

    let result = unsafe{
        CredReadA(PSTR(target), 0x1, 0, core::mem::transmute(&mut credential))
    };
    if result.0 == 0 { println!("{:x?}", unsafe { GetLastError() } ); return Err("Credentials could not be read") } 

    let pass = read_raw_string(credential.CredentialBlob).expect("Error getting password");
    let user = read_raw_string(credential.UserName.0).expect("Error getting username");
    println!("Username: {}, Password: {}", user,pass);

    Ok("Credentials read sucessfully")
}


fn read_raw_string(addr: *mut u8) -> Result<String, Box<dyn std::error::Error>>{
    let buf: &mut [u8;255] = &mut ['\0' as u8; 255];
    let mut i: usize = 0;
    while (unsafe { *addr.offset(i as isize) } != '\0' as u8) && (buf.len() > i) {
        buf[i] = unsafe { *addr.offset(i as isize) };
        i += 1;
    }
    Ok(std::str::from_utf8(&buf[0..i])?.to_string())
}

fn rust_store<'a>(user: String, pass: String, service: String) -> core::result::Result<(), &'a str> {

    let len = pass.len()+1;

    let username = (user+"\0").to_string().as_mut_ptr();
    let service = (service+"\0").to_string().as_mut_ptr();
    let password = (pass+"\0").to_string().as_mut_ptr();

    let mut credential= CREDENTIALA {
        Flags: CRED_FLAGS(0),
        Type: CRED_TYPE(1),
        Persist: CRED_PERSIST(0x2),
        LastWritten: FILETIME::default(),
        UserName: PSTR(username),
        Comment: PSTR(std::ptr::null_mut()),
        TargetName: PSTR(service),
        CredentialBlob: password,
        CredentialBlobSize: len as u32,
        Attributes: std::ptr::null_mut(),
        AttributeCount: 0,
        TargetAlias: PSTR(service),
    };
    let result = unsafe { CredWriteA(credential.borrow_mut(), 0) };
    if result.0 == 0 { return Err("Failed to store credentials.") }
    Ok(())
}

#[no_mangle]
fn add(a: u32,b: u32) -> u32{
    a + b
}