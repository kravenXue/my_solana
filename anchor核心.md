# Anchor 核心内容

## Anchor 的程序结构初步介绍

在执行完 **`anchor init my_project`** 命令后，会自动生成 Anchor 示例项目，其中的 **`lib.rs`**文件是 Anchor 框架的核心模块，包含了许多 **`macros`** 宏，这些宏为我们的程序生成 Rust 模板代码以及相应的安全校验

### 主要用到的宏

- **declare_id!**：声明**程序地址**。该宏创建了一个存储程序地址 **`program_id`** 的字段，可以通过一个指定的 **`program_id`** 访问到指定的链上程序
- **#[program]**：程序的**业务逻辑代码**实现都在**`#[program]`** 模块下完成
- **#[derive(Accounts)]**：由于Solana 账户模型的特点，大部分的参数将以**账户集合**的形式传入程序，在该宏修饰的结构体中定义了程序所需要的账户集合
- **#[account]**：该宏用来修饰程序所需要的自定义账户

#### Anchor 框架的结构

```rust
// 引入 anchor 框架的预导入模块
use anchor_lang::prelude::*;

// 程序的链上地址
declare_id!("3Vg9yrVTKRjKL9QaBWsZq4w7UsePHAttuZDbrZK3G5pf");

// 指令处理逻辑
#[program]
mod anchor_counter {
    use super::*;
    pub fn instruction_one(ctx: Context<InstructionAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.data = instruction_data;
        Ok(())
    }
}

// 指令涉及的账户集合
#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

// 自定义账户类型
#[account]
pub struct Counter {
    data: u64
}
```

我们以上方代码为例，该程序使用 **`instruction_one`** 指令函数接收 **`u64`** 类型的数据，并保存在链上 **`Counter`** 结构体中。当然，Solana 中一切皆账户，所以 **`Counter`** 结构体也是该程序的派生账户 PDA

- **导入框架依赖**：

  这里导入了 Anchor 框架的预导入模块，其中包含了编写 Solana 程序所需的基本功能，比如：

  - **`AnchorDeserialize`** 和 **`AnchorSerialize`**（反序列化和序列化）
  - **`Accounts`**（用于定义和管理程序账户的宏）
  - **`Context`**（提供有关当前程序执行上下文的信息，包括账户、系统程序等）
  - ……

  ```rust
  // 引入相关依赖
  use anchor_lang::prelude::*;
  ```

- **declare_id! 宏**：

  指定 Solana 链上程序地址。当你首次构建 Anchor 程序时，框架会自动生成用于部署程序的默认密钥对，其中相应的公钥即为**`declare_id!`** 宏所声明的程序ID（program_id）

  通常情况下，每次构建 Anchor 框架的 Solana 程序时，program_id 都会有所不同。但是通过 **`declare_id!`** 宏指定某个地址，我们就能保证程序升级后的地址不变。这种升级方式比起以太坊中智能合约的升级（使用代理模式），要简单很多。

  ```rust
  // 这里只是示意，实际的 program_id 会有所不同
  declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
  ```

- **#[program] 宏**：

  修饰包含了所有程序 instructions 指令的模块，该模块中实现了处理指令的具体业务逻辑，每个 **`pub`** 修饰的公共函数，都是一个单独的指令。函数的参数有以下2种：

  - 第一个参数为 **`Context`** 类型，包含了处理该指令的上下文信息。
  - 第二个参数为指令的数据，可选。

- **#[derive(Accounts)] 派生宏**：

  定义了 instruction 指令所要求的账户列表。该宏实现了给定结构体 **`InstructionAccounts`**（反）序列化的 Trait 特征，因此在获取账户时不再需要手动迭代账户以及反序列化操作。并且实现了账户满足程序安全运行所需要的安全检查，当然，需要 **`#[account]`**宏配合使用。

- **#[account]**：

  该宏用来修饰程序所需要的自定义账户，它支持定义账户的属性并实现相应的安全校验。这里我们的自定义了一个计数器 **`Counter`**。当然，可以有更复杂的结构，取决于我们的具体业务逻辑。

## Anchor 的程序结构 program

### 示例代码

这里展示了使用 Anchor 框架实现的计数器程序，实现了计数器的初始化和累加功能

```rust
// 引入 anchor 框架的预导入模块
use anchor_lang::prelude::*;

// 程序的链上地址
declare_id!("3Vg9yrVTKRjKL9QaBWsZq4w7UsePHAttuZDbrZK3G5pf");

// 指令处理逻辑
#[program]
mod anchor_counter {
    use super::*;
    pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}

// 指令涉及的账户集合
#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccounts<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

// 自定义账户类型
#[account]
pub struct Counter {
    count: u64
}
```

### #[program] 宏

