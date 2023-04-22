//泛型是泛化功能和函数功能。以扩大其适用范围。极大的减少代码重复。

//具体类型A。
struct A;
// 具体类型S。
struct S(A);
// 泛型类型。
struct Gen<T>(T);

fn reg_fn(_s: S) {}

fn gen_fn(_t: Gen<A>) {}
fn gen_s_fn(_t: Gen<i32>) {}

fn gen_spec_fn<T>(_t: Gen<T>) {}

fn main() {
    println!("generic");
    reg_fn(S(A));
    gen_fn(Gen(A));
    gen_s_fn(Gen(1));
    gen_spec_fn::<String>(Gen("hello generic".to_string()));
    gen_spec_fn(Gen("hello generic".to_string()));
}
