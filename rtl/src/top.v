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

    reg [31:0] video[6:0];
    reg [31:0] debug[5:0];

    assign led_r = ~debug[0];
    assign led_g = ~debug[1];
    assign led_b = ~debug[2];

    assign video[0] = debug[0];
    assign video[1] = debug[1];
    assign video[2] = debug[2];
    assign video[3] = debug[3];
    assign video[4] = debug[4];
    assign video[5] = debug[5];

    wire dashboard = x < 768 & y < 144;
    wire grid = (x % 24 < 2) | (y % 24 < 2);

    wire hit = video[y / 24][31 - x / 24];
    wire byte_tail = ((31 - x / 24) % 8) == 0;

    assign lcd_r = dashboard & ~grid & hit ? 5'b11111 : 0;
    assign lcd_g = dashboard & ~grid & ~byte_tail ? 5'b1111 : 0;
    assign lcd_b = dashboard & ~grid ? 5'b1111 : 0;

    reg [31:0] cnt;
    wire cpu_clk = cnt[20];
    always @(posedge sys_clk) begin
        cnt <= cnt + 1;
    end

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
        .clk(cpu_clk),
        .rst(~btn_a),
        .debug(debug)
    );

endmodule