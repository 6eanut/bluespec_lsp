// Test fixture for constant expansion feature
// This file contains various #define patterns for testing

// Simple numeric constants
#define 2 FOO;
#define 10 BAR;
#define 32 ADDR_WIDTH;
#define 8 DATA_WIDTH;

// Type function constants
#define TAdd#(FOO, 1) INCREMENTED;
#define TSub#(BAR, 3) OFFSET;
#define TMul#(DATA_WIDTH, 4) BUS_WIDTH;
#define TDiv#(BUS_WIDTH, 8) BUS_BYTES;

// Nested type functions
#define TAdd#(ADDR_WIDTH, DATA_WIDTH) TOTAL_WIDTH;
#define TMul#(TOTAL_WIDTH, 2) DOUBLE_WIDTH;

// Log and Exp functions
#define 256 PAGE_SIZE;
#define TLog#(PAGE_SIZE) PAGE_BITS;
#define TExp#(DATA_WIDTH) MAX_VALUES;

// Max and Min functions
#define 5 MIN_VAL;
#define 100 MAX_VAL;
#define TMax#(MIN_VAL, MAX_VAL) ACTUAL_MAX;
#define TMin#(MIN_VAL, MAX_VAL) ACTUAL_MIN;

// Complex nested expansion
#define 8 BASE_WIDTH;
#define TAdd#(BASE_WIDTH, 1) WIDTH_WITH_PARITY;
#define TMul#(WIDTH_WITH_PARITY, 4) TOTAL_BUS_WIDTH;
#define TDiv#(TOTAL_BUS_WIDTH, 8) TOTAL_BUS_BYTES;

// Module using these constants
module mkTest#(
    Bit#(ADDR_WIDTH) addr,
    Bit#(DATA_WIDTH) data
)();
    // Implementation
    Reg#(Bit#(TOTAL_WIDTH)) totalReg <- mkReg(0);
    
    rule process;
        totalReg <= {addr, data};
    endrule
    
    method Bit#(TOTAL_WIDTH) getTotal();
        return totalReg;
    endmethod
endmodule

// Function using constants
function Bit#(BUS_WIDTH) expandData(Bit#(DATA_WIDTH) data);
    return {data, data, data, data};
endfunction
