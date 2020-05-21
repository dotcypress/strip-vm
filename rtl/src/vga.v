module VGA800x480
(
    input clk,
    input rst,
    output vsync,
    output hsync,
    output de,
    output [9:0] x,
    output [9:0] y
);

    localparam SCREEN_WIDTH  = 16'd800;
    localparam SCREEN_HEIGHT = 16'd480;

    localparam V_SYNC       = 16'd5;
    localparam V_FRONTPORCH = 16'd62;
    localparam V_BACKPORCH  = 16'd6;

    localparam H_SYNC       = 16'd1;
    localparam H_FRONTPORCH = 16'd210;
    localparam H_BACKPORCH  = 16'd182;

    localparam FRAME_WIDTH  = H_BACKPORCH + H_FRONTPORCH + SCREEN_WIDTH;
    localparam FRAME_HEIGHT = V_BACKPORCH + V_FRONTPORCH + SCREEN_HEIGHT;

    reg [15:0] x_pos;
    reg [15:0] y_pos;

    always @(posedge clk or posedge rst) begin
        if (rst) begin
            y_pos <= 0;
            x_pos <= 0;
        end
        else if (x_pos == FRAME_WIDTH) begin
            x_pos <= 0;
            y_pos <= y_pos + 1;
        end
        else if (y_pos == FRAME_HEIGHT) begin
            y_pos <= 0;
            x_pos <= 0;
        end
        else begin
            x_pos <= x_pos + 1;
        end
    end

    assign vsync = ((y_pos >= V_SYNC ) && (y_pos <= FRAME_HEIGHT)) ? 0 : 1;
    assign hsync = ((x_pos >= H_SYNC) && (x_pos <= (FRAME_WIDTH - H_FRONTPORCH))) ? 0 : 1;

    assign de = ((x_pos > H_BACKPORCH) &&
                (x_pos <= (FRAME_WIDTH - H_FRONTPORCH)) &&
                (y_pos >= V_BACKPORCH) &&
                (y_pos <= FRAME_HEIGHT - V_FRONTPORCH - 1)) ? 1 : 0;

    assign x = x_pos - H_BACKPORCH;
    assign y = y_pos - V_BACKPORCH;
endmodule