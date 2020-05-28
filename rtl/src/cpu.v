module CPU
(
    input clk,
    input rst,
    output reg [31:0] debug[5:0]
);
    localparam REG_RA    = 1'd1;

    localparam S_FETCH   = 2'b00;
    localparam S_DECODE  = 2'b01;
    localparam S_EXEC    = 2'b10;
    localparam S_WBACK   = 2'b11;

    localparam OP_ADD   = 7'b0000000;
    localparam OP_ADDI  = 7'b0000001;
    localparam OP_ECALL = 7'b0000010;
    localparam OP_LA    = 7'b0000100;
    localparam OP_LUI   = 7'b0000101;
    localparam OP_AND   = 7'b0001000;
    localparam OP_ANDI  = 7'b0001001;
    localparam OP_OR    = 7'b0010000;
    localparam OP_ORI   = 7'b0010001;
    localparam OP_XOR   = 7'b0011000;
    localparam OP_XORI  = 7'b0011001;
    localparam OP_SLL   = 7'b0100000;
    localparam OP_SLLI  = 7'b0100001;
    localparam OP_SRL   = 7'b0101000;
    localparam OP_SRLI  = 7'b0101001;
    localparam OP_SRA   = 7'b0110000;
    localparam OP_SUB   = 7'b0111000;
    localparam OP_MUL   = 7'b1000000;
    localparam OP_MULI  = 7'b1000001;
    localparam OP_SLT   = 7'b1001000;
    localparam OP_BLT   = 7'b1001010;
    localparam OP_SLTU  = 7'b1010000;
    localparam OP_SLTIU = 7'b1010001;
    localparam OP_BLTU  = 7'b1010010;
    localparam OP_BEQ   = 7'b1011010;
    localparam OP_BNE   = 7'b1100010;
    localparam OP_BGE   = 7'b1101010;
    localparam OP_BGEU  = 7'b1110010;
    localparam OP_SB    = 7'b1111000;
    localparam OP_SH    = 7'b1111001;
    localparam OP_SW    = 7'b1111010;
    localparam OP_LBU   = 7'b1111011;
    localparam OP_LB    = 7'b1111100;
    localparam OP_LH    = 7'b1111101;
    localparam OP_LW    = 7'b1111110;
    localparam OP_LHU   = 7'b1111111;

    reg [1:0]  stage;
    reg [31:0] reg_file [7:0];
    reg [31:0] alu_src1;
    reg [31:0] alu_src2;
    reg [31:0] alu_res;
    reg [31:0] ir;
    reg [31:0] pc;
    reg [31:0] mem_load;
    reg [31:0] mem_store;
    reg [15:0] mem_addr;
    reg        alu_en;
    reg        branch;
    reg        mem_load_en;
    reg        mem_store_en;
    reg        mem_use_ram;

    wire [6:0]  fn  = ir[6:0];
    wire [2:0]  rd  = ir[9:7];
    wire [2:0]  rs1 = ir[12:10];
    wire [2:0]  rs2 = ir[15:13];
    wire [15:0] imm = ir[31:16];

    ALU alu (
        .clk(clk),
        .en(alu_en),
        .fn(fn ),
        .src1(alu_src1),
        .src2(alu_src2),
        .res(alu_res)
    );

    MMIO mmio (
        .clk(clk),
        .use_ram(mem_use_ram),
        .load_en(mem_load_en),
        .store_en(mem_store_en),
        .addr(mem_addr),
        .in(mem_store),
        .out(mem_load)
    );

    always @(posedge clk or posedge rst) begin
        if (rst) begin
            stage        <= S_FETCH;
            ir           <= 0;
            pc           <= 0;
            branch       <= 0;
            alu_en       <= 0;
            alu_src1     <= 0;
            alu_src2     <= 0;
            mem_load_en  <= 0;
            mem_store_en <= 0;
            mem_use_ram  <= 0;
            mem_addr     <= 0;
            mem_store    <= 0;
            reg_file[0]  <= 0;
            reg_file[1]  <= 0;
            reg_file[2]  <= 0;
            reg_file[3]  <= 0;
            reg_file[4]  <= 0;
            reg_file[5]  <= 0;
            reg_file[6]  <= 0;
            reg_file[7]  <= 0;
        end
        else
            case (stage)
                S_FETCH: begin
                    stage <= S_DECODE;
                    mem_use_ram <= 0;
                    mem_load_en <= 1;
                end
                S_DECODE: begin
                    stage <= S_EXEC;
                    ir <= mem_load;
                    mem_load_en <= 0;
                end
                S_EXEC: begin
                    stage <= S_WBACK;
                    case (fn )
                        OP_ADD | OP_AND | OP_OR | OP_XOR  | OP_SLL |
                        OP_SRL | OP_SLT | OP_SRA | OP_SUB | OP_MUL: begin
                            alu_src1 <= reg_file[rs1];
                            alu_src2 <= reg_file[rs2];
                            alu_en <= 1;
                        end
                        OP_ADDI | OP_ANDI | OP_ORI | OP_XORI |
                        OP_SLLI | OP_SRLI | OP_SLTIU | OP_MULI: begin
                            alu_src1 <= reg_file[rs1];
                            alu_src2 <= imm;
                            alu_en <= 1;
                        end
                        OP_BLT | OP_BLTU | OP_BEQ | OP_BNE | OP_BGE | OP_BGEU: begin
                            alu_src1 <= reg_file[rs1];
                            alu_src2 <= reg_file[rs2];
                            alu_en <= 1;
                            branch <= 1;
                        end
                        OP_LA: begin
                            reg_file[rd] <= reg_file[rs2] + imm;
                        end
                        OP_LUI: begin
                            reg_file[rd][31:16] <= imm;
                        end
                        OP_SB: begin
                            // TODO: fix
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_store <= reg_file[rd];
                            mem_store_en <= 1;
                        end
                        OP_SH: begin
                            // TODO: fix
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_store <= reg_file[rd];
                            mem_store_en <= 1;
                        end
                        OP_SW: begin
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_store <= reg_file[rd];
                            mem_store_en <= 1;
                        end
                        OP_LB: begin
                            // TODO: fix
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_load_en <= 1;
                        end
                        OP_LBU: begin
                            // TODO: fix
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_load_en <= 1;
                        end
                        OP_LH: begin
                            // TODO: fix
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_load_en <= 1;
                        end
                        OP_LHU: begin
                            // TODO: fix
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_load_en <= 1;
                        end
                        OP_LW: begin
                            mem_addr <= reg_file[rs2] + imm;
                            mem_use_ram <= 1;
                            mem_load_en <= 1;
                        end
                        OP_ECALL: begin
                            // TODO: fix
                        end
                    endcase
                end
                S_WBACK: begin
                    stage <= S_FETCH;
                    if (branch) begin
                        branch <= 0;
                        if (alu_res > 0) begin
                            pc <= reg_file[rs2] + imm;
                            reg_file[REG_RA] <= pc + 1;
                        end
                        else
                            pc <= pc + 1;
                    end
                    else begin
                        pc <= pc + 1;
                        if (mem_store_en)
                            mem_store_en <= 0;
                        else if (mem_load_en & rd > 0) begin
                            mem_load_en <= 0;
                            reg_file[rd] <= mem_load;
                        end
                        else if (alu_en)  begin
                            alu_en <= 0;
                            if (rd > 0)
                                reg_file[rd] <= alu_res;
                        end
                    end
                end
            endcase
    end

    assign debug[0] = { pc[29:0], stage};
    assign debug[1] = ir;
    assign debug[2] = reg_file[1];
    assign debug[3] = reg_file[2];
    assign debug[4] = reg_file[3];
    assign debug[5] = mem_addr;
endmodule

