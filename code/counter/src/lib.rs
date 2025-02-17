use solana_program::{account_info::next_account_info, entrypoint}; 
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

use borsh::{BorshDeserialize, BorshSerialize};
// 由于 solana 在发起交易指令时，值的存储和传输使用的是字节码，所以需要把字节码转为 Rust 类型
// 所以这里需要 BorshDeserialize 和 BorshSerialize 这两个派生宏来实现 (反)序列化

// 定义数据账户的结构
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u32, // 使用 count 来计数，当账户发起交易指令时 count += 1
}

entrypoint!(process_instruction); // solana 的入口函数

fn process_instruction(
    // 当前程序 id
    program_id: &Pubkey,
    // 指令涉及的账户集合
    accounts: &[AccountInfo],
    // 指令的参数
    instruction_data: &[u8],
) -> ProgramResult {
    // 获取调用账户
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    // 判断账户权限
    if account.owner != program_id{
        msg!("account 的所有者不属于当前 program_id");
        return Err(ProgramError::IncorrectProgramId);
    }
    // 反序列化读取 account 结构体实例信息
    let mut counter = CounterAccount::try_from_slice(&account.data.borrow())?;
    counter.count +=1;
    counter.serialize(&mut *account.data.borrow_mut())?;
    Ok(())
}