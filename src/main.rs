use reticle::lang::ast::*;
// use reticle::passes::select::dag::DAG;
// use reticle::backend::ultrascale;

fn main() {
    let mut comp = Def::new("muladd");
    comp.add_input("a", "i8");
    comp.add_input("b", "i8");
    comp.add_input("c", "i8");
    comp.add_input("en", "bool");
    comp.add_output("z", "i8");
    comp.add_instr(Instr::new_with_args("x", "i8", "mul", "a", "b", "??"));
    comp.add_instr(Instr::new_with_args("y", "i8", "reg", "x", "en", "??"));
    comp.add_instr(Instr::new_with_args("z", "i8", "add", "y", "c", "??"));
    let mut prog = Prog::new();
    prog.add_def(comp);
    println!("Original program:\n{}", prog);
    // let mut dag = DAG::new();
    // dag.from_prog(&prog);
    // dag.select();
    // println!("After instruction selection:\n{}", dag.to_prog());
    // let next_goal = format!("z: i8 = dsp_add_reg_mul(a, b, c, en) @dsp(??, ??);");
    // println!(
    //     "\n\nNext goal is to produce the following asm:\n{}\n\n",
    //     next_goal
    // );
    // let td = ultrascale::target();
    // println!("patterns\n");
    // for p in td.patterns.iter() {
    //     println!("name:{} cost:{}", p.name, p.cost);
    //     for i in p.instr.iter() {
    //         println!("    instr:{}", i);
    //     }
    // }
}
