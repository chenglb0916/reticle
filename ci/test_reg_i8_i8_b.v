module test_reg_i8_i8_b();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    reg [7:0] a;
    reg en;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            a <= 8'd9;
            en <= 1'b1;
        end
        else begin
            case (step)
                0: begin
                    step <= 1;
                    a <= 8'd0;
                    en <= 1'b0;
                    if (y != 8'd3) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                1: begin
                    step <= 2;
                    a <= 8'd0;
                    en <= 1'b0;
                    if (y != 8'd9) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                end
                2: begin
                    if (y != 8'd9) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    reg_i8_i8_b dut(.clock(clock), .reset(reset), .a(a), .en(en), .y(y));

endmodule
