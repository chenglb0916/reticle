module test_lut_and_i8_i8_i8();

    reg clock = 1'b0;
    reg reset = 1'b0;

    always #500 clock = ~clock;

    initial begin
        reset = 1'b1;
        repeat(16)@(negedge clock);
        reset = 1'b0;
    end

    reg [31:0] step;

    reg [7:0] a, b;
    wire [7:0] y;

    always @(posedge clock) begin
        if (reset | glbl.GSR) begin
            step <= 0;
            a <= 8'd9;
            b <= 8'd15;
        end
        else begin
            case (step)
                0: begin
                    if (y != 8'd9) begin
                        $display("~~FAIL~~");
                        $finish;
                    end
                    $finish;
                end
            endcase
        end
    end

    lut_and_i8_i8_i8 dut(.clock(clock), .reset(reset), .a(a), .b(b), .y(y));

endmodule
