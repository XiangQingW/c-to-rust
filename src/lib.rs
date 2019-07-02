#![crate_type = "staticlib"]
extern crate libc;

use libc::c_char;
use libc::uint8_t;
use std::ffi::{CStr, CString};
use std::mem::forget;
use std::str;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
enum HttpMethod {
    Get,
    Post,
}

#[repr(C)]
pub struct HttpHeader {
    name: *mut c_char,
    value: *mut c_char,
}

#[repr(C)]
pub struct HttpRequest {
    method: HttpMethod,
    url: *mut c_char,
    timeout: i32,

    body_len: u32,
    body: *mut uint8_t,

    header_len: u32,
    headers: *mut HttpHeader,
}

#[repr(C)]
pub struct HttpResponse {
    status_code: i32,
    unknown_err: *mut c_char,

    body_len: u32,
    body: *mut u8,

    header_len: u32,
    headers: *mut HttpHeader,
}

fn str2c_char(s: &str) -> *mut c_char {
    let s = CString::new(s).unwrap();
    s.into_raw()
}

fn c_char2str<'a>(c: *const c_char) -> &'a str {
    unsafe {
        let c = CStr::from_ptr(c);
        c.to_str().unwrap()
    }
}

#[no_mangle]
pub extern "C" fn send_http(req: *const HttpRequest) -> *const HttpResponse {
    unsafe {
        let r = &*req;

        let url = c_char2str(r.url);

        let body_len = r.body_len as usize;
        let body = Vec::from_raw_parts(r.body, body_len, body_len);

        let header_len = r.header_len as usize;
        let headers = Vec::from_raw_parts(r.headers, header_len, header_len);

        println!(
            "send http in rust: timeout= {:?} url= {} body= {:?} method= {:?}",
            r.timeout, url, body, r.method
        );

        for header in headers {
            let name = c_char2str(header.name);
            let value = c_char2str(header.value);

            println!("send http of header: name= {:?} value= {:?}", name, value);
        }

        let resp_body: Vec<u8> = vec![1, 2, 3];
        let resp_body_ptr = resp_body.as_ptr() as *mut u8;

        let mut resp_headers: Vec<HttpHeader> = vec![];

        let header1 = HttpHeader {
            name: str2c_char("resp_name1"),
            value: str2c_char("resp_value1"),
        };
        resp_headers.push(header1);

        let header2 = HttpHeader {
            name: str2c_char("resp_name2"),
            value: str2c_char("resp_value2"),
        };
        resp_headers.push(header2);

        let unknown_err = str2c_char("未知错误");
        let resp_headers_ptr = resp_headers.as_ptr() as *mut HttpHeader;

        let resp = Box::new(HttpResponse {
            status_code: 200,
            unknown_err,

            body: resp_body_ptr,
            body_len: resp_body.len() as u32,

            header_len: resp_headers.len() as u32,
            headers: resp_headers_ptr,
        });

        forget(resp_body);
        forget(resp_headers);

        Box::into_raw(resp)
    }
}

#[no_mangle]
pub extern "C" fn free_http_response(resp: *mut HttpResponse) {
    unsafe {
        if resp.is_null() {
            return;
        }

        println!("free http response successfully");
        let resp = Box::from_raw(resp);
        let _ = CString::from_raw(resp.unknown_err);

        let body_len = resp.body_len as usize;
        let _body = Vec::from_raw_parts(resp.body, body_len, body_len);

        let header_len = resp.header_len as usize;
        let headers = Vec::from_raw_parts(resp.headers, header_len, header_len);
        for header in headers {
            let _ = CString::from_raw(header.name);
            let _ = CString::from_raw(header.value);
        }
    }
}
