def main(i0:bool, i1:bool, i2:bool, i3:bool, i4:bool, i5:bool, i6:bool) -> (y:i4) {
    t29:i4 = lmuxrega_i4(t21, t0, t27, t7) @lut(??, ??);
    t7:bool = const[1];
    t21:bool = land_bool(t14, i6) @lut(??, ??);
    t0:i4 = const[0];
    t27:i4 = lmux_i4(t20, t6, t26) @lut(??, ??);
    t14:bool = leq_i4(t29, t6) @lut(??, ??);
    t20:bool = land_bool(t13, i5) @lut(??, ??);
    t6:i4 = const[6];
    t26:i4 = lmux_i4(t19, t5, t25) @lut(??, ??);
    t13:bool = leq_i4(t29, t5) @lut(??, ??);
    t19:bool = land_bool(t12, i4) @lut(??, ??);
    t5:i4 = const[5];
    t25:i4 = lmux_i4(t18, t4, t24) @lut(??, ??);
    t12:bool = leq_i4(t29, t4) @lut(??, ??);
    t18:bool = land_bool(t11, i3) @lut(??, ??);
    t4:i4 = const[4];
    t24:i4 = lmux_i4(t17, t3, t23) @lut(??, ??);
    t11:bool = leq_i4(t29, t3) @lut(??, ??);
    t17:bool = land_bool(t10, i2) @lut(??, ??);
    t3:i4 = const[3];
    t23:i4 = lmux_i4(t16, t2, t22) @lut(??, ??);
    t10:bool = leq_i4(t29, t2) @lut(??, ??);
    t16:bool = land_bool(t9, i1) @lut(??, ??);
    t2:i4 = const[2];
    t22:i4 = lmux_i4(t15, t1, t29) @lut(??, ??);
    t9:bool = leq_i4(t29, t1) @lut(??, ??);
    t15:bool = land_bool(t8, i0) @lut(??, ??);
    t1:i4 = const[1];
    t8:bool = leq_i4(t29, t0) @lut(??, ??);
    y:i4 = id(t29);
}