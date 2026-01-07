# ylong_one_error
提供统一Error定义以及Error转换的功能。该模块提供了一个以下三个宏：
- define_error
- raise
- throw（不常用）

## define_error
定义一个名称为`Error`的enum，并为所列出的异常提供异常转换能力。

举个例子：我们需要从一个文件中读取一个u32的数据，然后做后续的处理动作。

### 第一种做法-全unwrap

```rust
fn read_value(path: &str) -> u32 {
    let path = std::path::Path::new(path);
    let content = std::fs::read_to_string(path).unwrap();
    content.parse::<u32>().unwrap()
}

fn main() {
    let value = read_value("");
    println!("{}", value);
}
```

这种做法会有一些问题，比如：
- 路径不存在，程序会panic，然后挂掉
- 路径存在但是没权限，程序会panic，然后挂掉
- 路径存在有权限但是内容不是数字，程序会panic，然后挂掉

为了保证应用正常，大部分业务不能这么处理，所以需要将异常做处理

### 第二种做法-吃掉异常

```rust
fn read_value(path: &str) -> Option<u32> {
    let path = std::path::Path::new(path);
    if let Ok(content) = std::fs::read_to_string(path) {
        if let Ok(value) = content.parse::<u32>() {
            return Some(value);
        }
    }
    None
}

fn main() {
    let value = read_value("");
    println!("{}", if value.is_some() { value.unwrap() } else { 0 });
}
```

这种做法在异常场景确实不会panic了，但是有个更大的问题，那就是发生了问题就没有办法定位出具体的出错原因。

虽然可以通过打日志来解决，但是如果上层业务需要做一些适配工作，比如：
- 文件不存在时就换个文件
- 内容不正确时就使用另外的配置数据

这种情况上层业务是必须需要知道Error信息的，所以Error需要返回。


### 第三种做法-自定义Error+Result

```rust
#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

fn read_value(path: &str) -> Result<u32, Error> {
    let path = std::path::Path::new(path);
    let content = std::fs::read_to_string(path)?;
    Ok(content.parse::<u32>()?)
}

fn main() {
    let value = read_value("");
    match value {
        Ok(v) => println!("read value successfully, value: {}", v),
        Err(err) => match err {
            Error::IO(err) => println!("read value got io-error, err: {}", err),
            Error::ParseInt(err) => println!("parse value got error, err: {}", err),
        },
    }
}
```

这种写法确实让业务的处理可以更灵活，但是也带来了新的问题
- 第一，需要编写Error以及Error的转换，需要编写更多的代码
- 第二，异常只有具体的信息，如果在多很多层级的调用，同样的异常分不清是哪个地方触发的，这对定位而言就是一个完全不能接收的事情




### 第四种做法-使用define_error

[样例代码](examples/define_error_demo.rs):define_error_demo_code

```rust
use ylong_one_error::*;

define_error! {
    (IO,std::io::Error),
    (ParseInt,std::num::ParseIntError)
}

fn read_value(path: &str) -> Result<u32, Error> {
    let path = std::path::Path::new(path);
    let content = std::fs::read_to_string(path)?;
    Ok(content.parse::<u32>()?)
}

fn main() {
    let value = read_value("");
    match value {
        Ok(v) => println!("read value successfully, value: {}", v),
        Err(err) => match err {
            Error::IO(err) => println!("read value got io-error, err: {}", err),
            Error::ParseInt(err) => println!("parse value got error, err: {}", err),
            Error::CustomMessage(_) => todo!(),
        },
    }
}
```

只需要使用`define_error`这个宏来定义Error，业务代码保持和第三种一致，这种做法
- 可以少写很多代码，因为相关的转换自动生成了
- 异常发生时会显示具体的行号，更方便定位

## raise

返回一个基于字符串的Error实例

举个例子：检验函数的入参，如果这个值大于100就报错

[样例代码](examples/raise_demo.rs)

```rust
fn give_me_five(value: u32) -> Result<u32, Error> {
    const MAX_VALUE: u32 = 100;
    if value > MAX_VALUE {
        raise!("value can not lesser than {}, value: {}", MAX_VALUE, value)
    }
    Ok(value + 5)
}


fn main() {
     match give_me_five(110) {
        Ok(v) =>  println!("thank you, value: {}", v),
        Err(err) => println!("so sad, something wrong, error: {}", err),
    }
}
```

在值大于100时，则会直接报错，而且支持Result
