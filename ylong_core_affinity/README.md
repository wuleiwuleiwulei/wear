## 根据不同操作系统进行绑定核心相关操作

### 对应不同操作系统自动调用底层函数，对 CPU 核心进行绑定操作

- `set_current_affinity` 设定当前线程的绑核 `cpu`，返回 0 表示成功，非 0 表示失败

    - 操作系统为 `windows` 的情况下：

      ```rust
      use ylong_core_affinity::set_current_affinity;
      
      let ret = set_current_affinity(0);
      assert_eq!(ret, -1);
      ```

    - 操作系统为 `linux` 的情况下：

      ```rust
      use ylong_core_affinity::set_current_affinity;
      
      let ret = set_current_affinity(0);
      assert_eq!(ret, 0);
      ```

- `get_current_affinity` 获取当前线程绑定的核心，若未曾进行绑核操作则返回所有可用的 `cpu`

    - 操作系统为 `windows` 的情况下：

      ```rust
      use ylong_core_affinity::get_current_affinity;
      
      let ret = get_current_affinity();
      assert_eq!(ret, vec![0 as usize]);
      ```

    - 操作系统为 `linux` 的情况下：

      ```rust
      use ylong_core_affinity::get_current_affinity;
      
      let ret = get_current_affinity();
      assert!(ret.len() > 0);
      ```

- `get_other_thread_affinity` 获取其他线程绑定的核，若未曾绑定核心则返回所有可用的 `cpu`

    - 操作系统为 `windows` 的情况下：

      ```rust
      use ylong_core_affinity::get_other_thread_affinity;
      
      let ret = get_other_thread_affinity();
      assert_eq!(ret, vec![0 as usize]);
      ```

    - 操作系统为 `linux` 的情况下：

      ```rust
      use ylong_core_affinity::get_other_thread_affinity;
      
      let ret = get_other_thread_affinity();
      assert!(ret.len() > 0);
      ```

    