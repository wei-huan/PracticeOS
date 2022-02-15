# 实验3



## 编程作业



### stride 调度算法

ch3 中我们实现的调度算法十分简单。现在我们要为我们的 os 实现一种带优先级的调度算法：stride 调度算法。

算法描述如下:

1. 为每个进程设置一个当前 stride，表示该进程当前已经运行的“长度”。另外设置其对应的 pass 值（只与进程的优先权有关系），表示对应进程在调度后，stride 需要进行的累加值。
2. 每次需要调度时，从当前 runnable 态的进程中选择 stride 最小的进程调度。对于获得调度的进程 P，将对应的 stride 加上其对应的步长 pass。
3. 一个时间片后，回到上一步骤，重新调度当前 stride 最小的进程。

可以证明，如果令 P.pass = BigStride / P.priority 其中 P.priority 表示进程的优先权（大于  1），而 BigStride  表示一个预先定义的大常数，则该调度方案为每个进程分配的时间将与其优先级成正比。证明过程我们在这里略去，有兴趣的同学可以在网上查找相关资料。

其他实验细节：

- stride 调度要求进程优先级

，所以设定进程优先级

 会导致错误。

进程初始 stride 设置为 0 即可。

进程初始优先级设置为 16。





## 简答作业

stride 算法深入



stride 算法原理非常简单，但是有一个比较大的问题。例如两个 pass = 10 的进程，使用 8bit 无符号整形储存  stride， p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。

- 实际情况是轮到 p1 执行吗？为什么？

  不会，p2溢出，溢出后是4更小。



我们之前要求进程优先级 = 2 其实就是为了解决这个问题。可以证明，**在不考虑溢出的情况下**, 在进程优先级全部 = 2 的情况下，如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。

- 为什么？尝试简单说明（传达思想即可，不要求严格证明）。

  退化成Round Robin了，肯定是依次执行，没有一个任务会比其他任务多执行超过1次。



已知以上结论，**考虑溢出的情况下**，我们可以通过设计 Stride 的比较接口，结合 BinaryHeap 的 pop 接口可以很容易的找到真正最小的 Stride。

- 请补全如下 `partial_cmp` 函数（假设永远不会相等）。

```rust
 use core::cmp::Ordering;

 struct Stride(u64);

 impl PartialOrd for Stride {
     fn partial_cmp(&self, other: &Self) - Option<Ordering> {
         // ...
     }
 }

 impl PartialEq for Person {
     fn eq(&self, other: &Self) - bool {
         false
     }
 }
```

 例如使用 8 bits 存储 stride, BigStride = 255, 则:

 - (125 < 255) == false
 - (129 < 255) == true
