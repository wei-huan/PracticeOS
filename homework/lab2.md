# Lab2



## 实验要求

- 实现分支: ch2-lab

- 目录要求不变

- 为 sys_write 增加安全检查

  在 os 目录下执行 `make run TEST=1` 测试 `sys_write` 安全检查的实现，正确执行目标用户测例，并得到预期输出（详见测例注释）。

  注意：如果设置默认 log 等级，从 lab2 开始关闭所有 log 输出。

challenge: 支持多核，实现多个核运行用户程序。



## 增加的内容

增加单核安全静态变量模块，用户任务管理模块，特权级切换功能，异常处理功能



## 简答题

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。请自行测试这些内容 (运行 Rust 三个 bad 测例 ) ，描述程序出错行为，注明你使用的 sbi 及其版本。

​		

2. 请结合用例理解 [trap.S](https://github.com/rcore-os/rCore-Tutorial-v3/blob/ch2/os/src/trap/trap.S) 中两个函数 `__alltraps` 和 `__restore` 的作用，并回答如下几个问题：

   

   1. L40：刚进入 `__restore` 时，`a0` 代表了什么值。请指出 `__restore` 的两种使用情景。
      
      a0: 内核栈压入 Trap 上下文之后的栈底
      情形1：返回U态
      情形2：进行下一个任务
      
      
      
   2. L46-L51：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

      ```
      ld t0, 32*8(sp)
      ld t1, 33*8(sp)
      ld t2, 2*8(sp)
      csrw sstatus, t0
      csrw sepc, t1
      csrw sscratch, t2
      ```

      sstatus:`SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息

      sepc: 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址

      sscratch: 副栈，用户态时为内核栈地址

      

   3. L53-L59：为何跳过了 `x2` 和 `x4`？

      ```
      ld x1, 1*8(sp)
      ld x3, 3*8(sp)
      .set n, 5
      .rept 27
         LOAD_GP %n
         .set n, n+1
      .endr
      ```

      x2是栈顶指针，得把全部内容都恢复完了才能恢复栈，不然会恢复错误的内容

      x4是线程指针，得全部上下文都恢复完才能转到新的线程

      

   4. L63：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

      ```
      csrrw sp, sscratch, sp
      ```

      sp为用户栈指针，sscratch为内核栈指针。

      

   5. `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

      sret。不知道。

      

   6. L13：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

      ```
      csrrw sp, sscratch, sp
      ```

      sp为内核栈指针，sscratch为用户栈指针。

      

   7. 从 U 态进入 S 态是哪一条指令发生的？

      

3. 程序陷入内核的原因有中断和异常（系统调用），请问 riscv64 支持哪些中断 / 异常？如何判断进入内核是由于中断还是异常？描述陷入内核时的几个重要寄存器及其值。

   

   异常一览表

   | Interrupt | Exception Code | Description                    |
   | --------- | -------------- | ------------------------------ |
   | 0         | 0              | Instruction address misaligned |
   | 0         | 1              | Instruction access fault       |
   | 0         | 2              | Illegal instruction            |
   | 0         | 3              | Breakpoint                     |
   | 0         | 4              | Load address misaligned        |
   | 0         | 5              | Load access fault              |
   | 0         | 6              | Store/AMO address misaligned   |
   | 0         | 7              | Store/AMO access fault         |
   | 0         | 8              | Environment call from U-mode   |
   | 0         | 9              | Environment call from S-mode   |
   | 0         | 11             | Environment call from M-mode   |
   | 0         | 12             | Instruction page fault         |
   | 0         | 13             | Load page fault                |
   | 0         | 15             | Store/AMO page fault           |

   

   中断：时钟中断，软中断和硬中断



​		看scause寄存器的值



| CSR 名   | 该 CSR 与 Trap 相关的功能                                    |
| -------- | :----------------------------------------------------------- |
| sstatus  | `SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级（S/U）等信息 |
| sepc     | 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址 |
| scause   | 描述 Trap 的原因                                             |
| stval    | 给出 Trap 附加信息                                           |
| stvec    | 控制 Trap 处理代码的入口地址                                 |
| sscratch | 副栈，用户态时为内核栈地址，特权级别时为用户栈地址           |



2. 对于任何中断，`__alltraps` 中都需要保存所有寄存器吗？你有没有想到一些加速 `__alltraps` 的方法？简单描述你的想法。

   不是。如果是进行下一个任务或者任务被迫终止的话，就基本上不需要保存当前任务的内容。

   

   可以根据异常类型判断是否需要保存普通寄存器的值。

​		
