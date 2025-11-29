//! Predicate to Bytecode Compiler
//!
//! Compiles predicate expressions from AST into bytecode for the WAT VM.
//! The bytecode format must match what pxyz.wat expects.

use crate::{Opcode, CompileError, limits};
use crate::dsl::ast::{PredicateExpr, Value};
use crate::compiler::ir::StringPool;

/// Predicate bytecode compiler
pub struct PredicateCompiler<'a> {
    bytecode: Vec<u8>,
    strings: &'a mut StringPool,
}

impl<'a> PredicateCompiler<'a> {
    pub fn new(strings: &'a mut StringPool) -> Self {
        Self {
            bytecode: Vec::new(),
            strings,
        }
    }
    
    /// Compile a predicate expression to bytecode
    pub fn compile(&mut self, expr: &PredicateExpr) -> Result<Vec<u8>, CompileError> {
        self.bytecode.clear();
        self.compile_expr(expr)?;
        self.emit(Opcode::Ret as u8);
        
        if self.bytecode.len() > limits::MAX_PREDICATE_BYTECODE {
            return Err(CompileError::Predicate(format!(
                "Bytecode too large: {} bytes (max {})",
                self.bytecode.len(),
                limits::MAX_PREDICATE_BYTECODE
            )));
        }
        
        Ok(std::mem::take(&mut self.bytecode))
    }
    
    fn compile_expr(&mut self, expr: &PredicateExpr) -> Result<(), CompileError> {
        match expr {
            PredicateExpr::Always => {
                self.emit(Opcode::PushInt as u8);
                self.emit_i32(1);
            }
            
            PredicateExpr::Fail => {
                self.emit(Opcode::PushInt as u8);
                self.emit_i32(0);
            }
            
            PredicateExpr::Eq { left, right } => {
                self.compile_var(left);
                self.compile_value(right);
                self.emit(Opcode::Eq as u8);
            }
            
            PredicateExpr::Neq { left, right } => {
                self.compile_var(left);
                self.compile_value(right);
                self.emit(Opcode::Neq as u8);
            }
            
            PredicateExpr::Gt { left, right } => {
                self.compile_var(left);
                self.compile_value(right);
                self.emit(Opcode::Gt as u8);
            }
            
            PredicateExpr::Gte { left, right } => {
                self.compile_var(left);
                self.compile_value(right);
                self.emit(Opcode::Gte as u8);
            }
            
            PredicateExpr::Lt { left, right } => {
                self.compile_var(left);
                self.compile_value(right);
                self.emit(Opcode::Lt as u8);
            }
            
            PredicateExpr::Lte { left, right } => {
                self.compile_var(left);
                self.compile_value(right);
                self.emit(Opcode::Lte as u8);
            }
            
            PredicateExpr::Contains { left, right } => {
                self.compile_var(left);
                self.compile_var(right);
                self.emit(Opcode::Contains as u8);
            }
            
            PredicateExpr::Matches { left, pattern } => {
                self.compile_var(left);
                self.emit(Opcode::PushStr as u8);
                let offset = self.strings.intern(pattern);
                self.emit_u32(offset);
                self.emit(Opcode::Matches as u8);
            }
            
            PredicateExpr::StartsWith { left, prefix } => {
                self.compile_var(left);
                self.emit(Opcode::PushStr as u8);
                let offset = self.strings.intern(prefix);
                self.emit_u32(offset);
                self.emit(Opcode::StartsWith as u8);
            }
            
            PredicateExpr::EndsWith { left, suffix } => {
                self.compile_var(left);
                self.emit(Opcode::PushStr as u8);
                let offset = self.strings.intern(suffix);
                self.emit_u32(offset);
                self.emit(Opcode::EndsWith as u8);
            }
            
            PredicateExpr::And { conditions } => {
                if conditions.is_empty() {
                    // Empty AND = true
                    self.emit(Opcode::PushInt as u8);
                    self.emit_i32(1);
                } else {
                    self.compile_expr(&conditions[0])?;
                    for cond in conditions.iter().skip(1) {
                        self.compile_expr(cond)?;
                        self.emit(Opcode::And as u8);
                    }
                }
            }
            
            PredicateExpr::Or { conditions } => {
                if conditions.is_empty() {
                    // Empty OR = false
                    self.emit(Opcode::PushInt as u8);
                    self.emit_i32(0);
                } else {
                    self.compile_expr(&conditions[0])?;
                    for cond in conditions.iter().skip(1) {
                        self.compile_expr(cond)?;
                        self.emit(Opcode::Or as u8);
                    }
                }
            }
            
            PredicateExpr::Not { condition } => {
                self.compile_expr(condition)?;
                self.emit(Opcode::Not as u8);
            }
            
            PredicateExpr::Ref { predicate } => {
                // Predicate references are resolved during lowering
                // If we get here, it means the reference wasn't resolved
                return Err(CompileError::Predicate(format!(
                    "Unresolved predicate reference: {}",
                    predicate
                )));
            }
            
            PredicateExpr::Fn { name, arg } => {
                self.compile_var(arg);
                match name.to_lowercase().as_str() {
                    "length" | "len" => self.emit(Opcode::Len as u8),
                    "defined" | "isdefined" | "is_defined" => self.emit(Opcode::IsDefined as u8),
                    "null" | "isnull" | "is_null" => self.emit(Opcode::IsNull as u8),
                    "confirmed" | "isconfirmed" | "is_confirmed" => self.emit(Opcode::IsConfirmed as u8),
                    _ => {
                        return Err(CompileError::Predicate(format!(
                            "Unknown function: {}",
                            name
                        )));
                    }
                }
            }
        }
        Ok(())
    }
    
