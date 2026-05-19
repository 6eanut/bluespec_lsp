package TestRefs;

import Vector::*;

module mkHello();
    Reg#(Bit#(32)) val <- mkReg(0);
    rule hello;
        val <= val + 1;
    endrule

    function Bit#(32) add(Bit#(32) a, Bit#(32) b);
        return a + b;
    endfunction

    let result = add(val, 5);
endmodule

module mkWorld();
    mkHello hello_inst;  // reference to mkHello
endmodule

endpackage
