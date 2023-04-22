fn main(){
    let immutable = 1;
    let mut mutable = 2;
    println!("{}",immutable);
    println!("{}",mutable);
    mutable = 3;
    println!("{}",mutable);

    //变量遮蔽

    let long_live_binding = 11;
    {
        let short_live_binding = 22;
        println!("inner short:{}",short_live_binding);
        let long_live_binding = 12;
        println!("inner long:{}",long_live_binding);
    }
    let short_live_binding = 23;
    println!("outer short:{}",short_live_binding);
    
    println!("outer long:{}",long_live_binding);

    let long_live_binding = 'a';

    println!("{}",long_live_binding);

    //冻结
    let mut _mutable_interger = 33;
    {
        let _mutable_interger = _mutable_interger;
        //下面报错
        // _mutable_interger = 34;
    }
    //这里没有被冻结
    _mutable_interger = 34;
}