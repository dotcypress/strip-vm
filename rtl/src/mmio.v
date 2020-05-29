module MMIO
#(
    parameter ROM_WIDTH = 8,
    parameter RAM_WIDTH = 8,
    parameter BOOT_IMG  = "boot.rom"
)
(
    input clk,
    input load_en,
    input store_en,
    input use_ram,
    input [15:0] addr,
    input [31:0] in,
    output reg [31:0] out
);
    reg [31:0] ram[RAM_WIDTH - 1:0];
    reg [31:0] rom[ROM_WIDTH - 1:0]; /* synthesis syn_romstyle = "block_rom" */;

    initial $readmemh({ "../../", BOOT_IMG }, rom);

    always @(posedge clk) 
        if (store_en & use_ram)
            ram[addr] <= in;
        else if (load_en) begin
            out <= use_ram ? ram[addr] : rom[addr];
        end 
endmodule