/*
Author: 闫刚 (yes7rose@sina.com)
run_handle.rs (c) 2020
Desc: 异步控制全局变量
Created:  2020-11-30T09:52:21.266Z
Modified: !date!
*/

use cash_core::results::*;
use tokio::runtime::Handle;

pub static mut TOKIO_RUNTIME_HANDLE: Option<Handle> = None;

pub fn set_runtime_handle(handle: Handle) -> Result<(), OperationResult> {
    unsafe {
        if TOKIO_RUNTIME_HANDLE.is_some() {
            println!("运行时只能设置一次");
            return Err(operation_failed("set_handle", "运行时只能设置一次"));
        } else {
            TOKIO_RUNTIME_HANDLE.replace(handle);
        }
    }
    Ok(())
}

pub fn get_runtime_handle() -> Handle {
    unsafe {
        if TOKIO_RUNTIME_HANDLE.is_some() {
            TOKIO_RUNTIME_HANDLE.clone().unwrap()
        } else {
            panic!("运行时没有初始化");
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
