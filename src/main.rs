#![allow(unused_variables, unused_imports, dead_code)]

mod sha256;

use core::panic;

use garble_lang::{check, circuit::Circuit, compile, literal::Literal, token::UnsignedNumType::{U32, U8}, GarbleProgram};
use sha256::{H, sha256};

// fn test_sha256() {
//     let m: [u32; 16] = [0x80000000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
//     let x = sha256(m, H);
//     println!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}", x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]);

//     let code = include_str!("sha256.garble.rs");
//     let prg = compile(code).map_err(|e| e.prettify(&code)).unwrap();

//     prg.circuit.validate().unwrap();
//     println!("{}", prg.circuit.report_gates());

//     let mut eval = prg.evaluator();
//     eval.set_literal(Literal::Tuple(vec![
//         Literal::Array(m.into_iter().map(|x| Literal::NumUnsigned(x.into(), U32)).collect()),
//         Literal::Array(H.into_iter().map(|x| Literal::NumUnsigned(x.into(), U32)).collect()),
//     ])).map_err(|e| e.prettify(&code)).unwrap();
//     let output = eval.run().map_err(|e| e.prettify(&code)).unwrap();
//     let result = output.into_literal().map_err(|e| e.prettify(&code)).unwrap();
//     if let Literal::Array(x) = result {
//         assert_eq!(x.len(), 8);
//         let x: Vec<u32> = x.iter().map(|x| {
//             if let &Literal::NumUnsigned(i, U32) = x {
//                 u32::try_from(i).unwrap()
//             } else {
//                 panic!("result didn't match pattern");
//             }
//         }).collect();
//         println!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}", x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]);
//     } else {
//         panic!("result didn't match pattern");
//     }
// }

fn test_aes128(code: &str, prg: &GarbleProgram, key: &str, block: &str, expected: &str) {
    let key: [u8; 16] = hex::decode(key).unwrap().try_into().unwrap();
    let block: [u8; 16] = hex::decode(block).unwrap().try_into().unwrap();

    println!("{}", expected);

    let mut eval = prg.evaluator();
    eval.set_literal(Literal::Tuple(vec![
        Literal::Array(key.into_iter().map(|x| Literal::NumUnsigned(x.into(), U8)).collect()),
        Literal::Array(block.into_iter().map(|x| Literal::NumUnsigned(x.into(), U8)).collect()),
    ])).map_err(|e| e.prettify(&code)).unwrap();
    let output = eval.run().map_err(|e| e.prettify(&code)).unwrap();
    let result = output.into_literal().map_err(|e| e.prettify(&code)).unwrap();
    if let Literal::Array(x) = result {
        assert_eq!(x.len(), 16);
        let x: Vec<u8> = x.iter().map(|x| {
            if let &Literal::NumUnsigned(i, U8) = x {
                u8::try_from(i).unwrap()
            } else {
                panic!("result didn't match pattern");
            }
        }).collect();
        let x = hex::encode(x);
        println!("{}", x);
        assert_eq!(x, expected)
    } else {
        panic!("result didn't match pattern");
    }
}

fn output_bristol_fashion(x: &Circuit, num_output_wires: usize) {
    assert!(num_output_wires <= x.output_gates.len());
    let total_input_gates = x.input_gates.clone().into_iter().reduce(|x: usize, y: usize| {x + y}).unwrap_or(0);

    println!("{} {}", x.gates.len() + num_output_wires, x.wires().len() + num_output_wires);
    print!("{}", x.input_gates.len());
    for i in x.input_gates.iter() {
        print!(" {}", i);
    }
    println!("");
    println!("1 {}", num_output_wires);
    println!("");
    
    for (i, gate) in x.gates.iter().enumerate() {
        let i = i + total_input_gates;
        match gate {
            garble_lang::circuit::Gate::Xor(a, b) => println!("2 1 {} {} {} XOR", a, b, i),
            garble_lang::circuit::Gate::Not(a) => println!("1 1 {} {} INV", a, i),
            garble_lang::circuit::Gate::And(a, b) => println!("2 1 {} {} {} AND", a, b, i),
        }
    }

    let false_wire = total_input_gates;

    let mut i = x.wires().len();
    for foo in x.output_gates[(x.output_gates.len() - num_output_wires)..].iter().rev() {
        println!("2 1 {} {} {} XOR", false_wire, foo, i);
        // assert_eq!(*foo, i);
        i = i + 1;
    }
}

fn foo() {
    let code = include_str!("xor.garble.rs");
    let prg = compile(code).map_err(|e| e.prettify(&code)).unwrap();
    // println!("{}", prg.circuit.report_gates());
    // println!("input_gates.len(): {}", prg.circuit.input_gates.len());
    // println!("gates.len(): {}", prg.circuit.gates.len());
    // println!("output_gates.len(): {}", prg.circuit.output_gates.len());
    // println!("{:?}", prg.circuit);
    output_bristol_fashion(&prg.circuit, 8);
}

