// Constant Expansion Module for BSV Language Server
// 
// This module implements automatic constant expansion for #define macros
// and type functions (TAdd, TSub, TMul, TDiv, TLog, TExp, etc.)
//
// Example:
//   #define 2 FOO;
//   #define TAdd#(FOO, 1) BAR;
//
// When hovering over BAR, shows:
//   BAR = TAdd#(FOO, 1)
//   └─ FOO = 2
//   └─ Result: 3

mod evaluator;
mod parser;
mod types;

pub use types::{ConstantDef, ExpansionResult, ExpansionStep};
pub use evaluator::ConstantEvaluator;
pub use parser::ConstantParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_numeric_constant() {
        let source = "#define 2 FOO;";
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].name, "FOO");
        assert_eq!(defs[0].value, "2");
    }

    #[test]
    fn test_tadd_constant() {
        let source = r#"
#define 2 FOO;
#define TAdd#(FOO, 1) BAR;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        assert_eq!(defs.len(), 2);
        assert_eq!(defs[0].name, "FOO");
        assert_eq!(defs[1].name, "BAR");
        assert_eq!(defs[1].value, "TAdd#(FOO, 1)");
    }

    #[test]
    fn test_expand_simple_constant() {
        let source = "#define 2 FOO;";
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("FOO");
        
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.final_value, 2);
        assert!(result.steps.len() >= 1);
        assert_eq!(result.steps[0].expression, "2");
    }

    #[test]
    fn test_expand_tadd_constant() {
        let source = r#"
#define 2 FOO;
#define TAdd#(FOO, 1) BAR;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("BAR");
        
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.final_value, 3);
        assert!(result.steps.len() >= 2);
    }

    #[test]
    fn test_nested_type_functions() {
        let source = r#"
#define 4 WIDTH;
#define TAdd#(WIDTH, 4) EXTENDED;
#define TMul#(EXTENDED, 2) TOTAL;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        
        assert_eq!(evaluator.expand("WIDTH").unwrap().final_value, 4);
        assert_eq!(evaluator.expand("EXTENDED").unwrap().final_value, 8);
        assert_eq!(evaluator.expand("TOTAL").unwrap().final_value, 16);
    }

    #[test]
    fn test_tsub_function() {
        let source = r#"
#define 10 SIZE;
#define TSub#(SIZE, 3) OFFSET;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("OFFSET").unwrap();
        
        assert_eq!(result.final_value, 7);
    }

    #[test]
    fn test_tmul_function() {
        let source = r#"
#define 5 BASE;
#define TMul#(BASE, 4) RESULT;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("RESULT").unwrap();
        
        assert_eq!(result.final_value, 20);
    }

    #[test]
    fn test_tdiv_function() {
        let source = r#"
#define 20 TOTAL;
#define TDiv#(TOTAL, 4) CHUNK;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("CHUNK").unwrap();
        
        assert_eq!(result.final_value, 5);
    }

    #[test]
    fn test_tlog_function() {
        let source = r#"
#define 256 VALUE;
#define TLog#(VALUE) LOG_VAL;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("LOG_VAL").unwrap();
        
        assert_eq!(result.final_value, 8);
    }

    #[test]
    fn test_texp_function() {
        let source = r#"
#define 3 EXPONENT;
#define TExp#(EXPONENT) POWER;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("POWER").unwrap();
        
        assert_eq!(result.final_value, 8);
    }

    #[test]
    fn test_complex_nested_expansion() {
        let source = r#"
#define 8 DATA_WIDTH;
#define TAdd#(DATA_WIDTH, 1) WIDTH_WITH_PARITY;
#define TMul#(WIDTH_WITH_PARITY, 4) BUS_WIDTH;
#define TDiv#(BUS_WIDTH, 8) BUS_BYTES;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        
        assert_eq!(evaluator.expand("DATA_WIDTH").unwrap().final_value, 8);
        assert_eq!(evaluator.expand("WIDTH_WITH_PARITY").unwrap().final_value, 9);
        assert_eq!(evaluator.expand("BUS_WIDTH").unwrap().final_value, 36);
        assert_eq!(evaluator.expand("BUS_BYTES").unwrap().final_value, 4);
    }

    #[test]
    fn test_format_expansion_trace() {
        let source = r#"
#define 2 FOO;
#define TAdd#(FOO, 1) BAR;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("BAR").unwrap();
        
        let trace = result.format_trace();
        
        assert!(trace.contains("BAR"));
        assert!(trace.contains("3"));
    }

    #[test]
    fn test_undefined_constant() {
        let source = "#define 2 FOO;";
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("UNDEFINED");
        
        assert!(result.is_none());
    }

    #[test]
    fn test_multiple_defines_same_file() {
        let source = r#"
#define 32 ADDR_WIDTH;
#define 8 DATA_WIDTH;
#define TAdd#(ADDR_WIDTH, DATA_WIDTH) TOTAL_WIDTH;
#define TMul#(TOTAL_WIDTH, 2) DOUBLE_WIDTH;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        assert_eq!(defs.len(), 4);
        
        let evaluator = ConstantEvaluator::new(defs);
        
        assert_eq!(evaluator.expand("TOTAL_WIDTH").unwrap().final_value, 40);
        assert_eq!(evaluator.expand("DOUBLE_WIDTH").unwrap().final_value, 80);
    }

    #[test]
    fn test_tmax_function() {
        let source = r#"
#define 5 A;
#define 10 B;
#define TMax#(A, B) MAX_VAL;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("MAX_VAL").unwrap();
        
        assert_eq!(result.final_value, 10);
    }

    #[test]
    fn test_tmin_function() {
        let source = r#"
#define 5 A;
#define 10 B;
#define TMin#(A, B) MIN_VAL;
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        let evaluator = ConstantEvaluator::new(defs);
        let result = evaluator.expand("MIN_VAL").unwrap();
        
        assert_eq!(result.final_value, 5);
    }

    #[test]
    fn test_parse_with_other_bsv_code() {
        let source = r#"
// Some BSV code
module mkTest();
    // Module body
endmodule

#define 4 WIDTH;
#define TAdd#(WIDTH, 4) EXTENDED;

function Bit#(WIDTH) getData();
    return 0;
endfunction
"#;
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        assert_eq!(defs.len(), 2);
        assert_eq!(defs[0].name, "WIDTH");
        assert_eq!(defs[1].name, "EXTENDED");
    }

    #[test]
    fn test_hover_simulation() {
        let source = r#"
#define 8 DATA_WIDTH;
#define TMul#(DATA_WIDTH, 4) BUS_WIDTH;

function Bit#(BUS_WIDTH) getData();
    return 0;
endfunction
"#;
        
        let parser = ConstantParser::new();
        let defs = parser.parse(source);
        
        println!("Parsed {} constants:", defs.len());
        for def in &defs {
            println!("  - {} = {} at {:?}", def.name, def.value, def.range);
        }
        
        assert!(defs.iter().any(|d| d.name == "DATA_WIDTH"), "Should find DATA_WIDTH");
        assert!(defs.iter().any(|d| d.name == "BUS_WIDTH"), "Should find BUS_WIDTH");
        
        let evaluator = ConstantEvaluator::from_source(source);
        
        let result = evaluator.expand("BUS_WIDTH");
        assert!(result.is_some(), "Should expand BUS_WIDTH");
        
        let result = result.unwrap();
        println!("\nBUS_WIDTH expansion:");
        println!("  Final value: {}", result.final_value);
        println!("  Success: {}", result.success);
        println!("  Trace:\n{}", result.format_trace());
        
        assert_eq!(result.final_value, 32, "BUS_WIDTH should be 32");
        assert!(result.success, "Expansion should succeed");
    }

    #[test]
    fn test_complex_nested_user_example() {
        // User's example: TOTAL_BUS_BYTES
        let source = r#"
#define 8 BASE_WIDTH;
#define TAdd#(BASE_WIDTH, 1) WIDTH_WITH_PARITY;
#define TMul#(WIDTH_WITH_PARITY, 4) TOTAL_BUS_WIDTH;
#define TDiv#(TOTAL_BUS_WIDTH, 8) TOTAL_BUS_BYTES;
"#;
        
        let evaluator = ConstantEvaluator::from_source(source);
        
        let result = evaluator.expand("TOTAL_BUS_BYTES");
        assert!(result.is_some(), "Should expand TOTAL_BUS_BYTES");
        
        let result = result.unwrap();
        println!("\nTOTAL_BUS_BYTES expansion (user example):");
        println!("  Final value: {}", result.final_value);
        println!("  Trace:\n{}", result.format_trace());
        
        assert_eq!(result.final_value, 4, "TOTAL_BUS_BYTES should be 4");
        
        let trace = result.format_trace();
        assert!(trace.contains("TOTAL_BUS_BYTES"), "Should contain constant name");
        assert!(trace.contains("TDiv#"), "Should show TDiv");
        assert!(trace.contains("TMul#"), "Should show TMul");
        assert!(trace.contains("TAdd#"), "Should show TAdd");
        assert!(trace.contains("Result:"), "Should show Result");
    }
}
