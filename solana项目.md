# solana 猜数游戏

## 流程

- Solana 程序初始化

- 实现生成随机数功能

- 定义玩家账户

- 实现玩家猜数功能

- 将游戏部署到 Solana 测试网络

### 创建项目

```
anchor init anchor_bac
```

### 导入 anchor 库

```rust
use anchor_lang::prelude::*;
```

### 确定 program_id

#### 使用 **declare_id!** 宏	

```rust
declare_id!("AKUfpJjGJLQuYbVQ6Ny7CxdtCqtmXpk3gJgbUx5P1g68")
```

在 anchor 框架中，我们使用 **declare_id!** 宏来指定程序的链上地址，当构建一个新的 anchor 程序时，框架会生成一个新的密钥对。这个密钥对的公钥就是程序的 **program_id**

通常情况下每次使用 anchor 框架去创建程序的时候，**program_id** 都会有所不同。但是我们可以通过 **declare_id!** 宏为程序指定固定的 **program_id** 

### 定义 solana 程序模块

#### 使用 **#[program]** 宏

```rust
#[program]
```

使用 **#[program]** 宏来定义一个 solana 程序模块，它包含了程序的相关指令和其它相关的操作函数

#### 定义 anchor_bac 模块

```rust
#[program]
pub mod anchor_bac {
	
}
```

声明为 **pub** 确保模块的内容可以被外部访问和使用

#### 导入父模块内容

```rust
#[program]
pub mod anchor_bac {
	use super::*;
}
```

### 定义生成随机数函数

```rust
fn generate_number() -> u32 {
	
}
```

该函数会返回一个 u32 类型的整数，这个数就是我们需要的随机数

随机数生成逻辑：为了简化逻辑，我们使用 solana 程序库获取当前时间的时间戳，然后使用该时间戳来生成一个随机数

- **导入 Clock 依赖**

  ```
  use solana_program::clock::Clock;
  ```

  Cargo.toml 也要导入相关依赖

  ```
  solana-program = "2.2.1"
  ```

  **时间信息**: **Clock** 模块提供了当前的 **UNIX** 时间戳，即自**1970年1月1日**以来的秒数

  **安全性**: 尽管 **Clock** 模块可以方便地提供当前的时间信息，但它提供的时间是区块链节点共识的结果，而不是一个绝对准确的全球时间源，因此，依赖时间的逻辑有可能会受到恶意操作者利用时间操纵来进行攻击。 

  **注意⚠️**：这里时为了简化理解，我们才使用 **Clock** 来实现一个简易的随机数生成示例

- **获取当前时间**

  ```rust
  fn generate_number()-> u32 {
      let clock = Clock::get().expect("生成随机数时，获取 unix 时间戳失败");
      let last_digit = (clock.unix_timestamp % 10) as u8;
  }
  ```

- **生成随机数**

  ```rust
  fn generate_number()-> u32 {
      let clock = Clock::get().expect("生成随机数时，获取 unix 时间戳失败");
      let last_digit = (clock.unix_timestamp % 10) as u8;
      let random_number = (last_digit+1) as u32;
      random_number
  }
  ```

### 定义一个储存猜数数据的结构体

这个结构体存储程序随机数生成的数字，也可以用来记录玩家的猜数，后续就可以用来对比玩家猜数是否正确

#### 定义结构体

创建一个名为 **GuessingAccount** 的结构体

```rust
#[account]
pub struct GuessingAccount{

}
```

 Solana 作为一个分布式区块链系统，所有的信息都存储在账户中，如程序代码、状态信息、Token数据、配置信息等都是存储在一个个账户中

因此，我们要定义记录数据的结构体，也需要用 **#[account]** 标记为 Solana 的账户类型，这样就可以在链上存储游戏要记录的数字

**#[account]** 将结构体定义为账户类型，使得结构体能够映射到区块链上的一个账户，存储所需的状态信息，并通过合约中的函数进行访问和修改，同时自动处理数据的序列化、反序列化和验证

#### 添加字段

添加一个猜数字段，用来记录玩家所猜测的数字

```rust
#[account]
pub struct GuessingAccount{
	pub random_number: u32,
}
```

### 定义程序账户结构体

用于管理程序交互过程中的账户状态

#### 创建结构体

**AccountContext** 结构体将包含所有必要的账户引用，用于程序的执行。首先，我们定义结构体并使用 **#[derive(Accounts)]** 宏

```rust
#[derive(Accounts)]
pub struct AccountContext<'info>{
    // 字段定义在这
}
```

- **#[derive(Accounts)]**：这个派生宏可以实现对给定结构体数据的反序列化，自动生成账户等操作。有了这个派生宏，在获取账户时不再需要手动迭代账户以及反序列化操作，并且实现了账户满足程序安全运行所需要的安全检查

- **<'info>**：生命周期参数，用于指定结构体中引用的有效期

#### 添加一个玩家猜数的字段，用来传递玩家的猜数

往 **AccountContext** 结构体中添加一个名为 **guessing_account** 的字段

```rust
#[derive(Accounts)]
pub struct AccountContext<'info> {
    #[account(
        init_if_needed,
        space=8+4,
        payer=payer,
        seeds = [b"guessing pda"],
        bump
    )]
    pub guessing_account: Account<'info, GuessingAccount>,
}
```

