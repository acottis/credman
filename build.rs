fn main(){
    windows::build!(
        Windows::Win32::Security::{CredReadW, CredWriteW, CredDeleteW, CredEnumerateW,CREDENTIALW, CRED_ENUMERATE_FLAGS},
        Windows::Win32::System::SystemServices::PWSTR,
        Windows::Win32::System::WindowsProgramming::FILETIME,
    )
}