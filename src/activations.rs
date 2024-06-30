// 使用标准库中的f64类型中的常数E
use std::f64::consts::E;

// 定义激活函数结构体
#[derive(Clone)]
pub struct Activation<'a> {
    // 激活函数
    pub function: &'a dyn Fn(f64) -> f64,
    // 导数
    pub derivative: &'a dyn Fn(f64) -> f64,
}

// 定义恒等激活函数
pub const IDENTITY: Activation = Activation {
    function: &|x| x,
    derivative: &|_| 1.0,
};

// 定义Sigmoid激活函数
pub const SIGMOID: Activation = Activation {
    function: &|x| 1.0 / (1.0 + E.powf(-x)),
    derivative: &|x| x * (1.0 - x),
};

// 定义Tanh激活函数
pub const TANH: Activation = Activation {
    function: &|x| x.tanh(),
    derivative: &|x| 1.0 - (x.powi(2)),
};

// 定义ReLU激活函数
pub const RELU: Activation = Activation {
    function: &|x| x.max(0.0),
    derivative: &|x| if x > 0.0 { 1.0 } else { 0.0 },
};
