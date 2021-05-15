fn main(){
    windows::build!(
        Windows::Win32::Security::{CredFree, CredReadA, CredWriteA, CredDeleteW, CredEnumerateW,CREDENTIALW, CREDENTIALA, CRED_ENUMERATE_FLAGS},
        Windows::Win32::System::SystemServices::{PWSTR,PSTR},
        Windows::Win32::System::WindowsProgramming::FILETIME,
        Windows::Win32::System::Diagnostics::Debug::GetLastError,
    )
}