- **guessing_account: Account<'info, GuessingAccount>**：
  - **<'info>**：生命周期参数，声明引用在整个结构体生命周期内都是有效的
  - **GuessingAccount**：类型参数，指定了这个账户将持有的数据类型
- 这里还使用了 **#[account]** 宏，用来配置 PDA 账户的各种属性，如初始化方式，占用空间大小，付款账户等
  - **init_if_needed**：通知 Anchor 框架在需要时自动初始化一个派生账户地址 PDA。如果账户尚未初始化，Anchor 会根据提供的其他参数（如 space 和 payer ）来初始化它
  - **space=8+4**：指定账户的空间大小为 **8+4** 个字节，前 ***8*** 个字节为账户类型识别器，用于识别帐户类型，这样 Anchor 就能对账户进行（反）系列化；后 *4* 个字节为存储在 **GuessingAccount** 帐户类型中的数据分配空间（ **number** 为 **u32** 类型，占用 ***4*** 字节）
  - **payer=payer**：指定了支付账户
  - **seeds = [b'guessing pda'], bump**：需要创建 DPA 账户时，会结合 **program_id** 生成一个 PDA 账户

#### 添加 payer 字段，表示付款账户

**添加** **payer** **字段：**表示执行智能合约所需的付款账户

```rust
#[derive(Accounts)]
pub struct AccountContext<'info> {
    #[account(
        init_if_needed,
        space=8+4,
        payer=payer,
        seeds = [b"guessing pda"],
        bump
    )]
    pub guessing_account: Account<'info, GuessingAccount>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
}
```

- **\#[account(mut)]**：表示 **payer** 是一个可变的账户引用，这是因为执行合约时可能会修改账户状态（例如，扣除手续费）。
- **Signer<'info>**：**payer** 是 **Signer** 类型，表示对该笔交易进行签名的账户

#### 添加 system 字段，表示 solana 系统程序的引用

**添加** **system_program** **字段：**表示 Solana 系统程序的引用，它提供了执行合约所需的一些基础功能

```rust
#[derive(Accounts)]
pub struct AccountContext<'info> {
    #[account(
        init_if_needed,
        space=8+4,
        payer=payer,
        seeds = [b"guessing pda"],
        bump
    )]
    pub guessing_account: Account<'info, GuessingAccount>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
```

### 定义初始化随机数的函数

#### **创建函数**

```rust
pub fn initialize(ctx: Context<AccountContext>) -> Result<()> {
    // 函数体将在此实现
}
```

- **Context**：是 Anchor 框架中定义的一个结构体，用于封装与 Solana 程序执行相关的上下文信息。

- **ctx**：接受一个类型为 **`Context<AccountContext>`**的参数，表示当前的程序上下文。**AccountContext** 指定了指令函数所需要的账户集合。

- **Result<()>**：返回一个 **Result** 类型，其中 **()** 表示成功时不返回任何值，而错误情况将返回 **Err**

#### 为游戏生成一个随机数字

**设置目标数字**

```rust
pub fn initialize(ctx: Context<AccountContext>) -> Result<()> {
   let guessing_account = &mut ctx.accounts.guessing_account;
   guessing_account.random_number = generate_number();
   Ok(())
}
```

- **let guessing_account = &mut ctx.accounts.guessing_account;**：这行代码获取一个可变引用 **guessing_account**，它是一个可以用来存储数据的账户结构体实例，这样，我们接下来就可以把生成的随机数赋值给它，从而记录下来。

- **Ok(())**：表示初始化成功完成，并返回空元组 **()**

### 处理玩家猜数

**处理猜数**

```rust
use anchor_lang::prelude::*;
use solana_program::clock::Clock;
declare_id!("AKUfpJjGJLQuYbVQ6Ny7CxdtCqtmXpk3gJgbUx5P1g68");

#[program]
pub mod anchor_bac {
    //声明为 pub 确保 模块内的内容可以被外部访问和使用
    use super::*; // 导入父模块中的内容
    use std::cmp::Ordering;

    pub fn initialize(ctx: Context<AccountContext>) -> Result<()> {
        let guessing_account = &mut ctx.accounts.guessing_account;
        guessing_account.random_number = generate_number();
        Ok(())
    }

    pub fn guess(ctx: Context<AccountContext>, number: u32) -> Result<()> {
        let guessing_account = &mut ctx.accounts.guessing_account;
        let target = guessing_account.random_number;
        match number.cmp(&target) {
            Ordering::Less => return err!(MyError::NumberTooSmall),
            Ordering::Greater => return err!(MyError::NumberTooLarge),
            Ordering::Equal => return Ok(()),
        }
    }
}

fn generate_number() -> u32 {
    let clock = Clock::get().expect("生成随机数时，获取 unix 时间戳失败");
    let last_digit = (clock.unix_timestamp % 10) as u8;
    let random_number = (last_digit + 1) as u32;
    random_number
}

#[account]
pub struct GuessingAccount {
    pub random_number: u32,
}

#[derive(Accounts)]
pub struct AccountContext<'info> {
    #[account(
        init_if_needed,
        space=8+4,
        payer=payer,
        seeds = [b"guessing pda"],bump)]
    pub guessing_account: Account<'info, GuessingAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code] // 使用 #[error_code] 宏，创建用户自定义 error
pub enum MyError {
    #[msg("Number too small")]
    NumberTooSmall,
    #[msg("Number too large")]
    NumberTooLarge,
}
```