fn test_ghash(code: &str, prg: &GarbleProgram, key: &str, block: &str, expected: &str) {
    let key: [u8; 16] = hex::decode(key).unwrap().try_into().unwrap();
    let block: [u8; 16] = hex::decode(block).unwrap().try_into().unwrap();

    println!("{}", expected);

    let mut eval = prg.evaluator();
    eval.set_literal(Literal::Tuple(vec![
        Literal::Array(key.into_iter().map(|x| Literal::NumUnsigned(x.into(), U8)).collect()),
        Literal::Array(block.into_iter().map(|x| Literal::NumUnsigned(x.into(), U8)).collect()),
    ])).map_err(|e| e.prettify(&code)).unwrap_or_else(|x| {
        println!("{}", x);
        panic!();
    });
    let output = eval.run().map_err(|e| e.prettify(&code)).unwrap_or_else(|x| {
        println!("{}", x);
        panic!();
    });
    let result = output.into_literal().map_err(|e| e.prettify(&code)).unwrap_or_else(|x| {
        println!("{}", x);
        panic!();
    });
    if let Literal::Array(x) = result {
        assert_eq!(x.len(), 16);
        let x: Vec<u8> = x.iter().map(|x| {
            if let &Literal::NumUnsigned(i, U8) = x {
                u8::try_from(i).unwrap()
            } else {
                panic!("result didn't match pattern");
            }
        }).collect();
        let x = hex::encode(x);
        println!("{}", x);
        assert_eq!(x, expected)
    } else {
        panic!("result didn't match pattern");
    }
}

fn main() {
    // test_sha256();

    let code = include_str!("aes128.garble.rs");
    let code2 = include_str!("ghash.garble.rs");
    let prg = check(code).map_err(|e| e.prettify(&code)).unwrap_or_else(|x| {
        println!("{}", x);
        panic!();
    });
    let prg2 = compile(code).map_err(|e| e.prettify(&code)).unwrap_or_else(|x| {
        println!("{}", x);
        panic!();
    });
    let prg3 = check(code2).map_err(|e| e.prettify(&code2)).unwrap_or_else(|x| {
        println!("{}", x);
        panic!();
    });
    let prg4 = compile(code2).map_err(|e| e.prettify(&code2)).unwrap_or_else(|x| {
        println!("{}", x);
        panic!();
    });
    let (aes128, _) = prg.compile("aes128").unwrap();
    let (aes128_main, _) = prg.compile("main").unwrap();
    let (aes128_key_schedule, _) = prg.compile("aes128_key_schedule").unwrap();
    let (aes128_with_key_schedule, _) = prg.compile("aes128_with_key_schedule").unwrap();
    let (gcm_mult, _) = prg3.compile("gcm_mult").unwrap();
    let (gcm_setup, _) = prg3.compile("gcm_setup").unwrap();
    let (mod_polynomial_mult, _) = prg3.compile("mod_polynomial_mult").unwrap();
    
    // prg.circuit.validate().unwrap();
    println!("{}", gcm_mult.report_gates());
    println!("{}", gcm_setup.report_gates());
    println!("{}", mod_polynomial_mult.report_gates());
    // println!("{}", aes128_with_key_schedule.report_gates());

    // test_aes128(code, &prg2, "00000000000000000000000000000000", "00000000000000000000000000000000", "66e94bd4ef8a2c3b884cfa59ca342b2e");
    // test_aes128(code, &prg2, "2b7e151628aed2a6abf7158809cf4f3c", "6bc1bee22e409f96e93d7e117393172a", "3ad77bb40d7a3660a89ecaf32466ef97");
    // test_aes128(code, &prg2, "2b7e151628aed2a6abf7158809cf4f3c", "ae2d8a571e03ac9c9eb76fac45af8e51", "f5d3d58503b9699de785895a96fdbaaf");
    // test_aes128(code, &prg2, "2b7e151628aed2a6abf7158809cf4f3c", "30c81c46a35ce411e5fbc1191a0a52ef", "43b1cd7f598ece23881b00e3ed030688");

    // test_ghash(code2, &prg4, "66e94bd4ef8a2c3b884cfa59ca342b2e", "0388dace60b6a392f328c2b971b2fe78", "5e2ec746917062882c85b0685353deb7");
    test_ghash(code2, &prg4, "0388dace60b6a392f328c2b971b2fe78", "66e94bd4ef8a2c3b884cfa59ca342b2e", "5e2ec746917062882c85b0685353deb7");

    // output_bristol_fashion(&gcm_mult, 128);
    // foo();
}