    /// Compile a variable reference or string literal
    fn compile_var(&mut self, path: &str) {
        if path.starts_with('$') {
            // Variable reference: $token.sub, $entity.owner_id, etc.
            self.emit(Opcode::LoadVar as u8);
            let offset = self.strings.intern(&path[1..]); // Skip $
            self.emit_u32(offset);
        } else {
            // String literal
            self.emit(Opcode::PushStr as u8);
            let offset = self.strings.intern(path);
            self.emit_u32(offset);
        }
    }
    
    /// Compile a value (int, string, bool, or variable)
    fn compile_value(&mut self, value: &Value) {
        match value {
            Value::Int(n) => {
                self.emit(Opcode::PushInt as u8);
                self.emit_i32(*n as i32);
            }
            Value::Float(f) => {
                // Truncate to integer for now
                self.emit(Opcode::PushInt as u8);
                self.emit_i32(*f as i32);
            }
            Value::Str(s) => {
                self.emit(Opcode::PushStr as u8);
                let offset = self.strings.intern(s);
                self.emit_u32(offset);
            }
            Value::Bool(b) => {
                self.emit(Opcode::PushInt as u8);
                self.emit_i32(if *b { 1 } else { 0 });
            }
            Value::Var(path) => {
                self.compile_var(path);
            }
        }
    }
    
    fn emit(&mut self, byte: u8) {
        self.bytecode.push(byte);
    }
    
    fn emit_i32(&mut self, v: i32) {
        self.bytecode.extend_from_slice(&v.to_le_bytes());
    }
    
    fn emit_u32(&mut self, v: u32) {
        self.bytecode.extend_from_slice(&v.to_le_bytes());
    }
    
    #[allow(dead_code)]
    fn emit_u16(&mut self, v: u16) {
        self.bytecode.extend_from_slice(&v.to_le_bytes());
    }
}

/// Disassemble bytecode to human-readable string
pub fn disassemble(bytecode: &[u8], strings: &StringPool) -> String {
    let mut result = String::new();
    let mut pc = 0;
    
    while pc < bytecode.len() {
        let op = bytecode[pc];
        result.push_str(&format!("{:04x}: ", pc));
        
        if let Some(opcode) = Opcode::from_byte(op) {
            result.push_str(&format!("{:?}", opcode));
            pc += 1;
            
            match opcode {
                Opcode::PushInt => {
                    if pc + 4 <= bytecode.len() {
                        let val = i32::from_le_bytes([
                            bytecode[pc],
                            bytecode[pc + 1],
                            bytecode[pc + 2],
                            bytecode[pc + 3],
                        ]);
                        result.push_str(&format!(" {}", val));
                        pc += 4;
                    } else {
                        result.push_str(" <truncated>");
                        break;
                    }
                }
                Opcode::PushStr | Opcode::LoadVar | Opcode::LoadField => {
                    if pc + 4 <= bytecode.len() {
                        let offset = u32::from_le_bytes([
                            bytecode[pc],
                            bytecode[pc + 1],
                            bytecode[pc + 2],
                            bytecode[pc + 3],
                        ]);
                        if let Some(s) = strings.get(offset) {
                            result.push_str(&format!(" \"{}\"", s));
                        } else {
                            result.push_str(&format!(" @{}", offset));
                        }
                        pc += 4;
                    } else {
                        result.push_str(" <truncated>");
                        break;
                    }
                }
                Opcode::Matches => {
                    // Pattern follows as string offset
                    if pc + 4 <= bytecode.len() {
                        let offset = u32::from_le_bytes([
                            bytecode[pc],
                            bytecode[pc + 1],
                            bytecode[pc + 2],
                            bytecode[pc + 3],
                        ]);
                        if let Some(s) = strings.get(offset) {
                            result.push_str(&format!(" /{}/", s));
                        } else {
                            result.push_str(&format!(" @{}", offset));
                        }
                        pc += 4;
                    }
                }
                Opcode::CallPred => {
                    if pc + 2 <= bytecode.len() {
                        let id = u16::from_le_bytes([bytecode[pc], bytecode[pc + 1]]);
                        result.push_str(&format!(" #{}", id));
                        pc += 2;
                    } else {
                        result.push_str(" <truncated>");
                        break;
                    }
                }
                _ => {}
            }
        } else {
            result.push_str(&format!("??? (0x{:02x})", op));
            pc += 1;
        }
        
        result.push('\n');
    }
    
    result
}

