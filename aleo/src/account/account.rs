use crate::c_error;
use rand::{rngs::StdRng, SeedableRng};

use snarkvm_console::account::{Address, ComputeKey, PrivateKey, Signature, ViewKey};
use snarkvm_console::network::{prelude::*, MainnetV0};


use std::ffi::{CString, CStr};
use std::slice;

use sha2::{Sha256, Digest};

#[no_mangle]
pub extern "C" fn pk_str_from_str(pk_str: *const libc::c_char) -> *mut libc::c_char {
    let pk_str = unsafe {
        assert!(!pk_str.is_null());
        CStr::from_ptr(pk_str).to_str().unwrap()
    };
    let private_key = PrivateKey::<MainnetV0>::from_str(pk_str).unwrap();
    let pk_str = private_key.to_string();
    CString::new(pk_str).unwrap().into_raw()
}
#[no_mangle]
pub extern "C" fn pk_from_seed(n: *const u8, len: libc::size_t) -> *mut libc::c_char {
    let buf = unsafe {
        assert!(!n.is_null());
        std::slice::from_raw_parts(n, len as usize)
    };
    let hash = Sha256::digest(buf);
    let seed: [u8; 32] = hash.try_into().unwrap();
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let private_key = PrivateKey::<MainnetV0>::new(&mut rng);
    let pk_str = private_key.unwrap().to_string();
    CString::new(pk_str).unwrap().into_raw()
}

#[test]
fn test_seed(){
    // 创建一个512位(64字节)的数组
    let mut data: [u8; 64] = [0; 64];

    // 填充数组with some data
    for i in 0..64 {
        data[i] = i as u8;
    }

    // 获取数组的裸指针
    let ptr = data.as_ptr();

    // 使用 from_raw_parts 创建一个切片
    let slice = unsafe {
        slice::from_raw_parts(ptr, 64)
    };
    println!("{:?}", slice);
    println!("{}",slice.len());
  //  let mut rng: StdRng = SeedableRng::from_seed(slice.try_into().unwrap());
    let hash = Sha256::digest(slice);
    let seed: [u8; 32] = hash.try_into().unwrap();
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    println!("{:?}",rng);
    let private_key = PrivateKey::<MainnetV0>::new(&mut rng);
    let pk_str = private_key.unwrap().to_string();
    println!("{}",pk_str);

}
#[no_mangle]
fn pk_struct_from_str(pk_str: *const libc::c_char) -> PrivateKey<MainnetV0> {
    let pk_str = unsafe {
        assert!(!pk_str.is_null());
        CStr::from_ptr(pk_str).to_str().unwrap()
    };
    PrivateKey::<MainnetV0>::from_str(pk_str).unwrap()
}

#[no_mangle]
pub extern "C" fn vk_str_from_pk(pk_str: *const libc::c_char) -> *mut libc::c_char {
    let pk_s = pk_struct_from_str(pk_str);
    let view_key = ViewKey::<MainnetV0>::try_from(&pk_s).unwrap();
    let vk_str = view_key.to_string();
    CString::new(vk_str).unwrap().into_raw()
}


#[no_mangle]
pub extern "C" fn add_str_from_pk(pk_str: *const libc::c_char) -> *mut libc::c_char {
    let pk_s = pk_struct_from_str(pk_str);
    let add = Address::<MainnetV0>::try_from(&pk_s).unwrap();
    let add_str = add.to_string();
    CString::new(add_str).unwrap().into_raw()
}


#[no_mangle]
pub extern "C" fn free_rust_string(s: *mut libc::c_char) {
    unsafe {
        if s.is_null() { return; }
        CString::from_raw(s)
    };
}

#[test]
fn test_rng() {

    //fn pk_from_seed(n: *const u8, len: libc::size_t)
    //pk_from_seed(,size_t(32))

}


/*#[test]
fn test_vk(){
    let private_key = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";
    let res = vk_str_from_pk(private_key);
    // 安全地将 C 字符串转换为 Rust 字符串
    let rust_string = unsafe {
        assert!(!res.is_null());
        CStr::from_ptr(res).to_string_lossy().into_owned()
    };
    println!("pk_str: {}", rust_string);
    // 释放 C 字符串的内存
    unsafe {
        CString::from_raw(res);
    }
}
*/

/*#[test]
fn test_add(){
    let private_key = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";
    let res = add_str_from_pk(private_key);
    // 安全地将 C 字符串转换为 Rust 字符串
    let rust_string = unsafe {
        assert!(!res.is_null());
        CStr::from_ptr(res).to_string_lossy().into_owned()
    };
    println!("pk_str: {}", rust_string);
    // 释放 C 字符串的内存
    unsafe {
        CString::from_raw(res);
    }
}

#[test]
fn test_pk(){
    let private_key = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";
    let res = pk_str_from_str(private_key);
    // 安全地将 C 字符串转换为 Rust 字符串
    let rust_string = unsafe {
        assert!(!res.is_null());
        CStr::from_ptr(res).to_string_lossy().into_owned()
    };
    println!("pk_str: {}", rust_string);
    // 释放 C 字符串的内存
    unsafe {
        CString::from_raw(res);
    }
}
*/

#[test]
fn test_Account2() {
    let private_key = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";
    let private_key = PrivateKey::<MainnetV0>::from_str(private_key).unwrap();
    let view_key = ViewKey::<MainnetV0>::try_from(&private_key).unwrap();
    let address = Address::<MainnetV0>::try_from(&private_key).unwrap();
    println!("private_key: {}", private_key);
    println!("view_key: {}", view_key);
    println!("Address: {}", address);
}





/*#[test]
fn test_privateKey() {
    let private_key = "APrivateKey1zkp8cC4jgHEBnbtu3xxs1Ndja2EMizcvTRDq5Nikdkukg1p";

    let pk_ptr = pkStructFromStr(private_key);
    unsafe {
        let private_key = &*pk_ptr;
        println!("Private Key: {:?}", private_key);
    }
}
*/



