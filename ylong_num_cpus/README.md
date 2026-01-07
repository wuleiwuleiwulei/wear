## 根据不同操作系统获取当前 CPU 可用/禁用核心数目

### 对应不同操作系统自动调用底层函数，获取可用状态的 CPU 核心数目

- `get_cpu_num` 的基本使用操作

    - 操作系统为 `windows` 的情况：

      ```rust
      use ylong_num_cpus::get_cpu_num;
      
      let num = get_cpu_num();
      println!("cpu_num is {}", num);
      ```

      输出结果：

      ```rust
      cpu_num is -1
      ```

    - 操作系统为 `linux` 的情况：

      ```rust
      use ylong_num_cpus::get_cpu_num;
      
      let num = get_cpu_num();
      println!("cpu_num is {}", num);
      ```

      输出结果：

      ```rust
      cpu_num is 8
      ```

- `get_cpu_num_configured` 的基本使用操作

    - 操作系统为 `linux` 的情况

      ```rust
      use ylong_num_cpus::linux::get_cpu_num_configured;
      
      let num = get_cpu_num_configured();
      println!("cpu_num is {}", num);
      ```

      输出结果：

      ```rust
      cpu_num is 8
      ```

    

