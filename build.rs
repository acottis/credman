fn main(){
    windows::build!(
        Windows::Win32::Security::{CredFree, CredReadA, CredWriteA, CredDeleteW, CredEnumerateW,CREDENTIALW, CREDENTIALA, CRED_ENUMERATE_FLAGS},
        Windows::Win32::System::SystemServices::{PSTR, CHAR},
        Windows::Win32::System::WindowsProgramming::FILETIME,
        Windows::Win32::System::Diagnostics::Debug::GetLastError,
        Windows::Win32::NetworkManagement::Rras::{ RasGetCredentialsA, RasSetCredentialsA, RASCREDENTIALSA, RASCM_Password, RASCM_UserName, RASCM_Domain},
    )
}