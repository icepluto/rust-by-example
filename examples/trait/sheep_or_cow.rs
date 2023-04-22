struct Sheep{}
struct Cow{}
//动物有很多特征，但是只写了会叫这个特征
trait Animal{
    //所有的动物都会叫
    fn noise(&self)->&'static str;
}
//sheep是动物 让它实现动物会叫的特征
impl Animal for Sheep{
    //绵羊怎么叫啊，咩咩咩
    fn noise(&self)->&'static str {
        "咩咩咩"
    }
}
//cow也是动物，也实现动物会叫的特征
impl Animal for Cow {
    //奶牛哞哞哞
    fn noise(&self)->&'static str {
        "哞哞哞"
    }
}
//这是随机的抽一个动物来叫。
//这个函数返回的未知的类型，不知道是牛还是羊，所以需要动态的将animal类型写到box里
//box是引用堆上的数据，所以是静态已知大小的
fn random_animal(random_number:f32)->Box<dyn Animal>{
    //抽的数字大于0.5就让羊叫，不然就让牛叫
    if random_number>0.5{
        Box::new(Sheep{})
    }else{
        Box::new(Cow{})
    }
}
//主函数
fn main(){
    //随机的方法没有写，就随便给个0-1的数字作为随机值
    let random = 0.56;
    //把随机数传入随机动物函数，执行随机动物的函数并赋值给random_animal
    //这里肯定是羊，因为0.56>0.5
    let random_animal = random_animal(random);
    //使用打印行宏输出random_animal里的noise特征,也就是第12行的代码
    println!("{}",random_animal.noise());
}