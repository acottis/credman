mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::Security::*,
    Windows::Win32::System::SystemServices::{PSTR, CHAR},
    Windows::Win32::System::WindowsProgramming::FILETIME,
    Windows::Win32::System::Diagnostics::Debug::GetLastError,
    Windows::Win32::NetworkManagement::Rras::{ RasGetCredentialsA, RasSetCredentialsA, RASCREDENTIALSA, RASCM_Password, RASCM_UserName, RASCM_Domain},
};

use std::{borrow::BorrowMut, convert::{TryFrom, TryInto}};

// fn main() {
//     //store("Adam", "test", "Azphel:CredMan").unwrap();
//     //rust_read("*Session".to_string());
//     get_ras_cred()
// }

#[allow(non_camel_case_types)]
#[repr(C)]
struct c_str(*mut u8);

impl std::convert::From<c_str> for std::string::String{
    fn from(addr: c_str) -> Self {
        let buf: &mut [u8;255] = &mut ['\0' as u8; 255];
        let mut i: usize = 0;
        while (unsafe { *addr.0.offset(i as isize) } != '\0' as u8) && (buf.len() > i) {
            buf[i] = unsafe { *addr.0.offset(i as isize) };
            i += 1;
        }
        std::str::from_utf8(&buf[0..i]).unwrap().to_string()
    }
}

#[no_mangle]
extern "C" fn store(user: c_str, pass: c_str, service: c_str) -> i32{

    let user= String::from(user);
    let pass = String::from(pass);
    let service= String::from(service);

    rust_store(user, pass, service).expect("Could not store credentials");
    println!("Credentials wrote Sucessfully Stored");
    0
}

#[no_mangle]
extern "C" fn read(service: c_str){

    let service: String = service.into();

    match rust_read(service) {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg),
    }
}

fn rust_read<'a>(target: String) -> Result<&'a str, &'a str> {

    let target: *mut u8 = (target+"\0").to_string().as_mut_ptr();

    let mut credential = &CREDENTIALA::default();

    let result = unsafe{
        CredReadA(PSTR(target), 0x2, 0, core::mem::transmute(&mut credential))
    };
    if result.0 == 0 { println!("{:x?}", unsafe { GetLastError() } ); return Err("Credentials could not be read") } 

    let pass: String = c_str(credential.CredentialBlob).into();
    let user: String = c_str(credential.UserName.0).into();
    println!("Username: {}, Password: {}", user,pass);

    Ok("Credentials read sucessfully")
}

fn rust_store<'a>(user: String, pass: String, service: String) -> core::result::Result<(), &'a str> {

    let len = pass.len()+1;

    let username = (user+"\0").as_mut_ptr();
    let service = (service+"\0").as_mut_ptr();
    let password = (pass+"\0").as_mut_ptr();

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