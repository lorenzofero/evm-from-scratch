use primitive_types::U256;
use sha3::{Digest, Keccak256};

use crate::evm::utils::{
    errors::EVMError,
    helpers::{hex_string_from_bytes, modulo_address_size, pop_n, update_active_words_memory},
    types::{ExecutionContext, OpcodeResult},
};

/// 0x30
pub fn address(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.address);
    Ok(None)
}

/// 0x31
pub fn balance(ctx: &mut ExecutionContext) -> OpcodeResult {
    let address = modulo_address_size(&pop_n(ctx, 1)?[0]);
    let balance = if let Some(account_state) = ctx.global_state.get(&address) {
        account_state.balance
    } else {
        U256::zero()
    };

    ctx.machine_state.stack.push(balance);
    ctx.accrued_substate.accessed_accounts.insert(address);

    Ok(None)
}

/// 0x32 TODO: add a check that origin has always empty code
pub fn origin(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.origin);

    Ok(None)
}

/// 0x33 Solidity calls this msg.sender
pub fn caller(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.sender);

    Ok(None)
}

/// 0x34 Solidity calls this msg.value
pub fn callvalue(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.value);
    Ok(None)
}

/// 0x35
pub fn calldataload(ctx: &mut ExecutionContext) -> OpcodeResult {
    let offset = pop_n(ctx, 1)?[0];
    let offset: usize = offset
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(offset, ctx.clone()))?;

    let data: Vec<u8> = vec![0_u8; 32]
        .iter()
        .enumerate()
        .map(|(idx, zero_byte)| *ctx.input.data.get(offset + idx).unwrap_or(zero_byte))
        .collect();

    let str_data = hex_string_from_bytes(&data);
    let data = U256::from_str_radix(&str_data, 16)
        .map_err(|_| EVMError::FromStrRadixError(str_data, ctx.clone()))?;

    ctx.machine_state.stack.push(data);

    Ok(None)
}

/// 0x36
pub fn calldatasize(ctx: &mut ExecutionContext) -> OpcodeResult {
    let size = U256::from(ctx.input.data.len());

    ctx.machine_state.stack.push(size);

    Ok(None)
}

/// 0x37
pub fn calldatacopy(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 3)?;

    let dest_offset: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let offset: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;
    let size: usize = stack_items[2]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[2], ctx.clone()))?;

    vec![0_u8; size]
        .iter()
        .enumerate()
        .for_each(|(idx, zero_byte)| {
            let byte = ctx.input.data.get(offset + idx).unwrap_or(zero_byte);
            ctx.machine_state.memory[dest_offset + idx] = *byte
        });

    update_active_words_memory(ctx, offset + size);

    Ok(None)
}

/// 0x38
pub fn codesize(ctx: &mut ExecutionContext) -> OpcodeResult {
    let size = U256::from(ctx.input.bytecode.len());
    ctx.machine_state.stack.push(size);
    Ok(None)
}

/// 0x39
pub fn codecopy(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 3)?;

    let dest_offset: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let offset: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;
    let size: usize = stack_items[2]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[2], ctx.clone()))?;

    vec![0_u8; size]
        .iter()
        .enumerate()
        .for_each(|(idx, zero_byte)| {
            let byte = ctx.input.bytecode.get(offset + idx).unwrap_or(zero_byte);
            ctx.machine_state.memory[dest_offset + idx] = *byte
        });

    update_active_words_memory(ctx, offset + size);

    Ok(None)
}

/// 0x3a
pub fn gasprice(ctx: &mut ExecutionContext) -> OpcodeResult {
    ctx.machine_state.stack.push(ctx.input.price);
    Ok(None)
}

/// 0x3b
pub fn extcodesize(ctx: &mut ExecutionContext) -> OpcodeResult {
    let address = modulo_address_size(&pop_n(ctx, 1)?[0]);

    let size = if let Some(account_state) = ctx.global_state.get(&address) {
        U256::from(account_state.code.len())
    } else {
        U256::zero()
    };

    ctx.machine_state.stack.push(size);
    ctx.accrued_substate.accessed_accounts.insert(address);

    Ok(None)
}

/// 0x39
pub fn extcodecopy(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 4)?;

    let address = modulo_address_size(&stack_items[0]);
    let dest_offset: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;
    let offset: usize = stack_items[2]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[2], ctx.clone()))?;
    let size: usize = stack_items[3]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[3], ctx.clone()))?;

    let empty_code: Vec<u8> = Vec::with_capacity(0);
    let code = if let Some(account_state) = ctx.global_state.get(&address) {
        &account_state.code
    } else {
        &empty_code
    };

    vec![0_u8; size]
        .iter()
        .enumerate()
        .for_each(|(idx, zero_byte)| {
            let byte = code.get(offset + idx).unwrap_or(zero_byte);
            ctx.machine_state.memory[dest_offset + idx] = *byte
        });

    update_active_words_memory(ctx, offset + size);
    ctx.accrued_substate.accessed_accounts.insert(address);

    Ok(None)
}

/// 0x3d
pub fn returndatasize(ctx: &mut ExecutionContext) -> OpcodeResult {
    let return_data_size = U256::from(ctx.machine_state.output.len());
    ctx.machine_state.stack.push(return_data_size);
    Ok(None)
}

/// 0x3e
pub fn returndatacopy(ctx: &mut ExecutionContext) -> OpcodeResult {
    let stack_items = pop_n(ctx, 3)?;
    let dest_offset: usize = stack_items[0]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[0], ctx.clone()))?;
    let offset: usize = stack_items[1]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[1], ctx.clone()))?;
    let size: usize = stack_items[2]
        .try_into()
        .map_err(|_| EVMError::U256ToUSizeError(stack_items[2], ctx.clone()))?;

    for i in offset..offset + size {
        ctx.machine_state.memory[dest_offset + i] = *ctx.machine_state.output.get(i).unwrap_or(&0);
    }

    update_active_words_memory(ctx, dest_offset + size);

    Ok(None)
}

/// 0x3f
pub fn extcodehash(ctx: &mut ExecutionContext) -> OpcodeResult {
    let address = modulo_address_size(&pop_n(ctx, 1)?[0]);

    if let Some(account_state) = ctx.global_state.get(&address) {
        let mut hasher = Keccak256::new();
        hasher.update(&account_state.code);

        let hash = hasher.finalize();
        let hash_vec = hash.to_vec();

        let val = U256::from(&hash_vec[..]);
        ctx.machine_state.stack.push(val);
    } else {
        ctx.machine_state.stack.push(U256::zero())
    }

    ctx.accrued_substate.accessed_accounts.insert(address);

    Ok(None)
}
