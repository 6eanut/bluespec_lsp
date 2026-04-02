// Test fixture: BSV code with syntax errors
// Used for testing error-tolerant symbol extraction
// The second module has a misspelled 'endmodule' -> 'endm'

module mkTest();
    // test logic
    Reg#(Bit#(32)) counter <- mkReg(0);
endmodule

module mkMain();
    mkTest my_test_inst;
    Reg#(Bit#(8)) value <- mkReg(0);
endm  // <-- This is intentionally misspelled (should be 'endmodule')

function Bit#(32) add(Bit#(32) a, Bit#(32) b);
    return a + b;
endfunction
