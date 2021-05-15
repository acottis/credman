
fn get_ras_cred() {

    let domain: [CHAR; 16] = [CHAR(0u8); 16];
    let password: [CHAR; 257] = [CHAR(0u8); 257];
    let user: [CHAR; 257] = [CHAR(0u8); 257];

    let mut credentials = &mut RASCREDENTIALSA {
        dwMask: RASCM_Domain | RASCM_Password | RASCM_UserName,
        dwSize: core::mem::size_of::<RASCREDENTIALSA>() as u32,
        szDomain: domain,
        szPassword: password,
        szUserName: user,
    };

    let pbk = r"C:\Users\adamc\AppData\Roaming\Microsoft\Network\Connections\Pbk\rasphone.pbk";
    //let pbk = "%userprofile%\\AppData\\Roaming\\Microsoft\\Network\\Connections\\PBK\\rasphone.pbk\0".to_owned().as_mut_ptr();
    let entry = "remote.mst-uk.com\0".to_owned().as_mut_ptr();

    println!("{:?}", pbk);

    // let result = unsafe { 
    //     RasSetCredentialsA(PSTR(core::mem::zeroed()), PSTR(enty), credentials, true)
    // };
    let result = unsafe {
        RasGetCredentialsA(PSTR(core::mem::zeroed()), PSTR(entry), credentials)
    };
    //println!("{:x?}", unsafe { GetLastError() });
    //println!("{:?}", result);

    for char in credentials.szUserName.iter(){
        if char.0 == 0 { break }
            println!("{:?}", char.0)
    }

    //println!("{:?}", credentials.szUserName);

    
    //if result == 0 { println!("{:x?}", unsafe { GetLastError() } ); return} 
}