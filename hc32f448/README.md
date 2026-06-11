# HC32F448 驱动库的 Rust 绑定

此库为 HC32F448 单片机提供 C 库提供生成的 Rust 绑定与预构建静态库。

* 静态库使用 clang target=thumbv7m-none-eabihf mcpu=cortex-m4 opt_level=3 进行编译
* 部分宏定义（如晶振频率）修改为全局变量以适应不同的硬件配置
