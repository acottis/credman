mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::Security::*,
    Windows::Win32::System::SystemServices::PWSTR,
    Windows::Win32::System::WindowsProgramming::FILETIME,
};

use std::{borrow::BorrowMut};

// fn main() {
//     store("Adam", "test", "Azphel:CredMan").unwrap();
//     //windows_rs()
// }


type c_str = *const [u8];

fn cstr_into_str(cstr: *const [u8; 255]) -> Result<&'static str, ()>{
    unsafe{
        if (*cstr)[0] == '\0' as u8 { return Err(()) }

        let mut length = 0;
        for chr in (cstr.as_ref().unwrap()).into_iter(){
            if *chr == '\0' as u8 { break }
            length += 1;
        }

        return Ok(std::str::from_utf8(&cstr.as_ref().unwrap()[0..length]).unwrap())
    }
}


#[no_mangle]
extern "C" fn store(user: *const [u8; 255], pass: *const [u8; 255], service: *const [u8; 255]) -> i32{

    let user = cstr_into_str(user).unwrap();
    let pass = cstr_into_str(pass).unwrap();
    let service = cstr_into_str(service).unwrap();
    // println!("{:?} {:?} {:?}", user, pass, service );

    rust_store(user, pass, service).expect("Could not store credentials");
    //rust_store("Adam", "test", "Azphel:CredMan").expect("Could not store credentials");
    println!("We made it!");
    0
}

#[no_mangle]
fn rust_store<'a>(user: &str, pass: &str, service: &str) -> core::result::Result<(), &'a str> {

    let comm = "Internal Test comment";

    let mut username: Vec<u16> = (user.to_owned()+"\0").encode_utf16().collect();
    let mut service: Vec<u16> = (service.to_owned()+"\0").encode_utf16().collect();
    let password = (pass.to_owned()+"\0").to_string().as_mut_ptr();
    let mut comment: Vec<u16> = (comm.to_owned()+"\0").encode_utf16().collect();

    let mut credential= CREDENTIALW{
        Flags: CRED_FLAGS(0),
        Type: CRED_TYPE(1),
        Persist: CRED_PERSIST(0x2),
        LastWritten: FILETIME::default(),
        UserName: PWSTR(username.as_mut_ptr()),
        TargetName: PWSTR(service.as_mut_ptr()),
        Comment: PWSTR(comment.as_mut_ptr()),
        CredentialBlob: password,
        CredentialBlobSize: pass.len() as u32,
        Attributes: std::ptr::null_mut(),
        AttributeCount: 0,
        TargetAlias: PWSTR(service.as_mut_ptr()),
    };
    let result = unsafe { CredWriteW(credential.borrow_mut(), 0) };
    if result.0 == 0 { return Err("Failed to store credentials.") }
    Ok(())
}

#[no_mangle]
fn add(a: u32,b: u32) -> u32{
    a + b
}