**声明使用**

```rust
#[program]
mod program_module_name {
    // ...
}
```

该宏定义一个 Solana 程序模块，其中包含了程序的指令（instructions）和其他相关逻辑。它包含如下的功能：

- **定义处理不同指令的函数：**在程序模块中，开发者可以定义处理不同指令的函数。这些函数包含了具体的指令处理逻辑。上节只有1个指令函数 **`instruction_one`**，本节我们在 #[program] 宏中实现了2个指令函数：**`initialize`** 和 **`increment`**，用来实现计数器的相关逻辑，使其更接近于实际的业务场景

  ```rust
  #[program]
  mod anchor_counter {
      use super::*;
  		// 初始化账户，并以传入的 instruction_data 作为计数器的初始值
      pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
  				ctx.accounts.counter.count = instruction_data;
          Ok(())
      }
  
  		// 在初始值的基础上实现累加 1 操作
      pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
          let counter = &mut ctx.accounts.counter;
          msg!("Previous counter: {}", counter.count);
          counter.count = counter.count.checked_add(1).unwrap();
          msg!("Counter incremented. Current count: {}", counter.count);
          Ok(())
      }
  }
  ```

- **提供与** **Solana SDK** **交互的功能：**通过 **`#[program]`** 宏，Anchor 框架提供了一些功能，使得与 Solana SDK 进行交互变得更加简单。例如，可以直接使用 **`declare_id`**、**`Account`**、**`Context`**、**`Sysvar`** 等结构和宏，而不必手动编写底层的 Solana 交互代码，本单元第一节我们没有使用 Anchor 框架，所以需要手动迭代账户、判断账户权限等操作，现在 Anchor 已经替我们实现了这些功能。

- **自动生成 IDL（接口定义语言）：** **`#[program]`** 宏也用于自动生成程序的 IDL。IDL 是用于描述 Solana 程序接口的一种规范，它定义了合约的数据结构、指令等。Anchor 框架使用这些信息生成用于与客户端进行交互的 Rust 代码。

  Solana 的 IDL（接口定义语言）和以太坊的 ADSL（Application Binary Interface Description Language）有一些相似之处。它们都是一种用于描述智能合约接口的语言规范，包括合约的数据结构、指令等信息。

## Anchor 的程序结构 Context

### 示例代码

```rust
// 引入 anchor 框架的预导入模块
use anchor_lang::prelude::*;

// 程序的链上地址
declare_id!("3Vg9yrVTKRjKL9QaBWsZq4w7UsePHAttuZDbrZK3G5pf");

// 指令处理逻辑
#[program]
mod anchor_counter {
    use super::*;
    pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}

// 指令涉及的账户集合
#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
    pub pda_counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccounts<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

// 自定义账户类型
#[account]
pub struct Counter {
    count: u64
}
```

### Context 

```rust
#[program]
mod anchor_counter {
    pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }
}
```

### Context 参数类型

**`Context`** 是 Anchor 框架中定义的一个结构体，用于封装与 Solana 程序执行相关的上下文信息，**包含了 instruction 指令元数据以及逻辑中所需要的所有账户信息**。它的结构如下：

```rust
// anchor_lang::context
pub struct Context<'a, 'b, 'c, 'info, T> {
    /// 当前的program_id
    pub program_id: &'a Pubkey,
    /// 反序列化的账户集合accounts
    pub accounts: &'b mut T,
    /// 不在 accounts 中的账户，它是数组类型
    pub remaining_accounts: &'c [AccountInfo<'info>],
    /// ...
}
```

Context 使用泛型 **`T`** 指定了指令函数所需要的账户集合，在实际的使用中我们需要指定泛型 T 的具体类型，如**`Context<InitializeAccounts>`**、**`Context<UpdateAccounts>`**等，通过这个参数，指令函数能够获取到如下数据：

- **`ctx.program_id`**：程序ID，当前执行的程序地址。它是一个 **`Pubkey`** 类型的值。

- **`ctx.accounts`**：账户集合，它的类型为泛型 T 所指定的具体类型，如初始化操作所需的账户集合 **`InitializeAccounts`**，更新操作所需的账户集合 **`UpdateAccounts`**，通过派生宏 **`#[derive(Accounts)]`** 生成。并且 Anchor 框架会为我们自动进行反序列化。

- **`ctx.remaining_accounts`**：剩余账户集合，包含了当前指令中未被 **`#[derive(Accounts)]`** 明确声明的账户。它提供了一种灵活的方式，使得程序能够处理那些在编写程序时不确定存在的账户，或者在执行过程中动态创建的账户。其中的账户不支持直接的反序列化，需要手动处理

### `Context<T>` 泛型 T

我们先看下第一个指令函数的泛型T：**`InitializeAccounts`**，该账户集合有3个账户：