/// Validate bytecode without executing
pub fn validate_bytecode(bytecode: &[u8]) -> Result<(), String> {
    if bytecode.is_empty() {
        return Err("Empty bytecode".into());
    }
    
    if bytecode.len() > limits::MAX_PREDICATE_BYTECODE {
        return Err(format!(
            "Bytecode too large: {} bytes (max {})",
            bytecode.len(),
            limits::MAX_PREDICATE_BYTECODE
        ));
    }
    
    // Check that bytecode ends with RET
    if bytecode.last() != Some(&(Opcode::Ret as u8)) {
        return Err("Bytecode must end with RET".into());
    }
    
    // Validate instruction stream
    let mut pc = 0;
    while pc < bytecode.len() {
        let op = bytecode[pc];
        
        if Opcode::from_byte(op).is_none() {
            return Err(format!("Invalid opcode 0x{:02x} at offset {}", op, pc));
        }
        
        let opcode = Opcode::from_byte(op).unwrap();
        pc += 1;
        
        // Check operand bounds
        let operand_size = match opcode {
            Opcode::PushInt | Opcode::PushStr | Opcode::LoadVar | Opcode::LoadField | Opcode::Matches => 4,
            Opcode::CallPred => 2,
            _ => 0,
        };
        
        if pc + operand_size > bytecode.len() {
            return Err(format!(
                "Truncated operand for {:?} at offset {}",
                opcode,
                pc - 1
            ));
        }
        
        pc += operand_size;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compile_always() {
        let mut strings = StringPool::new();
        let mut compiler = PredicateCompiler::new(&mut strings);
        let bytecode = compiler.compile(&PredicateExpr::Always).unwrap();
        
        assert!(!bytecode.is_empty());
        assert_eq!(bytecode[0], Opcode::PushInt as u8);
        assert_eq!(*bytecode.last().unwrap(), Opcode::Ret as u8);
        
        // Validate
        assert!(validate_bytecode(&bytecode).is_ok());
    }
    
    #[test]
    fn test_compile_fail() {
        let mut strings = StringPool::new();
        let mut compiler = PredicateCompiler::new(&mut strings);
        let bytecode = compiler.compile(&PredicateExpr::Fail).unwrap();
        
        // PUSH_INT 0, RET
        assert_eq!(bytecode[0], Opcode::PushInt as u8);
        let val = i32::from_le_bytes([bytecode[1], bytecode[2], bytecode[3], bytecode[4]]);
        assert_eq!(val, 0);
    }
    
    #[test]
    fn test_compile_eq() {
        let mut strings = StringPool::new();
        let mut compiler = PredicateCompiler::new(&mut strings);
        let bytecode = compiler.compile(&PredicateExpr::Eq {
            left: "$token.sub".into(),
            right: Value::Str("admin".into()),
        }).unwrap();
        
        // LOAD_VAR "token.sub", PUSH_STR "admin", EQ, RET
        assert!(bytecode.len() > 5);
        assert!(validate_bytecode(&bytecode).is_ok());
    }
    
    #[test]
    fn test_compile_and() {
        let mut strings = StringPool::new();
        let mut compiler = PredicateCompiler::new(&mut strings);
        let bytecode = compiler.compile(&PredicateExpr::And {
            conditions: vec![
                PredicateExpr::Always,
                PredicateExpr::Always,
            ],
        }).unwrap();
        
        // Two conditions with AND between them
        assert!(bytecode.contains(&(Opcode::And as u8)));
        assert!(validate_bytecode(&bytecode).is_ok());
    }
    
    #[test]
    fn test_compile_not() {
        let mut strings = StringPool::new();
        let mut compiler = PredicateCompiler::new(&mut strings);
        let bytecode = compiler.compile(&PredicateExpr::Not {
            condition: Box::new(PredicateExpr::Fail),
        }).unwrap();
        
        assert!(bytecode.contains(&(Opcode::Not as u8)));
        assert!(validate_bytecode(&bytecode).is_ok());
    }
    
    #[test]
    fn test_disassemble() {
        let mut strings = StringPool::new();
        let mut compiler = PredicateCompiler::new(&mut strings);
        let bytecode = compiler.compile(&PredicateExpr::Eq {
            left: "$token.role".into(),
            right: Value::Str("admin".into()),
        }).unwrap();
        
        let disasm = disassemble(&bytecode, &strings);
        assert!(disasm.contains("LoadVar"));
        assert!(disasm.contains("token.role"));
        assert!(disasm.contains("PushStr"));
        assert!(disasm.contains("admin"));
        assert!(disasm.contains("Eq"));
        assert!(disasm.contains("Ret"));
    }
    
    #[test]
    fn test_bytecode_too_large() {
        let mut strings = StringPool::new();
        let mut compiler = PredicateCompiler::new(&mut strings);
        
        // Create deeply nested AND that will exceed limit
        let mut expr = PredicateExpr::Always;
        for _ in 0..100 {
            expr = PredicateExpr::And {
                conditions: vec![expr, PredicateExpr::Always],
            };
        }
        
        let result = compiler.compile(&expr);
        assert!(result.is_err());
    }
}