//! Process management syscalls
use core::mem::size_of;

use crate::{
    config::PAGE_SIZE, 
    mm::{translated_byte_buffer, MapPermission, VirtAddr}, 
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_task_status, get_task_syscall_times, get_task_time, suspend_current_and_run_next, task_mmap, TaskInfo,
        task_munmap}, 
    timer::get_time_us
};


#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
/// ref:https://sjodqtoogh.feishu.cn/docx/ZoqBdmcmAoXi9yxZUkucMmxBnzg 了解了translated_byte_buffer的具体用法
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us: usize = get_time_us();
    let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8,size_of::<TimeVal>());
    let res = TimeVal{
        sec:us/1000000,
        usec:us%1000000,
    };
    let mut res_ptr = &res as *const _ as *const u8;
    for buffer in buffers{
        unsafe{
            core::ptr::copy_nonoverlapping(res_ptr, buffer.as_mut_ptr(), buffer.len());
            res_ptr=res_ptr.add(buffer.len());
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
/// ref:https://sjodqtoogh.feishu.cn/docx/ZoqBdmcmAoXi9yxZUkucMmxBnzg 了解了translated_byte_buffer的具体用法
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let buffers = translated_byte_buffer(current_user_token(), _ti as *const u8,size_of::<TaskInfo>());
    let res = TaskInfo{
        status:get_task_status(),
        syscall_times:get_task_syscall_times(),
        time:get_task_time(),
    };
    let mut res_ptr = &res as *const _ as *const u8;
    for buffer in buffers{
        unsafe{
            core::ptr::copy_nonoverlapping(res_ptr, buffer.as_mut_ptr(), buffer.len());
            res_ptr=res_ptr.add(buffer.len());
        }
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start%PAGE_SIZE!=0 || _port&!0x7 !=0 || _port&0x7 == 0 {
        return -1;
    }
    let map_permission = MapPermission::from_bits(((_port|8)<<1) as u8).unwrap();
    task_mmap(VirtAddr::from(_start), VirtAddr::from(_start+_len), map_permission)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start%PAGE_SIZE!=0 {
        return -1;
    }
    task_munmap(VirtAddr::from(_start), VirtAddr::from(_start+_len))
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
