module ALU
(
    input clk,
    input en,
    input [3:0] fn,
    input [31:0] src1,
    input [31:0] src2,
    output reg [31:0] res
);
    localparam FN_ADD  = 4'b0000;
    localparam FN_AND  = 4'b0001;
    localparam FN_OR   = 4'b0010;
    localparam FN_XOR  = 4'b0011;
    localparam FN_SLL  = 4'b0100;
    localparam FN_SRL  = 4'b0101;
    localparam FN_SRA  = 4'b0110;
    localparam FN_SUB  = 4'b0111;
    localparam FN_MUL  = 4'b1000;
    localparam FN_SLT  = 4'b1001;
    localparam FN_SLTU = 4'b1010;
    localparam FN_EQ   = 4'b1011;
    localparam FN_NEQ  = 4'b1100;
    localparam FN_GE   = 4'b1101;
    localparam FN_GEU  = 4'b1110;

    always @(posedge clk) begin
        if (en)
            case (fn)
                FN_ADD: res <= src1 + src2;
                FN_AND: res <= src1 & src2;
                FN_OR:  res <= src1 | src2;
                FN_XOR: res <= src1 ^ src2;
                FN_SLL: res <= src1 << src2;
                FN_SRL: res <= src1 >>> src2;
                FN_SRA: res <= src1 >> src2;
                FN_SUB: res <= src1 - src2;
                FN_MUL: res <= src1 * src2;
                FN_SLT: res <= src1 < src2;// TODO: fix
                FN_SLTU: res <= src1 < src2;// TODO: fix
                FN_EQ:  res <= src1 == src2;
                FN_NEQ: res <= src1 != src2;
                FN_GE: res <= src1 >= src2;// TODO: fix
                FN_GEU: res <= src1 >= src2;// TODO: fix
                default: res <= 0;
            endcase
    end
endmodule