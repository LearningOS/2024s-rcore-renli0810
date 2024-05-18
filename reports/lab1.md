# Lab1 
## ch3 
1. 重新更换了TaskInfo的位置，从process.rs移到了task.rs，使得结构更加清晰,相应的需要修改一些地方。
2. 重新调整了TaskControlBlock内容，将TaskInfo加入了TaskControlBlock，重新定义了time的含义，修改为了该任务的起始时间。
3. 对于TaskInfo的更新首先需要在lazy_static进行初始化，status设定为ready，syscall_times初始化为MAX_SYSCALL_NUM大小全为0的数组，time通过get_time()获得该任务初始时间。
4. 对于TaskInfo中status的更新，直接用原有代码即可。
5. 对于TaskInfo中syscall_times的更新，首先在TASKMANGE导出一个接口statistics_current_syscall(),后续每次syscall就可以通过这个接口更新信息。
6. 对于TaskInfo中的各种访问，都可以使用TASKMANGE中导出的接口get_task_xxx()进行获取，其中对于time的获取，则可以通过get_time-TaskControlBlock中的起始时间。

## 简答
1. bad_address 页错误 访问了00000000地址
   bad_instruction 非法指令 U态使用了sret指令
   bad_register 非法指令 U态访问了CSR寄存器
   sbi版本：RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
2. 1. a0：用户态从哪里执行
   __restore: 系统调用内核态返回用户态，异常或者中断内核态返回用户态
   2. sstatus：用于恢复处理器状态
      sepc: 设置返回用户程序继续执行的地址
      sscratch: 设置用户态的栈指针
   3. x2 是栈指针，即sp，栈指针需要用于恢复其他指针
      x4 是线程指针，即tp，线程指针，对于单线程可以不用恢复
   4. 交换了sp和sscratch的值，sp现在为用户态栈指针，sscratch为内核态栈指针
   5. sret。执行完这条指令后，特权级修改为了U级，pc值调整为了sepc的值。
   6. 交换了sp和sscratch的值，sp现在为内核态栈指针，sscratch为用户态栈指针
   7. ecall

## 荣誉准则
我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。