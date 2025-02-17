# 开发一个 solana 程序

#### 核心 crate

solana-program = “2.2.1”

#### 入口函数声明

```rust
use solana_program::entrypoint; 
// 我们需要的 entrypoint! 是个宏，
// 所以声明时最好单独提取出来，不然和下方的声明在一起为 
// use solana_program::entrypoint{self,ProgramResult} 的话会报错

use solana_program::entrypoint::ProgramResult; 
// ProgramResult 是 solana 中定义的一个通用错误处理类型，可以用于入口函数的返回类型
// 成功返回 ()，失败返回 ProgramError；ProgramError也是个枚举

use solana_program::program_error::ProgramError; 
// ProgramError 中定义了 23 种常见的错误原因枚举值，也支持自定义的错误类型

use solana_program::account_info::AccountInfo; 
// account_info 模块中的一个结构体，允许我们访问帐户信息

use solana_program::pubkey::Pubkey;
// pubkey 模块中的一个结构体，允许我们将地址作为公钥访问

use solana_program::msg;
// 一个允许我们将消息打印到程序日志的宏，类似于 Rust 中的 println宏

entrypoint!(process_instruction);

fn process_instruction(
    // 当前程序 id
    program_id: &Pubkey,
    // 指令涉及的账户集合
    accounts: &[AccountInfo],
    // 指令的参数
    instruction_data: &[u8],
) -> ProgramResult {
    // ... 逻辑处理
    Ok(())
}
```

### 数据账户定义

```rust
use borsh::{BorshDeserialize, BorshSerialize};
// 由于 solana 在发起交易指令时，值的存储和传输使用的是字节码，所以需要把字节码转为 Rust 类型
// 所以这里需要 BorshDeserialize 和 BorshSerialize 这两个派生宏来实现 (反)序列化
// 它们都是对解析后类型为TokenStream的 Rust 代码元数据进行处理，并返回处理后的元数据

/// 定义数据账户的结构
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u32, // 使用 count 来计数，当账户发起交易指令时 count += 1
}
```

borsh 需要在 Cargo.toml 中引入

```toml
borsh = "1.5.1"
```

### 指令处理

#### 获取账户

为了处理指令，指令所需的数据账户必须通过 accounts 参数显式传递到程序中

因为要对数据账户进行累加的操作，所以 accounts 包含了该数据账户，我们可以通过迭代器获取到该账户 account

```rust
use solana_program::account_info::next_account_info;

pub fn process_instruction(
    // 程序ID，即程序地址
    program_id: &Pubkey,
    // 该指令涉及到的账户集合
    accounts: &[AccountInfo]) -> ProgramResult 
{
    // 账户迭代器
    let accounts_iter = &mut accounts.iter();
    // 获取调用者账户
    let account = next_account_info(accounts_iter)?;

    // ……
    Ok(())
}
```

#### 账户权限校验

account数据账户是由该程序派生出来的账户，因此当前程序为它的owner所有者，并且只有所有者才可以对其进行写操作。所以我们在这里要进行账户权限的校验

```rust
use solana_program::account_info::next_account_info;

pub fn process_instruction(
    // 程序ID，即程序地址
    program_id: &Pubkey,
    // 该指令涉及到的账户集合
    accounts: &[AccountInfo]) -> ProgramResult 
{
    // 使用accounts的迭代器获取账户
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
	
    // 权限校验，验证调用者身份
    if account.owner != program_id{
        msg!("account 的所有者不属于当前 program_id");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    // ……
    Ok(())
}
```

#### 读取数据账户

权限校验 ok 后，接下来就是读取账户存储的数据

这时候，我们就需要从 account 数据账户中反序列化出 CounterAccount 结构体的实例，这样就能读取到该账户的数据了

```rust
pub fn process_instruction(...) -> ProgramResult{
	// 获取账户
	// 权限校验
	// 反序列化获取 counter
	let mut counter = CounterAccount::try_from_slice(&account.data.borrow())?;
}
```

- **`&account.data`**：获取账户的数据字段的引用。在 Solana 中，账户的数据字段 **`data`** 存储着与账户关联的实际数据，对于程序账户而言，它是程序的二进制内容，对于数据账户而言，它就是存储的数据。

- **`borrow()`**：使用该方法获取data数据字段的不可变引用。并通过 **`&account.data.borrow()`** 方式得到账户数据字段的不可变引用。

- **`CounterAccount::try_from_slice(...)`**：调用 ==try_from_slice== 方法，它是 **`BorshDeserializetrait`**  的一个方法，==用于从字节序列中反序列化出一个结构体的实例==。这里 **`CounterAccount`** 实现了 **`BorshDeserialize`** ，所以可以使用这个方法。

- **`?`**：是一个错误处理操作符，如果 **`try_from_slice`** 返回错误，整个表达式将提前返回，将错误传播给调用方。

通过如上方式，我们获取了 **`CounterAccount`** 数据账户进行了反序列化，并获取到它的可变借用

#### 修改数据账户

获取到数据账户反序列化后的可变借用，就可以对数据账户信息进行修改

修改后再序列化为字节数组并写入到 solana 账户的可变数据字段中

```rust
pub fn process_instruction(...) -> ProgramResult{
	// 获取账户
	// 权限校验
	// 反序列化获取 counter
	let mut counter = CounterAccount::try_from_slice(&account.data.borrow())?;
    // counter 的 count 计数 +1
    counter.count +=1;
    // 序列化 counter 为字节数组 并写入 数据账户的 data
    counter.serialize(&mut *account.data.borrow_mut())?;
    Ok(())
}
```

- 首先对 **`CounterAccount`** 结构体中的 **`count`** 字段进行递增操作。

- **`&mut *account.data.borrow_mut()`**：通过 **`borrow_mut()`** 方法获取账户数据字段的可变引用，然后使用 **`*`** 解引用操作符获取该 **`data`** 字段的值，并通过 **`&mut`** 将其转换为可变引用。

- ==serialize== 函数方法，它是 **`BorshSerialize`** trait 的一个方法，用于将结构体序列化为字节数组。

- **`?`**：是一个错误处理操作符，如果 **`serialize`** 方法返回错误，整个表达式将提前返回，将错误传播给调用方。

通过如上的方式，将 **`CounterAccount`** 结构体中的修改后的值递增，并将更新后的结构体序列化为字节数组，然后写入 Solana 账户的可变数据字段中。实现了在 Solana 程序中对计数器值进行更新和存储。