- 第1个为数据账户pda_counter，它是该程序的衍生账户，用于存储计数器数据
- 第2个参数为对交易发起签名的账户user，支付了该笔交易费
- 第3个参数为 Solana 系统账户system_program，因为PDA账户需要由系统生成，所以我们也需要这个系统账户

```rust
#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
		// pda 账户
    #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
    pub pda_counter: Account<'info, Counter>,
		// 交易签名账户
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

### 指令参数（可选）

在 Anchor 框架中，指令函数的第一个参数 **`ctx`** 是**必须**的，而第二个参数是指令函数执行时传递的额外数据，是**可选**的，是否需要取决于指令的具体逻辑和需求。在 **`initialize`** 中，它被用于初始化计数器的初始值；而在 **`increment`** 中，该指令不需要额外的数据，所以只有 **`ctx`** 参数

## Anchor 的程序结构 Accounts

### 示例代码

```rust
// 引入 anchor 框架的预导入模块
use anchor_lang::prelude::*;

// 程序的链上地址
declare_id!("3Vg9yrVTKRjKL9QaBWsZq4w7UsePHAttuZDbrZK3G5pf");

// 指令处理逻辑
#[program]
mod anchor_counter {
    use super::*;
    pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter: {}", counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented. Current count: {}", counter.count);
        Ok(())
    }
}

// 指令涉及的账户集合
#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
    pub pda_counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccounts<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

// 自定义账户类型
#[account]
pub struct Counter {
    count: u64
}
```

### #[derive(Accounts)]

上一节的学习我们知道使用 **`ctx.accounts`** 可以获取指令函数的账户集合 **`InitializeAccounts`**，它是一个实现了 **`#[derive(Accounts)]`** 派生宏的结构体。该派生宏为结构体生成与 Solana 程序账户相关的处理逻辑，以便开发者能够更方便地访问和管理其中的账户

```rust
// anchor_lang::context
pub struct Context<'a, 'b, 'c, 'info, T> {
    pub accounts: &'b mut T,
    // ...
}

#[program]
mod anchor_counter {
    pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
        ctx.accounts.counter.count = instruction_data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    // ...
}
```

### #[derive(Accounts)] 宏的介绍

该宏应用于指令所要求的账户列表，实现了给定 struct 结构体数据的反序列化功能，**因此在获取账户时不再需要手动迭代账户以及反序列化操作，并且实现了账户满足程序安全运行所需要的安全检查**，当然，需要#[account]宏配合使用

- 1.下面我们看下示例中的InitializeAccounts结构体，当initialize指令函数被调用时，程序会做如下2个校验：

  ```rust
  #[derive(Accounts)]
  pub struct InitializeAccounts<'info> {
      #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
      pub pda_counter: Account<'info, Counter>,
      #[account(mut)]
      pub user: Signer<'info>,
      pub system_program: Program<'info, System>,
  }
  ```

    - **账户类型校验：**传入的账户是否跟 **`InitializeAccounts`** 定义的账户类型相匹配，例如Account、Singer、Program等类型。

    - **账户权限校验**：根据账户标注的权限，框架会进行相应的权限校验，例如检查是否有足够的签名权限、是否可以修改等

  如果其中任何一个校验失败，将导致指令执行失败并产生错误

- 2.**`InitializeAccounts`** 结构体中有如下 3 种账户类型：

  - **`Account`** 类型：它是 **`AccountInfo`** 类型的包装器，可用于验证账户所有权并将底层数据反序列化为Rust类型。对于结构体中的 **`counter`** 账户，Anchor 会实现如下功能：

    ```rust
    pub pda_counter: Account<'info, Counter>,
    ```

    - 该账户类型的 Counter 数据自动实现反序列化。

    - 检查传入的所有者是否跟 Counter 的所有者匹配。

  - **`Signer`** 类型：这个类型会检查给定的账户是否签署了交易，但并不做所有权的检查。只有在并不需要底层数据的情况下，才应该使用Signer类型。

    ```rust
    pub user: Signer<'info>,
    ```

  - **`Program`** 类型：验证这个账户是个特定的程序。对于 system_program 字段，Program 类型用于指定程序应该为系统程序，Anchor 会替我们完成校验。

    ```rust
    pub system_program: Program<'info, System>,
    ```

  这里，只是对 **`#[derive(Accounts)]`** 中的账户的类型进行了介绍，其实每个字段还有 **`#[account(..)]`** 属性宏

  总的来说，**`#[derive(Accounts)]`** 是 Anchor 框架的宏，简化 Solana 程序中的账户处理代码。通过结构体属性标注，自动生成账户操作逻辑，提高可读性和可维护性，使开发者更专注于业务逻辑
