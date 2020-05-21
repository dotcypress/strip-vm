module top
(
    input xtal,
    input btn_a,
    input btn_b,
    output led_g,
    output led_b,
    output led_r,
    output lcd_clk,
    output lcd_hsync,
    output lcd_vsync,
    output lcd_de,
    output [5:0] lcd_g,
    output [4:0] lcd_b,
    output [4:0] lcd_r
);
    wire clk;
    wire sys_clk;
    wire [9:0] x;
    wire [9:0] y;

    reg [30:0] cnt;
    reg [31:0] video [16:0];
    reg [31:0] debug[5:0];

    wire slow_clk = cnt[30];

    assign led_r = ~debug[0];
    assign led_g = ~debug[1];
    assign led_b = ~debug[2];

    wire grid = (x % 16 < 2) | (y % 16 < 2);
    wire dashboard = x < 512 & y < 160;
    wire hit = video[y / 16][x / 16];

    assign lcd_g = 0;
    assign lcd_r = dashboard & ~grid & hit ? 5'b11111 : 0;
    assign lcd_b = dashboard & ~grid ? 5'b11111 : 0;

    RPLL pll_instance (
        .clkin(xtal),
        .clkoutd3(lcd_clk), // 33.33 MHz
        .clkoutd(sys_clk),  // 10 MHz
        .clkout(clk)        // 100 MHz
    );

    VGA800x480 display (
        .clk(lcd_clk),
        .rst(~btn_b),
        .hsync(lcd_hsync),
        .vsync(lcd_vsync),
        .de(lcd_de),
        .x(x),
        .y(y)
    );

    CPU cpu (
        .clk(slow_clk),
        .rst(~btn_a),
        .debug(debug)
    );

    assign video[0] = debug[0];
    assign video[1] = debug[1];
    assign video[2] = debug[2];
    assign video[3] = debug[3];
    assign video[4] = debug[4];
    assign video[9] = debug[5];

    always @(posedge sys_clk) begin
        cnt <= cnt + 1;
    end
endmodule