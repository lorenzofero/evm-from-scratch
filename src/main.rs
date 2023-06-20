use std::collections::HashMap;

use evm_from_scratch_new::evm::{
    evm::EVM,
    utils::{
        errors::EVMError,
        test_types::EvmTest,
        types::{AccountState, ExecutionContext},
    },
};

use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

fn main() -> Result<(), EVMError> {
    let text = std::fs::read_to_string("./difficult_test.json").unwrap();
    let data: Vec<EvmTest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();

        let mut ctx = ExecutionContext::new();
        ctx.global_state = if let Some(gs) = &test.state {
            gs.iter()
                .map(|(k, v)| (k.clone(), AccountState::from(v)))
                .collect()
        } else {
            HashMap::new()
        };
        ctx.input.bytecode = code;

        if let Some(tx) = &test.tx {
            if let Some(to) = &tx.to {
                ctx.input.address = to.clone()
            }
            if let Some(from) = &tx.from {
                ctx.input.sender = from.clone()
            }
            if let Some(origin) = &tx.origin {
                ctx.input.origin = origin.clone()
            }
            if let Some(gasprice) = &tx.gasprice {
                ctx.input.price = U256::from_str_radix(gasprice, 16).unwrap();
            }
        }

        let mut result = EVM::execute(ctx)?;

        // Reverse the order of the stack for checking the tests
        result.ctx.machine_state.stack.reverse();

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.ctx.machine_state.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.ctx.machine_state.stack.len() {
                if result.ctx.machine_state.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching && result.output.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.output.success);
            println!("Actual stack: [");
            for v in &result.ctx.machine_state.stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            println!("Execution context: {:#x?}", result.ctx);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
    Ok(())
}
