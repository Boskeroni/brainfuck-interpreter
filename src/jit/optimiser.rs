use std::collections::HashMap;
use crate::jit::ast::{SyntaxTree, SyntaxTreeInst};

pub fn optimise(ast: &mut SyntaxTree) {
    optimise_recursive(ast);
}

fn optimise_loop(ast: &SyntaxTree) -> Vec<SyntaxTreeInst> {
    // this will only have instructions
    // check if the start of the loop is always the same
    let mut final_change: i16 = 0;
    let mut relative_array_changes: HashMap<i16, i8> = HashMap::new();
    for inst in &ast.instructions {    
        match inst {
            SyntaxTreeInst::Shiftup(i) => final_change = final_change.wrapping_add(*i as i16),
            SyntaxTreeInst::ShiftDown(i) => final_change = final_change.wrapping_sub(*i as i16),
            SyntaxTreeInst::Add(i) => {
                if let Some(x) = relative_array_changes.get_mut(&final_change) {
                    *x = x.wrapping_add(*i as i8);
                    continue;
                }
                relative_array_changes.insert(final_change, *i as i8);
            }
            SyntaxTreeInst::Sub(i) => {
                if let Some(x) = relative_array_changes.get_mut(&final_change) {
                    *x = x.wrapping_sub(*i as i8);
                    continue;
                }
                relative_array_changes.insert(final_change, -(*i as i8));
            }
            _ => return Vec::new(), // dont want to mess with something which prints or takes input
        }
    }

    // I'm not sure on how to optimise this right now
    if final_change != 0 {
        return Vec::new();
    }

    // this means all the relative stuff can be done
    let mut new_insts: Vec<SyntaxTreeInst> = Vec::new();
    let start_change_ratio = relative_array_changes.get(&0).unwrap();
    for (offset, change_ratio) in relative_array_changes.iter() {
        // save it for last
        if *offset == 0 {
            continue;
        }
        new_insts.push(SyntaxTreeInst::AddRelative(*offset as i16, *change_ratio, *start_change_ratio));
    }
    
    // this will always end up being 0
    new_insts.push(SyntaxTreeInst::Clear);

    return new_insts;
}

fn optimise_recursive(ast: &mut SyntaxTree) {
    let mut loops_count = 0;
    let mut new_instructions = Vec::new();
    for i in &ast.instructions {
        match i {
            SyntaxTreeInst::Loop => {}
            _ => {
                new_instructions.push(*i);
                continue;
            }            
        }

        let mut sub_ast = &mut ast.loops[loops_count];
        loops_count += 1;
        if !sub_ast.loops.is_empty() {
            optimise_recursive(&mut sub_ast);

            // check if we couldn't clear the loops
            if !sub_ast.loops.is_empty() {
                new_instructions.push(*i);
                continue;
            }
        }

        let possible_optimisations = optimise_loop(&sub_ast);
        // couldnt optimise it down
        if possible_optimisations.is_empty() {
            new_instructions.push(*i);
            continue;
        }

        // we could optimise it down
        new_instructions.extend(possible_optimisations);
        ast.loops.remove(loops_count-1);
        loops_count -= 1;
    }

    ast.instructions = new_instructions;
}