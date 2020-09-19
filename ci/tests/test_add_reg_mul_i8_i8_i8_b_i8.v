module test_add_reg_mul_i8_i8_i8_b_i8(
    input clock,
    input reset,
    output fail,
    output finish);

    reg [31:0] step;
    reg t_fail;
    reg t_finish;

    reg [7:0] a;
    reg [7:0] b;
    reg [7:0] c;
    reg en;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            step <= 0;
            a <= 8'd4;
            b <= 8'd2;
            c <= 8'd3;
            en <= 1'b1;
            t_fail <= 1'b0;
            t_finish <= 1'b0;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    a <= 8'd0;
                    b <= 8'd0;
                    c <= 8'd3;
                    en <= 1'b0;
                    if (y != 8'd3) begin
                        $display("[test_add_reg_mul_i8_i8_i8_b_i8] ~~FAIL~~ res:%d exp:3", $signed(y));
                        t_fail <= 1'b1;
                    end
                end
                1: begin
                    if (y != 8'd11) begin
                        $display("[test_add_reg_mul_i8_i8_i8_b_i8] ~~FAIL~~ res:%d exp:11", $signed(y));
                        t_fail <= 1'b1;
                    end
                    t_finish <= 1'b1;
                end
            endcase
        end
    end

    add_reg_mul_i8_i8_i8_b_i8 dut(.clock(clock), .reset(reset), .a(a), .b(b), .c(c), .en(en), .y(y));

    assign fail = t_fail;
    assign finish = t_finish;

endmodule
