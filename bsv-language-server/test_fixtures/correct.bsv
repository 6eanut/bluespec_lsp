// Test fixture: syntactically correct BSV code
// Used for baseline testing of symbol extraction

package TestPackage;

export mkTest;
export mkMain;
export add;

module mkTest();
    // test logic
    Reg#(Bit#(32)) counter <- mkReg(0);
    
    rule increment;
        counter <= counter + 1;
    endrule
endmodule

module mkMain();
    mkTest my_test_inst;
endmodule

function Bit#(32) add(Bit#(32) a, Bit#(32) b);
    return a + b;
endfunction

endpackage
