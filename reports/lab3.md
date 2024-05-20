# lab3
## ch5
1. 将ch4的内容基本移植了过来，修改了一定的布局。
2. 对于spwan，主要就是把fork和exec结合了一下，思路为fork出一个新空间，然后执行elf代码。
3. 对于stride算法，在TCB中加入stride和priority后，直接在TaskManager里面修改找出stride最小的即可，(rust的大小堆有点难写，改了大半天还不如直接暴力解，一直报页缺失？)

## 简答
1. 不是的，p1的stride会溢出
2. $t==0，stride_{max}=0,stride_{min}=0,stride_{max}-stride_{min}=0<BigStride/2$
   $t==1, stride_{max}=BigStride/Prio,stride_{min}=0,stride_{max}-stride_{min}=BigStride/Prio<=BigStride/2$
   $假设t=tm，stride_{max}=s1,stride_{min}=s2,s1>s2,且s1-s2<=BigStride/2$
   $t=tm+1时 stride_{max}=max{s1,s2+BigStride/2},stride_{min}>=s2,s1>s2,则stride_{max}-stride_{min}=0<BigStride/2$
   通过归纳法即可证明

## 荣誉准则
在完成本次实验的过程（含此前学习的过程）中，我未与其他人就（与本次实验相关的）做过交流。但我参考了以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
https://sjodqtoogh.feishu.cn/docx/ZoqBdmcmAoXi9yxZUkucMmxBnzg
https://hangx-ma.github.io/2023/07/07/rcore-note-ch5.html
我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。